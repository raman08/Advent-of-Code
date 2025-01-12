use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;
use std::{collections::HashSet, env};

#[derive(Clone)]
struct Garden {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct Measurement {
    area: usize,
    paremeter: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Fence {
    direction: Direction,
    row: usize,
    col: usize,
}

impl Fence {
    fn next(&mut self) {
        match self.direction {
            Direction::South | Direction::North => self.col += 1,
            Direction::West | Direction::East => self.row += 1,
        }
    }

    fn prev(&mut self) {
        match self.direction {
            Direction::South | Direction::North => self.col = self.col.wrapping_sub(1),
            Direction::West | Direction::East => self.row = self.row.wrapping_sub(1),
        }
    }
}

impl Garden {
    fn new(lines: &str) -> Self {
        let grid = lines
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            height,
            width,
        }
    }

    fn measure_area_parameter(
        &mut self,
        row: usize,
        col: usize,
        measurement: &mut Measurement,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if visited[row][col] {
            return;
        }

        visited[row][col] = true;
        let plant = self.grid[row][col];

        measurement.area += 1;

        if row > 0 && self.grid[row - 1][col] == plant {
            self.measure_area_parameter(row - 1, col, measurement, visited)
        } else {
            measurement.paremeter += 1;
        }

        if row < self.height - 1 && self.grid[row + 1][col] == plant {
            self.measure_area_parameter(row + 1, col, measurement, visited)
        } else {
            measurement.paremeter += 1;
        }

        if col > 0 && self.grid[row][col - 1] == plant {
            self.measure_area_parameter(row, col - 1, measurement, visited)
        } else {
            measurement.paremeter += 1;
        }

        if col < self.width - 1 && self.grid[row][col + 1] == plant {
            self.measure_area_parameter(row, col + 1, measurement, visited)
        } else {
            measurement.paremeter += 1;
        }
    }

    fn measure_area_sides(
        &mut self,
        row: usize,
        col: usize,
        measurement: &mut Measurement,
        visited: &mut Vec<Vec<bool>>,
        fences: &mut HashSet<Fence>,
    ) {
        if visited[row][col] {
            return;
        }

        visited[row][col] = true;
        let plant = self.grid[row][col];

        measurement.area += 1;

        if row > 0 && self.grid[row - 1][col] == plant {
            self.measure_area_sides(row - 1, col, measurement, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::North,
                row,
                col,
            });
        }

        if row < self.height - 1 && self.grid[row + 1][col] == plant {
            self.measure_area_sides(row + 1, col, measurement, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::South,
                row,
                col,
            });
        }

        if col > 0 && self.grid[row][col - 1] == plant {
            self.measure_area_sides(row, col - 1, measurement, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::West,
                row,
                col,
            });
        }

        if col < self.width - 1 && self.grid[row][col + 1] == plant {
            self.measure_area_sides(row, col + 1, measurement, visited, fences);
        } else {
            fences.insert(Fence {
                direction: Direction::East,
                row,
                col,
            });
        }
    }
}

fn count_sides(fences: &mut HashSet<Fence>) -> usize {
    let mut out = 0;

    while !fences.is_empty() {
        let fence_org = fences.iter().next().unwrap().clone();
        let mut fence = fence_org.clone();

        while fences.remove(&fence) {
            fence.next();
        }

        let mut fence = fence_org;
        fence.prev();
        while fences.remove(&fence) {
            fence.prev();
        }

        out += 1;
    }

    out
}

fn part_1(lines: &str) -> usize {
    let mut garden = Garden::new(lines);

    let mut visited = vec![vec![false; garden.width]; garden.height];

    let mut out = 0;
    for i in 0..garden.height {
        for j in 0..garden.width {
            if visited[i][j] {
                continue;
            }

            let mut measurement = Measurement {
                area: 0,
                paremeter: 0,
            };

            garden.measure_area_parameter(i, j, &mut measurement, &mut visited);

            out += measurement.paremeter * measurement.area;
        }
    }

    out
}

fn part_2(lines: &str) -> usize {
    let mut garden = Garden::new(lines);
    let mut visited = vec![vec![false; garden.width]; garden.height];
    let mut fences = HashSet::new();
    let mut out = 0;

    for i in 0..garden.height {
        for j in 0..garden.width {
            if visited[i][j] {
                continue;
            }

            let mut measurement = Measurement {
                area: 0,
                paremeter: 0,
            };

            garden.measure_area_sides(i, j, &mut measurement, &mut visited, &mut fences);

            out += count_sides(&mut fences) * measurement.area;
        }
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(12, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
