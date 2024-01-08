use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use num::Signed;
use std::{
    isize,
    ops::{Add, Div},
    str::Lines,
};

#[derive(PartialEq, Debug, Eq, Clone, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    fn move_in_with_length(self: Self, dir: Direction, step: isize) -> Point {
        match dir {
            Direction::North => self + Point { x: 0, y: step },
            Direction::East => self + Point { x: step, y: 0 },
            Direction::West => self + Point { x: -step, y: 0 },
            Direction::South => self + Point { x: 0, y: -step },
        }
    }

    fn manhattan_distance(self: Self, other: Self) -> isize {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

fn shoelace_formula(points: &Vec<Point>) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0, 0), |(area, perimeter), (idx, point)| {
                let nidx = (idx + 1) % len;
                let npoint = points[nidx];

                let npara = perimeter + point.manhattan_distance(npoint);
                let narea = area + (point.y * npoint.x) - (point.x * npoint.y);

                (narea, npara)
            });

    area.abs().add(perimeter).div(2).add(1)
}

fn part_1(lines: Lines) -> isize {
    let points = lines
        .map(|line| {
            let (dir, len, _) = line.split(" ").collect_tuple().unwrap();

            return (
                match dir {
                    "R" => Direction::East,
                    "L" => Direction::West,
                    "U" => Direction::North,
                    "D" => Direction::South,
                    _ => unreachable!("Invalid Direction"),
                },
                len.parse::<isize>().unwrap(),
            );
        })
        .collect_vec();

    let mut start = Point { x: 0, y: 0 };

    let mut edges = vec![start];

    points.iter().for_each(|(dir, step)| {
        let new = start.move_in_with_length(dir.clone(), *step);
        edges.push(new);
        start = new;
    });

    shoelace_formula(&edges)
}

fn part_2(lines: Lines) -> isize {
    let points = lines
        .map(|line| {
            let (_, _, hex) = line.split(" ").collect_tuple().unwrap();

            let hex = &hex[1..hex.len() - 1];

            let dir = &hex.chars().last().unwrap();

            let dir = match dir {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => unreachable!("Invalid hex end"),
            };

            let len = isize::from_str_radix(&hex[1..6], 16).unwrap();

            return (
                match dir {
                    "R" => Direction::East,
                    "L" => Direction::West,
                    "U" => Direction::North,
                    "D" => Direction::South,
                    _ => unreachable!("Invalid Direction"),
                },
                len,
            );
        })
        .collect_vec();

    let mut start = Point { x: 0, y: 0 };

    let mut edges = vec![start];

    points.iter().for_each(|(dir, step)| {
        let new = start.move_in_with_length(dir.clone(), *step);
        edges.push(new);
        start = new;
    });

    shoelace_formula(&edges)
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(18, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
