use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// =========================
// DEFINE STRUCT
// =========================
#[derive(Debug, Deserialize)]
struct Change {
    file: String,
    timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    change_type: String,
    description: String,
    hash: String,
}

// =========================
// CLUSTER STRUCT (Phase 3C)
// =========================
#[derive(Debug)]
struct Cluster {
    changes: Vec<Change>,
}

// =========================
// MAIN FUNCTION
// =========================
fn main() {
    let file_path = "../data/changes.jsonl";

    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    // STEP 1: Store all changes
    let mut changes: Vec<Change> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                match serde_json::from_str::<Change>(&line_content) {
                    Ok(change) => changes.push(change),
                    Err(e) => eprintln!("JSON error: {}", e),
                }
            }
            Err(e) => eprintln!("Read error: {}", e),
        }
    }

    // STEP 2: Sort descending (latest first)
    changes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    println!("--- Raw Sorted Data ---");
    for change in &changes {
        println!("{} - {}", change.file, change.timestamp);
    }

    println!("\nTotal changes loaded: {}", changes.len());

    // STEP 3: Deduplication
    let mut unique_changes: HashMap<String, Change> = HashMap::new();

    for change in changes {
        unique_changes.entry(change.file.clone()).or_insert(change);
    }

    println!("\n--- After Deduplication ---");
    for (file, change) in &unique_changes {
        println!("{} -> {}", file, change.timestamp);
    }

    // =========================
    // PHASE 3C: TEMPORAL CLUSTERING
    // =========================

    // STEP 4: Convert to vector
    let mut deduped_changes: Vec<Change> = unique_changes
        .into_iter()
        .map(|(_, change)| change)
        .collect();

    // STEP 5: Sort ascending (oldest first)
    deduped_changes.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // STEP 6: Initialize clusters
    let mut clusters: Vec<Cluster> = Vec::new();
    let threshold = chrono::Duration::minutes(30);

    // STEP 7: Clustering logic
    for change in deduped_changes {
        if clusters.is_empty() {
            clusters.push(Cluster {
                changes: vec![change],
            });
            continue;
        }

        let last_cluster = clusters.last_mut().unwrap();
        let last_change = last_cluster.changes.last().unwrap();

        let time_diff = change.timestamp - last_change.timestamp;

        if time_diff <= threshold {
            last_cluster.changes.push(change);
        } else {
            clusters.push(Cluster {
                changes: vec![change],
            });
        }
    }

    // STEP 8: Print clusters
    println!("\n--- Clusters Formed ---");
    println!("Total clusters: {}", clusters.len());

    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCluster {}", i + 1);

        for change in &cluster.changes {
            println!("{} - {}", change.file, change.timestamp);
        }
    }
}