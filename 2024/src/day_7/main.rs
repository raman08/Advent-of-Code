use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;

fn part_1(lines: &str) -> usize {
    let equations = lines
        .lines()
        .map(|line| {
            let (total, numbers) = line.split_once(':').unwrap();
            let total = total.parse::<usize>().unwrap();
            let numbers = numbers
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().expect("number parsing falied"))
                .collect_vec();

            (total, numbers)
        })
        .collect_vec();

    equations
        .iter()
        .filter(|&eq| {
            can_pass(
                eq.clone().0,
                eq.clone().1[0],
                eq.clone().1[1..].into(),
                false,
            )
        })
        .map(|e| e.0)
        .reduce(|e, acc| acc + e)
        .unwrap_or(0)
}

fn can_pass(total: usize, start: usize, numbers: Vec<usize>, concat: bool) -> bool {
    if numbers.is_empty() {
        return total == start;
    }

    if start > total {
        return false;
    }

    can_pass(total, start * numbers[0], numbers[1..].into(), concat)
        || can_pass(total, start + numbers[0], numbers[1..].into(), concat)
        || (concat
            && can_pass(
                total,
                concat_numbers(start, numbers[0]),
                numbers[1..].into(),
                concat,
            ))
}

fn concat_numbers(a: usize, b: usize) -> usize {
    let mut offset = 1;
    while offset <= b {
        offset *= 10;
    }

    a * offset + b
}

fn part_2(lines: &str) -> usize {
    let equations = lines
        .lines()
        .map(|line| {
            let (total, numbers) = line.split_once(':').unwrap();
            let total = total.parse::<usize>().unwrap();
            let numbers = numbers
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().expect("number parsing falied"))
                .collect_vec();

            (total, numbers)
        })
        .collect_vec();

    equations
        .iter()
        .filter(|&eq| {
            can_pass(
                eq.clone().0,
                eq.clone().1[0],
                eq.clone().1[1..].into(),
                true,
            )
        })
        .map(|e| e.0)
        .reduce(|e, acc| acc + e)
        .unwrap_or(0)
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(7, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[test]
fn test_concat() {
    assert_eq!(concat_numbers(2, 3), 23);
    assert_eq!(concat_numbers(20, 3), 203);
    assert_eq!(concat_numbers(2, 30), 230);
    assert_eq!(concat_numbers(20, 20), 2020);
}
