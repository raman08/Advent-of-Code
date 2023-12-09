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
echo "fn main() {" >> "$folder_name/main.rs"
echo '	println!("Hello from day_'"$day_number"'!");' >> "$folder_name/main.rs"
echo "}" >> "$folder_name/main.rs"

#creating input files
touch "$folder_name/sample.txt"
touch "$folder_name/input.txt"


# Update the Cargo.toml file
echo "" >> Cargo.toml 
echo "[[bin]]" >> Cargo.toml
echo "name = \"day_$day_number\"" >> Cargo.toml
echo "path = \"$folder_name/main.rs\"" >> Cargo.toml

echo "Day $day_number setup complete!"
