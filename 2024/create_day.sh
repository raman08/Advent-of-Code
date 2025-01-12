#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

day_number=$1
folder_name="src/day_$day_number"

# Create the folder
mkdir -p "$folder_name"

# Create the main.rs file
echo "use aoc_2024::{get_file_path, read_file};
use std::env;

fn part_1(lines: &str) -> i32 {
    0
}

fn part_2(lines: &str) -> i32 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path($day_number, is_sample);
    let contents = read_file(path);

    println!(\"Part 1: {}\", part_1(&contents));
    println!(\"Part 2: {}\", part_2(&contents));
}" >> "$folder_name/main.rs"

#creating input files
touch "$folder_name/sample.txt"
touch "$folder_name/input.txt"


# Update the Cargo.toml file
echo "" >> Cargo.toml
echo "[[bin]]" >> Cargo.toml
echo "name = \"day_$day_number\"" >> Cargo.toml
echo "path = \"$folder_name/main.rs\"" >> Cargo.toml

echo "Day $day_number setup complete!"
