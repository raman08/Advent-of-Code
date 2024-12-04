use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;

fn part_1(lines: String) -> i32 {
    let reports = lines.trim().split("\n").collect_vec();

    reports
        .iter()
        .map(|levels| {
            let levels = levels
                .split_whitespace()
                .map(|l| l.parse::<i32>().ok().unwrap())
                .collect_vec();

            let safe = is_level_safe(&levels);

            // println!("{:?} is {:?}", levels, safe);

            match safe {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn part_2(lines: String) -> i32 {
    let reports = lines.trim().split("\n").collect_vec();

    reports
        .iter()
        .map(|levels| {
            let levels = levels
                .split_whitespace()
                .map(|l| l.parse::<i32>().ok().unwrap())
                .collect_vec();

            let safe = levels
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mut new_levels: Vec<i32> = Vec::with_capacity(levels.len() - 1);
                    new_levels.extend(&levels[0..i]);
                    new_levels.extend(&levels[i + 1..]);
                    new_levels
                })
                .any(|levels| is_level_safe(&levels));

            match safe {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn is_level_safe(levels: &Vec<i32>) -> bool {
    let diff = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|pair| {
            let diff = pair.0 - pair.1;
            // println!("{:?} is diff {:?} ", pair, diff);
            diff
        })
        .collect_vec();

    let pass = (diff.iter().all(|d| d > &0) && diff.iter().all(|d| d >= &1 && d <= &3))
        || (diff.iter().all(|d| d < &0)
            && diff.iter().all(|d| i32::abs(*d) >= 1 && i32::abs(*d) <= 3));
    pass
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(2, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.clone()));
    println!("Part 2: {}", part_2(contents.clone()));
}
