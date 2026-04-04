#!/bin/bash

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Output file
OUTPUT="$PROJECT_ROOT/data/changes.jsonl"

# Create data directory
mkdir -p "$PROJECT_ROOT/data"

# Get repo path from argument
REPO_PATH="$1"

# Check if argument provided
if [ -z "$REPO_PATH" ]; then
    echo "Error: Please provide a git repository path"
    echo "Usage: ./collector.sh /path/to/repo"
    exit 1
fi

# Check if directory exists
if [ ! -d "$REPO_PATH" ]; then
    echo "Error: Directory does not exist"
    exit 1
fi

# Check if it's a git repo
if [ ! -d "$REPO_PATH/.git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

# Move into repo
cd "$REPO_PATH" || exit

# Clear old output
> "$OUTPUT"

# Extract git data
git log --pretty=format:'%H|%ai|%s' | while IFS="|" read -r hash timestamp message
do
    # Convert timestamp
    timestamp=$(date -d "$timestamp" -u +"%Y-%m-%dT%H:%M:%SZ")

    git diff-tree --no-commit-id --name-status -r --root "$hash" | while read status file
    do
        if [ "$status" = "A" ]; then
            type="Added"
        elif [ "$status" = "M" ]; then
            type="Modified"
        elif [ "$status" = "D" ]; then
            type="Deleted"
        else
            type="Modified"
        fi

        echo "{\"file\":\"$file\",\"timestamp\":\"$timestamp\",\"type\":\"$type\",\"description\":\"$message\",\"hash\":\"$hash\"}" >> "$OUTPUT"
    done
done

echo "Data collected in $OUTPUT"
