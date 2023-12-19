use aoc2023::{get_file_path, read_file};
use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, i32, str::Lines};

fn part_1(mut lines: Lines) -> i32 {
    let instructions = lines.next().unwrap();

    let instructions: Vec<_> = instructions.chars().collect();

    let map = lines.fold(HashMap::new(), |mut acc, line| {
        if line.len() == 0 {
            return acc;
        }

        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let a = re.captures(line).unwrap();

        let key = a.get(1).unwrap().as_str();
        let left = a.get(2).unwrap().as_str();
        let right = a.get(3).unwrap().as_str();

        acc.insert(key, (left, right));
        acc
    });

    let mut start = "AAA";
    let mut steps = 0;

    let mut i = 0;

    while start != "ZZZ" {
        let next = map.get(start).unwrap();

        i = i % instructions.len();

        let lr = instructions[i];

        start = match lr {
            'L' => next.0,
            'R' => next.1,
            _ => unreachable!(),
        };

        i += 1;

        steps += 1;
    }

    steps
}

fn part_2(lines: Lines) -> i128 {
    let mut lines = lines.clone();

    let instructions = lines.next().unwrap();

    let instructions: Vec<_> = instructions.chars().collect();

    let map = lines.fold(HashMap::new(), |mut acc, line| {
        if line.len() == 0 {
            return acc;
        }

        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let a = re.captures(line).unwrap();

        let key = a.get(1).unwrap().as_str();
        let left = a.get(2).unwrap().as_str();
        let right = a.get(3).unwrap().as_str();

        acc.insert(key, (left, right));
        acc
    });

    let starts: Vec<_> = map
        .keys()
        .filter(|&key| key.ends_with("A"))
        .cloned()
        .collect();

    let mut counts: HashMap<String, i128> = HashMap::new();

    for start in starts.into_iter() {
        let mut start_clone = start;

        let mut count = 0;
        let mut instructions_iter = instructions.iter().cycle();

        while !start_clone.ends_with("Z") {
            let current_instruction = instructions_iter.next().unwrap();
            let next = map.get(start_clone).unwrap();

            start_clone = match current_instruction {
                'L' => next.0,
                'R' => next.1,
                _ => unreachable!(),
            };

            count += 1;
        }

        counts.insert(start.to_string(), count);
    }

    let counts = counts.values().cloned().collect::<Vec<_>>();

    counts.into_iter().reduce(lcm).unwrap()
}

fn main() {
    let is_sample = false;
    let path = get_file_path(8, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines().clone()));
    println!("Part 2: {}", part_2(contents.lines()));
}
