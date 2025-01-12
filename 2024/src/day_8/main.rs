use std::{
    collections::{HashMap, HashSet},
    env,
    ops::{Add, Sub},
};

use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    row: i64,
    col: i64,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl Position {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }

    fn in_bound(self, width: i64, height: i64) -> bool {
        0 <= self.col && self.col < width && 0 <= self.row && self.row < height
    }
}

fn part_1(lines: &str) -> usize {
    let mut antineas: HashMap<char, Vec<Position>> = HashMap::new();

    lines.lines().enumerate().for_each(|(row, line)| {
        line.char_indices()
            .filter(|(_, ch)| ch != &'.')
            .for_each(|(col, ch)| {
                antineas
                    .entry(ch)
                    .or_default()
                    .push(Position::new(row as i64, col as i64))
            })
    });

    let height = lines.lines().count() as i64;
    let width = lines.lines().next().expect("no elements???????").len() as i64;

    let mut antinodes = HashSet::new();

    antineas.iter().for_each(|(_, val)| {
        val.iter().combinations(2).for_each(|comb| {
            let first = *comb[0];
            let second = *comb[1];

            let diff = second - first;

            let a_first = second + diff;
            let a_second = first - diff;

            if a_first.in_bound(width, height) {
                antinodes.insert(a_first);
            }

            if a_second.in_bound(width, height) {
                antinodes.insert(a_second);
            }
        })
    });

    antinodes.len()
}

fn part_2(lines: &str) -> usize {
    let mut antineas: HashMap<char, Vec<Position>> = HashMap::new();

    lines.lines().enumerate().for_each(|(row, line)| {
        line.char_indices()
            .filter(|(_, ch)| ch != &'.')
            .for_each(|(col, ch)| {
                antineas
                    .entry(ch)
                    .or_default()
                    .push(Position::new(row as i64, col as i64))
            })
    });

    let height = lines.lines().count() as i64;
    let width = lines.lines().next().expect("no elements???????").len() as i64;

    let mut antinodes = HashSet::new();

    antineas.iter().for_each(|(_, val)| {
        val.iter().combinations(2).for_each(|comb| {
            let first = *comb[0];
            let second = *comb[1];

            let diff = second - first;

            let mut antinode = first;

            while antinode.in_bound(width, height) {
                antinodes.insert(antinode);
                antinode = antinode + diff;
            }

            antinode = second;

            while antinode.in_bound(width, height) {
                antinodes.insert(antinode);
                antinode = antinode - diff;
            }
        })
    });

    antinodes.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(8, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
