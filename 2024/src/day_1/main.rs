use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;
use std::{collections::BinaryHeap, i32};

#[allow(unused_variables)]
fn part_1(lines: String) -> i32 {
    let mut left: Vec<i32>;
    let mut right: Vec<i32>;
    let numbers = lines
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect_vec();

    let right: BinaryHeap<i32> = numbers.iter().skip(1).step_by(2).cloned().collect();

    let left: BinaryHeap<i32> = numbers.iter().step_by(2).cloned().collect();

    left.into_sorted_vec()
        .iter()
        .zip(right.into_sorted_vec().iter())
        .map(|nums| i32::abs(nums.0 - nums.1))
        .sum()
}

#[allow(unused_variables)]
fn part_2(lines: String) -> i32 {
    let mut left: Vec<i32>;
    let mut right: Vec<i32>;
    let numbers = lines
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect_vec();

    let left = numbers.iter().step_by(2).collect_vec();
    let right = numbers.iter().skip(1).step_by(2).collect_vec();

    left.iter()
        .map(|num| **num * (right.iter().filter(|&b| b == num).count() as i32))
        .sum()
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(1, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.clone()));
    println!("Part 2: {}", part_2(contents.clone()));
}
