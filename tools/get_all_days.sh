#!/bin/bash

# Define the base URL
BASE_URL="https://adventofcode.com"
COOKIE="session=<REDACTED>"

# Loop through the years
for YEAR in {2015..2024}; do
  # Loop through the days
  for DAY in {1..25}; do
    # Format the day to always be two digits
    DAY_FORMATTED=$(printf "%02d" $DAY)

    # Define the URL and output file path
    URL="$BASE_URL/$YEAR/day/$DAY/input"
    OUTPUT_DIR="./input/year$YEAR"
    OUTPUT_FILE="$OUTPUT_DIR/day$DAY_FORMATTED.txt"

    # Create the output directory if it doesn't exist
    mkdir -p "$OUTPUT_DIR"

    # Send the GET request with the cookie and save the result
    curl -s -b "$COOKIE" -o "$OUTPUT_FILE" "$URL"

    # Sleep for 1 second between requests
    sleep 1
  done
done