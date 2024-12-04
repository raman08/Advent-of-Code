use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    ops::Add,
    str::Lines,
};

#[derive(Hash, PartialOrd, Ord, Clone, Debug, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
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
    fn valid_next(&self, grid: &Vec<Vec<usize>>) -> Vec<(Direction, Point)> {
        let mut next = Vec::new();

        if self.x > 0 {
            next.push((
                Direction::West,
                Self {
                    x: self.x - 1,
                    y: self.y,
                },
            ));
        }
        if self.y > 0 {
            next.push((
                Direction::North,
                Self {
                    x: self.x,
                    y: self.y - 1,
                },
            ));
        }
        if self.x < grid[0].len() - 1 {
            next.push((
                Direction::East,
                Self {
                    x: self.x + 1,
                    y: self.y,
                },
            ));
        }
        if self.y < grid.len() - 1 {
            next.push((
                Direction::South,
                Self {
                    x: self.x,
                    y: self.y + 1,
                },
            ));
        }
        next
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn opposite(&self) -> Self {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
struct Node {
    position: Point,
    direction: Direction,
    dir_count: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_neighbours(grid: &Vec<Vec<usize>>, curr: &Node, min: usize, max: usize) -> Vec<Node> {
    let mut neighbours = Vec::new();

    for (direction, point) in curr.position.valid_next(grid) {
        if direction == curr.direction.opposite() {
            continue;
        }

        if direction != curr.direction && curr.dir_count >= min {
            neighbours.push(Node {
                position: point,
                direction,
                dir_count: 1,
            });
        } else if direction == curr.direction && curr.dir_count < max {
            neighbours.push(Node {
                position: point,
                direction,
                dir_count: curr.dir_count + 1,
            });
        }
    }

    neighbours
}

fn path_find(
    grid: &Vec<Vec<usize>>,
    start: Point,
    end: Point,
    min: usize,
    max: usize,
) -> Option<usize> {
    let mut distance = HashMap::new();

    distance.insert(
        Node {
            position: start.clone(),
            direction: Direction::East,
            dir_count: 0,
        },
        0,
    );

    distance.insert(
        Node {
            position: start.clone(),
            direction: Direction::South,
            dir_count: 0,
        },
        0,
    );

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        node: Node {
            position: start.clone(),
            direction: Direction::East,
            dir_count: 0,
        },
    });

    heap.push(State {
        cost: 0,
        node: Node {
            position: start.clone(),
            direction: Direction::South,
            dir_count: 0,
        },
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node.position == end {
            return Some(cost);
        }

        let neighbours = find_neighbours(grid, &node, min, max);
        // println!("{:?} -> {:?}", &node, &neighbours);

        for neighbour in neighbours {
            let new_cost = cost + grid[neighbour.position.y][neighbour.position.x];

            if let Some(&best) = distance.get(&neighbour) {
                if new_cost >= best {
                    continue;
                }
            }

            distance.insert(neighbour.clone(), new_cost);
            heap.push(State {
                cost: new_cost,
                node: neighbour,
            });
        }
    }

    None
}

fn part_1(lines: Lines) -> usize {
    let grid = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let start = Point { x: 0, y: 0 };

    let end = Point {
        x: grid[0].len() - 1,
        y: grid.len() - 1,
    };

    path_find(&grid, start, end, 0, 3).expect("No valid path found!!")
}

fn part_2(lines: Lines) -> usize {
    let grid = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let start = Point { x: 0, y: 0 };

    let end = Point {
        x: grid[0].len() - 1,
        y: grid.len() - 1,
    };

    path_find(&grid, start, end, 4, 10).expect("No valid path found!!")
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(17, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
