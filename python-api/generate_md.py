import json
from datetime import datetime
from collections import defaultdict

# =========================
# FILE PATHS
# =========================
DATA_FILE = "../data/output.json"
OUTPUT_FILE = "../data/CHANGELOG.md"

# =========================
# LOAD JSON DATA
# =========================
def load_data():
    try:
        with open(DATA_FILE, "r") as f:
            return json.load(f)
    except Exception as e:
        print("Error loading JSON:", e)
        return []

# =========================
# EXTRACT DATE FROM TIMESTAMP
# =========================
def extract_date(timestamp):
    dt = datetime.fromisoformat(timestamp.replace("Z", ""))
    return dt.date()

# =========================
# CATEGORY DETECTION
# =========================
def categorize(summary):
    s = summary.lower()

    if "fix" in s or "bug" in s:
        return "Bug Fixes"
    elif "add" in s or "feature" in s:
        return "Features"
    elif "refactor" in s:
        return "Refactoring"
    else:
        return "Other"

# =========================
# GENERATE MARKDOWN
# =========================
def generate_markdown(clusters):
    grouped = defaultdict(list)

    # Group clusters by date
    for cluster in clusters:
        try:
            date = extract_date(cluster["changes"][0]["timestamp"])
            grouped[date].append(cluster)
        except Exception as e:
            print("Skipping cluster due to error:", e)

    md = "# Changelog\n\n"

    # Sort dates (latest first)
    for date in sorted(grouped.keys(), reverse=True):
        md += f"## {date}\n\n"

        # Categorize clusters
        sections = defaultdict(list)

        for cluster in grouped[date]:
            category = categorize(cluster["summary"])
            sections[category].append(cluster)

        # Write sections
        for category, items in sections.items():
            md += f"### {category}\n"

            for cluster in items:
                summary = cluster["summary"]

                files = [c["file"] for c in cluster["changes"]]
                file_list = ", ".join(files)

                md += f"- {summary} ({file_list})\n"

            md += "\n"

    return md

# =========================
# SAVE MARKDOWN FILE
# =========================
def save_markdown(content):
    try:
        with open(OUTPUT_FILE, "w") as f:
            f.write(content)
    except Exception as e:
        print("Error saving markdown:", e)

# =========================
# MAIN FUNCTION
# =========================
def main():
    clusters = load_data()

    if not clusters:
        print("No data found. Make sure Rust output.json is correct.")
        return

    markdown = generate_markdown(clusters)
    save_markdown(markdown)

    print("CHANGELOG.md generated successfully!")

# =========================
# RUN SCRIPT
# =========================
if __name__ == "__main__":
    main()