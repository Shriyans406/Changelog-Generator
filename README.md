```md
# Changelog Generator

## Overview

Changelog Generator is a multi-language developer tool that automatically generates structured and human-readable changelogs from a Git repository. It analyzes commit history, groups related changes, and produces a clean markdown document that summarizes development activity over time.

The system combines Bash, Rust, and Python to create a pipeline that transforms raw Git data into meaningful documentation. It also supports AI-based summarization using external models such as Gemini to improve the readability of change descriptions.

---

## Objective

The primary objective of this project is to eliminate the manual effort required to write changelogs. Developers often neglect changelog maintenance because it is repetitive and time-consuming. This tool automates the entire process by:

- Extracting commit-level data from a repository
- Structuring and deduplicating changes
- Grouping related modifications
- Generating readable summaries
- Producing a final markdown changelog

The result is a consistent and maintainable changelog without manual intervention.

---

## How It Works

The system operates as a pipeline with three main stages:

### 1. Data Collection (Bash)

A shell script extracts raw data from a Git repository using commands such as:

- git log for commit history
- git diff-tree for file-level changes
- find for recently modified files

This data is formatted into JSON Lines format and stored in:

```

data/changes.jsonl

```

Each entry represents a single change with attributes such as file name, timestamp, type, description, and commit hash.

---

### 2. Processing and Clustering (Rust)

The Rust core performs structured processing of the collected data:

- Parses JSON input into strongly typed structures
- Sorts changes based on timestamps
- Removes duplicate entries using file-based deduplication
- Groups related changes using:
  - String similarity (Jaro-Winkler)
  - Time-based clustering

The output is a structured JSON file:

```

data/output.json

```

Each cluster represents a logical group of changes with a summary and associated files.

---

### 3. Changelog Generation (Python)

The Python layer converts structured data into a human-readable changelog:

- Reads clustered JSON output
- Groups changes by date
- Categorizes them into sections such as Features, Bug Fixes, and Refactoring
- Optionally enhances summaries using an AI model (Gemini)
- Generates a markdown file:

```

data/CHANGELOG.md

```

---

### 4. Automation and CLI Execution

The entire pipeline is orchestrated using:

- A shell script for sequential execution
- A Rust-based CLI tool for user interaction

The CLI allows users to generate changelogs with a single command by providing the path to a Git repository.

---

## Features

- Automatic changelog generation from Git history
- Multi-language pipeline combining Bash, Rust, and Python
- Deduplication of redundant changes
- Intelligent clustering of related commits
- AI-powered summarization using Gemini
- Categorized markdown output
- Command-line interface for ease of use
- Extensible architecture for future enhancements

---

## Project Structure

```

changelog-generator/
│
├── bash/
│   └── collector.sh
│
├── rust-core/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
├── python-api/
│   ├── generate_md.py
│   └── venv/
│
├── data/
│   ├── changes.jsonl
│   ├── output.json
│   └── CHANGELOG.md
│
├── generate_changelog.sh
│
└── README.md

```

---

## Installation

### Prerequisites

- Linux environment
- Git installed
- Rust toolchain installed
- Python 3 installed

---

### Step 1: Clone the repository

```

git clone <repository_url>
cd changelog-generator

```

---

### Step 2: Setup Python environment

```

cd python-api
python3 -m venv venv
source venv/bin/activate
pip install google-generativeai

```

---

### Step 3: Configure Gemini API (Optional)

Edit generate_md.py and set your API key:

```

genai.configure(api_key="YOUR_GEMINI_API_KEY")

```

---

### Step 4: Build Rust project

```

cd ../rust-core
cargo build --release

```

---

### Step 5: Make scripts executable

```

chmod +x ../bash/collector.sh
chmod +x ../generate_changelog.sh

```

---

## Usage

### Using Shell Script

```

./generate_changelog.sh <path_to_repo>

```

Example:

```

./generate_changelog.sh ./test-repo

```

---

### Using CLI Tool

```

changelog-gen <path_to_repo>

```

Example:

```

changelog-gen ~/projects/test-repo

```

---

## Output

The final changelog is generated at:

```

data/CHANGELOG.md

```

Example structure:

```

# Changelog

## 2026-04-04

### Features

* Added new files for improved functionality

### Other

* Initial project setup

```

---

## Behavior and Flow

When executed, the system performs the following sequence:

1. Collects Git data from the target repository
2. Converts raw data into structured JSON
3. Processes and clusters changes using Rust
4. Enhances summaries using AI or fallback logic
5. Generates a categorized markdown changelog

Each stage is modular and can be modified independently.

---

## Design Principles

- Separation of concerns across languages
- Deterministic core with optional AI enhancement
- Fail-safe behavior with fallback mechanisms
- Extensibility for integration with external systems
- Performance through Rust-based processing

---

## Limitations

- Requires a valid Git repository
- AI summarization depends on API availability
- Initial setup requires multiple dependencies
- Clustering logic may need tuning for large repositories

---

## Future Improvements

- GitHub and GitLab integration
- Automatic release creation
- Web dashboard for changelog visualization
- Configuration file support
- Advanced NLP-based categorization
- Scheduling and CI/CD integration

---

## Conclusion

This project demonstrates a complete pipeline that combines systems programming, scripting, and AI integration to solve a real developer problem. It reflects practical engineering skills across multiple domains, including data processing, automation, and intelligent summarization.

The tool is designed to be both functional and extensible, making it suitable for personal use as well as further development into a production-grade system.
```
