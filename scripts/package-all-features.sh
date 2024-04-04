#!/bin/bash

# Array of feature names
features=("cloneable-secret", "alloc", "zeroize", "debug-secret")

# Calculate the total number of features
total_features=${#features[@]}

# Export `ENV_VAR`
export ENV_VAR="69"

# Function to generate combinations of features
generate_combinations() {
    local index=$1
    local combination=$2

    if [ $index -eq $total_features ]; then
        # Run cargo package with the current combination of features
        echo "Running: cargo package --features $combination"
        cargo package --features "$combination" --allow-dirty && cargo package --features "$combination" --list --allow-dirty
    else
        # Include the current feature in the combination and recurse
        generate_combinations "$((index + 1))" "$combination ${features[index]}"
        # Exclude the current feature and recurse
        generate_combinations "$((index + 1))" "$combination"
    fi
}

# Start generating combinations from index 0
generate_combinations 0 ""

echo "All feature combinations packagee successfully."
