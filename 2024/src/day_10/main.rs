use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn up(self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    fn down(self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    fn left(self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    fn right(self) -> Self {
        Self::new(self.row, self.col + 1)
    }
}

fn find_trailheads(
    grid: &HashMap<Position, usize>,
    curr: Position,
    visited: &mut HashSet<Position>,
) -> i32 {
    let height = grid.get(&curr).unwrap();

    if height == &9 && !visited.contains(&curr) {
        visited.insert(curr);
        return 1;
    }

    let mut result = 0;

    if grid.contains_key(&curr.up()) && grid.get(&curr.up()) == Some(&(height + 1)) {
        result += find_trailheads(grid, curr.up(), visited);
    }

    if grid.contains_key(&curr.down()) && grid.get(&curr.down()) == Some(&(height + 1)) {
        result += find_trailheads(grid, curr.down(), visited);
    }

    if grid.contains_key(&curr.left()) && grid.get(&curr.left()) == Some(&(height + 1)) {
        result += find_trailheads(grid, curr.left(), visited);
    }

    if grid.contains_key(&curr.right()) && grid.get(&curr.right()) == Some(&(height + 1)) {
        result += find_trailheads(grid, curr.right(), visited);
    }

    result
}

fn find_rating(grid: &HashMap<Position, usize>, curr: Position) -> i32 {
    let height = grid.get(&curr).unwrap();

    if height == &9 {
        return 1;
    }

    let mut result = 0;

    if grid.contains_key(&curr.up()) && grid.get(&curr.up()) == Some(&(height + 1)) {
        result += find_rating(grid, curr.up());
    }

    if grid.contains_key(&curr.down()) && grid.get(&curr.down()) == Some(&(height + 1)) {
        result += find_rating(grid, curr.down());
    }

    if grid.contains_key(&curr.left()) && grid.get(&curr.left()) == Some(&(height + 1)) {
        result += find_rating(grid, curr.left());
    }

    if grid.contains_key(&curr.right()) && grid.get(&curr.right()) == Some(&(height + 1)) {
        result += find_rating(grid, curr.right());
    }

    result
}

fn part_1(lines: &str) -> usize {
    let mut grid = HashMap::new();

    let mut start_postition = Vec::new();
    lines.lines().enumerate().for_each(|(row, line)| {
        line.char_indices().for_each(|(col, ch)| {
            let ch = ch.to_digit(10).unwrap() as usize;
            let pos = Position::new(row as isize, col as isize);
            grid.entry(pos).or_insert(ch);

            if ch == 0 {
                start_postition.push(pos);
            }
        })
    });

    start_postition
        .iter()
        .map(|pos| find_trailheads(&grid, *pos, &mut HashSet::new()))
        .reduce(|val, acc| acc + val)
        .unwrap() as usize
}

fn part_2(lines: &str) -> usize {
    let mut grid = HashMap::new();

    let mut start_postition = Vec::new();
    lines.lines().enumerate().for_each(|(row, line)| {
        line.char_indices().for_each(|(col, ch)| {
            let ch = ch.to_digit(10).unwrap() as usize;
            let pos = Position::new(row as isize, col as isize);
            grid.entry(pos).or_insert(ch);

            if ch == 0 {
                start_postition.push(pos);
            }
        })
    });

    start_postition
        .iter()
        .map(|pos| find_rating(&grid, *pos))
        .reduce(|val, acc| acc + val)
        .unwrap() as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(10, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
