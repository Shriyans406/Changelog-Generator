#!/bin/bash

echo "======================================"
echo "🚀 Starting Changelog Generation"
echo "======================================"

# -----------------------------
# STEP 1: Run Bash Collector
# -----------------------------
echo "📦 Collecting Git data..."

REPO_PATH=$1

if [ -z "$REPO_PATH" ]; then
    echo "❌ Please provide repo path"
    echo "Usage: ./generate_changelog.sh <repo_path>"
    exit 1
fi

bash bash/collector.sh "$REPO_PATH"

if [ $? -ne 0 ]; then
    echo "❌ Collector failed"
    exit 1
fi

# -----------------------------
# STEP 2: Run Rust Core
# -----------------------------
echo "⚙️ Processing data with Rust..."

cd rust-core
cargo run

if [ $? -ne 0 ]; then
    echo "❌ Rust processing failed"
    exit 1
fi

cd ..

# -----------------------------
# STEP 3: Run Python Generator
# -----------------------------
echo "🧠 Generating Markdown with AI..."

cd python-api

# Activate virtual environment
source venv/bin/activate

python generate_md.py

if [ $? -ne 0 ]; then
    echo "❌ Python generation failed"
    exit 1
fi

cd ..

# -----------------------------
# DONE
# -----------------------------
echo "======================================"
echo "✅ CHANGELOG GENERATED SUCCESSFULLY!"
echo "📄 Check: data/CHANGELOG.md"
echo "======================================"