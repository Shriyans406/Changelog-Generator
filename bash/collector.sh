#!/bin/bash

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Output file
OUTPUT="$PROJECT_ROOT/data/changes.jsonl"

# Create data directory if not exists
mkdir -p "$PROJECT_ROOT/data"

# Clear old file
> "$OUTPUT"

# Get git log
git log --pretty=format:'%H|%ai|%s' | while IFS="|" read -r hash timestamp message
do
    # Convert timestamp to ISO format
    timestamp=$(date -d "$timestamp" -u +"%Y-%m-%dT%H:%M:%SZ")

    # Get changed files
    git diff-tree --no-commit-id --name-status -r "$hash" | while read status file
    do
        # Map status
        if [ "$status" = "A" ]; then
            type="Added"
        elif [ "$status" = "M" ]; then
            type="Modified"
        elif [ "$status" = "D" ]; then
            type="Deleted"
        else
            type="Modified"
        fi

        # Output JSON line
        echo "{\"file\":\"$file\",\"timestamp\":\"$timestamp\",\"type\":\"$type\",\"description\":\"$message\",\"hash\":\"$hash\"}" >> "$OUTPUT"
    done
done

echo "Data collected in $OUTPUT"
