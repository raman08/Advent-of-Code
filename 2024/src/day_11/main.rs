use aoc_2024::{get_file_path, read_file};
use std::{collections::HashMap, env};

fn iteration(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut out: HashMap<usize, usize> = HashMap::new();

    stones.iter().for_each(|(stone, count)| {
        if stone == &0 {
            let cnt = out.entry(1).or_default();
            *cnt += count;
        } else if stone.to_string().len() % 2 == 0 {
            let str_stone = stone.to_string();
            let left = str_stone[0..str_stone.len() / 2]
                .parse::<usize>()
                .expect("Failed to convert left {stone}");

            let right = str_stone[str_stone.len() / 2..]
                .parse::<usize>()
                .expect("Failed to convert right {stone}");

            let cnt = out.entry(left).or_default();
            *cnt += count;

            let cnt = out.entry(right).or_default();
            *cnt += count;
        } else {
            let cnt = out.entry(stone * 2024).or_default();
            *cnt += count;
        }
    });

    out
}

fn part_1(lines: &str) -> usize {
    let stones = lines
        .trim()
        .split_ascii_whitespace()
        .map(|stone| {
            (
                stone.parse::<usize>().expect("Failed to convert {stone}"),
                1,
            )
        })
        .collect::<HashMap<usize, usize>>();

    let mut out = stones;
    for _ in 0..25 {
        out = iteration(&out);
    }

    out.values().sum()
}

fn part_2(lines: &str) -> usize {
    let stones = lines
        .trim()
        .split_ascii_whitespace()
        .map(|stone| {
            (
                stone.parse::<usize>().expect("Failed to convert {stone}"),
                1,
            )
        })
        .collect::<HashMap<usize, usize>>();

    let mut out = stones;
    for _ in 0..75 {
        out = iteration(&out);
    }

    out.values().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(11, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
