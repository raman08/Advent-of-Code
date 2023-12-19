use aoc2023::{get_file_path, read_file};
use std::str::Lines;

fn part_1(lines: Lines) -> i32 {
    lines
        .map(|line| {
            let line: Vec<_> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            let mut history = Vec::new();
            history.push(line);

            while history.last().unwrap().iter().any(|x| x != &0) {
                let last = history.last().unwrap();
                let values: Vec<_> = last.windows(2).map(|x| x[1] - x[0]).collect();
                history.push(values);
            }

            let mut last_num = 0;

            history.iter().rev().for_each(|l| {
                last_num += l.last().unwrap();
            });

            last_num
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn part_2(lines: Lines) -> i32 {
    lines
        .map(|line| {
            let line: Vec<_> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            let mut history = Vec::new();
            history.push(line);

            while history.last().unwrap().iter().any(|x| x != &0) {
                let last = history.last().unwrap();
                let values: Vec<_> = last.windows(2).map(|x| x[1] - x[0]).collect();
                history.push(values);
            }

            let mut last_num = 0;

            history.iter().rev().for_each(|l| {
                last_num = l.first().unwrap() - last_num;
            });

            last_num
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn main() {
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(9, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
