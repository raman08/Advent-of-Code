use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use std::{str::Lines, usize};

type Grid = Vec<Vec<char>>;

enum Direction {
    North,
    East,
    West,
    South,
}

fn tilt(grid: Grid, direction: Direction) -> Grid {
    let mut tilted_grid: Grid = grid.clone();

    match direction {
        Direction::North => {
            for i in 0..tilted_grid.len() {
                for j in 0..tilted_grid[i].len() {
                    if tilted_grid[i][j] == 'O' {
                        let mut k = i;

                        while k > 0 && tilted_grid[k - 1][j] == '.' {
                            k -= 1;
                        }

                        tilted_grid[i][j] = '.';
                        tilted_grid[k][j] = 'O';
                    }
                }
            }
        }
        Direction::East => {
            for i in 0..tilted_grid.len() {
                for j in (0..tilted_grid[i].len()).rev() {
                    if tilted_grid[i][j] == 'O' {
                        let mut k = j;

                        while k < tilted_grid[0].len() - 1 && tilted_grid[i][k + 1] == '.' {
                            k += 1;
                        }

                        tilted_grid[i][j] = '.';
                        tilted_grid[i][k] = 'O';
                    }
                }
            }
        }
        Direction::West => {
            for i in 0..tilted_grid.len() {
                for j in 0..tilted_grid[i].len() {
                    if tilted_grid[i][j] == 'O' {
                        let mut k = j;

                        while k > 0 && tilted_grid[i][k - 1] == '.' {
                            k -= 1;
                        }

                        tilted_grid[i][j] = '.';
                        tilted_grid[i][k] = 'O';
                    }
                }
            }
        }
        Direction::South => {
            for i in (0..tilted_grid.len()).rev() {
                for j in 0..tilted_grid[i].len() {
                    if tilted_grid[i][j] == 'O' {
                        let mut k = i;

                        while k < tilted_grid.len() - 1 && tilted_grid[k + 1][j] == '.' {
                            k += 1;
                        }

                        tilted_grid[i][j] = '.';
                        tilted_grid[k][j] = 'O';
                    }
                }
            }
        }
    }

    return tilted_grid;
}

fn cycle(grid: &Grid, ammount: usize) -> Grid {
    let mut rev_grid = grid.clone();

    for _ in 0..ammount {
        rev_grid = tilt(rev_grid, Direction::North);
        rev_grid = tilt(rev_grid, Direction::West);
        rev_grid = tilt(rev_grid, Direction::South);
        rev_grid = tilt(rev_grid, Direction::East);
    }

    rev_grid
}

fn parse_input(lines: Lines) -> Grid {
    lines.map(|line| line.chars().collect()).collect()
}

fn calculate_score(grid: &Grid) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| {
            let no_of_rounded = row.iter().filter(|c| **c == 'O').collect_vec().len();

            no_of_rounded * (idx + 1)
        })
        .sum()
}

fn part_1(lines: Lines) -> i32 {
    let grid: Grid = parse_input(lines);

    let tilted_grid = tilt(grid, Direction::North);

    calculate_score(&tilted_grid) as i32
}

fn part_2(lines: Lines) -> i32 {
    let mut grid: Grid = parse_input(lines);
    let ammount = 1000000000;

    let mut seen = vec![grid.clone()];

    loop {
        grid = cycle(&grid, 1);

        if let Some(idx) = seen.iter().position(|g| g == &grid) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (ammount - idx) % cycle_len;

            return calculate_score(&seen[final_idx]) as i32;
        }

        seen.push(grid.clone());
    }
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(14, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}

#[cfg(test)]
mod tests {
    use crate::{cycle, parse_input, tilt, Direction};
    use aoc2023::{get_file_path, read_file};

    #[test]
    fn check_tilt() {
        let is_sample = true;
        let path = get_file_path(14, is_sample);
        let contents = read_file(path);

        let grid = parse_input(contents.lines());

        let tilted_grid = tilt(grid, Direction::North);

        let north_content = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        assert_eq!(tilted_grid, parse_input(north_content.lines()));
    }

    #[test]
    fn check_cycle() {
        let is_sample = true;
        let path = get_file_path(14, is_sample);
        let contents = read_file(path);

        let grid = parse_input(contents.lines());

        let cycled_content_1 = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        assert_eq!(cycle(&grid, 1), parse_input(cycled_content_1.lines()));

        let cycled_content_2 = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        assert_eq!(cycle(&grid, 2), parse_input(cycled_content_2.lines()));

        let cycled_content_3 = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        assert_eq!(cycle(&grid, 3), parse_input(cycled_content_3.lines()));
    }
}
