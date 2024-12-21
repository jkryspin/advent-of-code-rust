#!/bin/bash

# Directory containing the files
DIR="/Users/johnkryspin/Documents/projects/advent-of-code-rust/src/year2024/"

# Loop through all files in the directory
for FILE in "$DIR"/*; do
  # Get the base name of the file
  BASENAME=$(basename "$FILE")

  # Rename the file by appending "day" to the base name
  mv "$FILE" "$DIR/day${BASENAME}"
done