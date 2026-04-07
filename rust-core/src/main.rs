use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use strsim::jaro_winkler;

// =========================
// CHANGE STRUCT
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
// CLUSTER STRUCT (FINAL)
// =========================
#[derive(Debug)]
struct Cluster {
    changes: Vec<Change>,
    summary: String,
    confidence: f64,
}

// =========================
// MAIN FUNCTION
// =========================
fn main() {
    let file_path = "../data/changes.jsonl";

    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    // =========================
    // STEP 1: LOAD DATA
    // =========================
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

    // =========================
    // STEP 2: SORT (LATEST FIRST)
    // =========================
    changes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    println!("--- Raw Sorted Data ---");
    for change in &changes {
        println!("{} - {}", change.file, change.timestamp);
    }

    println!("\nTotal changes loaded: {}", changes.len());

    // =========================
    // STEP 3: DEDUPLICATION
    // =========================
    let mut unique_changes: HashMap<String, Change> = HashMap::new();

    for change in changes {
        unique_changes.entry(change.file.clone()).or_insert(change);
    }

    println!("\n--- After Deduplication ---");
    for (file, change) in &unique_changes {
        println!("{} -> {}", file, change.timestamp);
    }

    // =========================
    // STEP 4: CONVERT TO VECTOR
    // =========================
    let mut deduped_changes: Vec<Change> = unique_changes
        .into_iter()
        .map(|(_, change)| change)
        .collect();

    // =========================
    // STEP 5: SORT (OLDEST FIRST)
    // =========================
    deduped_changes.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // =========================
    // STEP 6: CLUSTERING
    // =========================
    let mut clusters: Vec<Cluster> = Vec::new();
    let threshold = chrono::Duration::minutes(30);

    for change in deduped_changes {
        if clusters.is_empty() {
            clusters.push(Cluster {
                summary: change.description.clone(),
                changes: vec![change],
                confidence: 1.0,
            });
            continue;
        }

        let last_cluster = clusters.last_mut().unwrap();
        let last_change = last_cluster.changes.last().unwrap();

        let time_diff = change.timestamp - last_change.timestamp;

        if time_diff <= threshold {
            let similarity = jaro_winkler(
                &last_cluster.summary,
                &change.description
            );

            if similarity >= 0.80 {
                // Improve summary (pick better description)
                if change.description.len() > last_cluster.summary.len() {
                    last_cluster.summary = change.description.clone();
                }

                last_cluster.changes.push(change);

                // Update confidence
                last_cluster.confidence =
                    (last_cluster.confidence + similarity) / 2.0;

            } else {
                clusters.push(Cluster {
                    summary: change.description.clone(),
                    changes: vec![change],
                    confidence: 1.0,
                });
            }

        } else {
            clusters.push(Cluster {
                summary: change.description.clone(),
                changes: vec![change],
                confidence: 1.0,
            });
        }
    }

    // =========================
    // STEP 7: OUTPUT
    // =========================
    println!("\n--- Clusters Formed ---");
    println!("Total clusters: {}", clusters.len());

    for (i, cluster) in clusters.iter().enumerate() {
        println!("\n============================");
        println!("Cluster {}", i + 1);
        println!("Summary: {}", cluster.summary);
        println!("Confidence: {:.2}", cluster.confidence);
        println!("Total Changes: {}", cluster.changes.len());
        println!("Files:");

        for change in &cluster.changes {
            println!(" - {}", change.file);
        }
    }
}