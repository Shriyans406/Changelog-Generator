use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


// =========================
// DEFINE STRUCT (MUST BE ABOVE main)
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
// MAIN FUNCTION
// =========================
fn main() {
    let file_path = "../data/changes.jsonl";

    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    // STEP 1: Create empty vector
    let mut changes: Vec<Change> = Vec::new();

    // STEP 2: Read and store data
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                match serde_json::from_str::<Change>(&line_content) {
                    Ok(change) => {
                        changes.push(change);
                    }
                    Err(e) => eprintln!("JSON error: {}", e),
                }
            }
            Err(e) => eprintln!("Read error: {}", e),
        }
    }
    changes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    for change in &changes {
    println!("{} - {}", change.file, change.timestamp);
}
// STEP 3: Print total count
    println!("Total changes loaded: {}", changes.len());
let mut unique_changes: HashMap<String, Change> = HashMap::new();

for change in changes {
    // Insert only if not already present
    unique_changes.entry(change.file.clone()).or_insert(change);
}
println!("\nAfter Deduplication:");

for (file, change) in &unique_changes {
    println!("{} -> {}", file, change.timestamp);
}

    
}
