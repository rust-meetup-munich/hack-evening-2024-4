#!/usr/bin/env bash

# Check if at least one argument is provided
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 file1.json file2.json ... fileN.json"
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for required tools
for tool in jq sort awk sed; do
    if ! command_exists "$tool"; then
        echo "Error: $tool is not installed."
        exit 1
    fi
done

# Temporary file to store intermediate results
temp_file=$(mktemp)

# Process each JSON file
for json_file in "$@"; do
    # Extract the filename without the path
    filename=$(basename "$json_file" .json)

    # Use jq to extract the mean value and append to the temporary file
    jq -r --arg filename "$filename" '.results[] | [$filename, .mean, .stddev] | @csv' "$json_file" | sed 's/,/;/g' >>"$temp_file"
done

# Sort the temporary file by the second column (mean value) in descending order and save to final CSV
sort -t\; -k2,2n "$temp_file" >results.csv

# Remove the temporary file
rm "$temp_file"

echo "Results have been saved to results.csv"

LEADERBOARD=guide/src/20_leaderboard/SUMMARY.md
rm -f $LEADERBOARD
# Generate Markdown table from results.csv
{
    echo "# Leaderboard"
    echo ""
    echo "| Rank | User     | Mean [s] | Stddev [s] |"
    echo "|:------:|:----------|:----------|:---------|"
    awk -F';' '{
    rank = NR
    user = substr($1, 2, length($1) - 2) # Remove quotes
    mean = $2
    stddev = $3
    printf "| %d | [![%s](https://github.com/%s.png?size=128) %s](https://github.com/%s) | %s | ± %s |\n", rank, user, user, user, user, mean, stddev
  }' results.csv
} >$LEADERBOARD

# write a tiny timestamp to the page to indicate when it was last updated
echo "" >>$LEADERBOARD
echo "" >>$LEADERBOARD
echo "<sub>updated at: $(date --rfc-3339=seconds) </sub>" >>$LEADERBOARD
echo "Markdown table has been saved to: $LEADERBOARD"
