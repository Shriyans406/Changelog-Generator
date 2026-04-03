#!/bin/bash

# Output file
OUTPUT="../data/changes.jsonl"

# Clear old file
> $OUTPUT

# Get git log (hash | timestamp | message)
git log --pretty=format:'%H|%ai|%s' | while IFS="|" read -r hash timestamp message
do
    # For each commit, get files changed
    git diff-tree --no-commit-id --name-status -r $hash | while read status file
    do
        # Convert git status to readable type
        if [ "$status" = "A" ]; then
            type="Added"
        elif [ "$status" = "M" ]; then
            type="Modified"
        elif [ "$status" = "D" ]; then
            type="Deleted"
        else
            type="Modified"
        fi

        # Print JSON line
        echo "{\"file\":\"$file\",\"timestamp\":\"$timestamp\",\"type\":\"$type\",\"description\":\"$message\",\"hash\":\"$hash\"}" >> $OUTPUT
    done
done

echo "Data collected in $OUTPUT"
