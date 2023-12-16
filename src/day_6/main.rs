use aoc2023::{get_file_path, read_file};
use std::{i64, str::Lines};

fn part_1(lines: Lines) -> i32 {
    let parsed: Vec<Vec<i64>> = lines
        .map(|line| {
            line.split_whitespace().skip(1)
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let times = &parsed[0];
    let distances = &parsed[1];

    let mut ans = 1;

    times.iter().enumerate().for_each(|(i, time)| {
        let mut options = 0;

        let distance = &distances[i];

        for speed in 0..=time.clone() {
            let distance_covered = speed * (time - speed);

            if distance_covered > distance.clone() {
                options += 1;
            }
        }

        ans *= options;
    });

    ans
}

fn part_2(lines: Lines) -> i32 {
    let parsed: Vec<_> = lines
        .map(|line| {
            line.split_whitespace().skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    let time = &parsed[0];
    let distance = &parsed[1];

    let mut ans = 1;

    let mut options = 0;

    for speed in 0..=time.clone() {
        let distance_covered = speed * (time - speed);

        if distance_covered > distance.clone() {
            options += 1;
        }
    }

    ans *= options;

    ans
}

fn main() {
    let is_sample = false;
    let path = get_file_path(6, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
