#!/bin/bash

# Array of feature names
features=("cloneable-secret", "alloc")

# Calculate the total number of features
total_features=${#features[@]}

# Function to generate combinations of features
generate_combinations() {
    local index=$1
    local combination=$2

    if [ $index -eq $total_features ]; then
        # Run cargo clippy with the current combination of features
        echo "Running: cargo clippy --features $combination"
        cargo clippy --features "$combination"
    else
        # Include the current feature in the combination and recurse
        generate_combinations "$((index + 1))" "$combination ${features[index]}"
        # Exclude the current feature and recurse
        generate_combinations "$((index + 1))" "$combination"
    fi
}

# Start generating combinations from index 0
generate_combinations 0 ""

echo "All feature combinations checked with clippy successfully."
