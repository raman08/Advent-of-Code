use std::collections::{HashMap, HashSet};

use aoc_2024::{get_file_path, read_file};

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Up,
    Dowm,
    Left,
    Right,
}

type Grid = HashMap<(isize, isize), Tile>;

fn create_grid(lines: &String) -> Grid {
    HashMap::from_iter(lines.lines().enumerate().flat_map(|(line_num, line)| {
        line.chars().enumerate().map(move |(tile_num, tile)| {
            let tt = match tile {
                '.' => Tile::Empty,
                '#' => Tile::Obstacle,
                '^' => Tile::Up,
                '>' => Tile::Right,
                '<' => Tile::Left,
                'v' => Tile::Dowm,
                _ => unreachable!(),
            };
            ((line_num as isize, tile_num as isize), tt)
        })
    }))
}

fn part_1(lines: &String) -> i32 {
    let mut grid = create_grid(lines);

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let (mut inital_position, mut face) = grid
        .iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Left | Tile::Right | Tile::Up | Tile::Dowm => Some((*pos, *tile)),
            _ => None,
        })
        .expect("Should have initial position");

    grid.insert(inital_position, Tile::Empty);

    while grid.contains_key(&inital_position) {
        visited.insert(inital_position);

        let next_position = match face {
            Tile::Up => (inital_position.0 - 1, inital_position.1),
            Tile::Dowm => (inital_position.0 + 1, inital_position.1),
            Tile::Left => (inital_position.0, inital_position.1 - 1),
            Tile::Right => (inital_position.0, inital_position.1 + 1),
            _ => unreachable!(),
        };

        inital_position = if let Some(tile) = grid.get(&next_position) {
            if *tile != Tile::Empty {
                face = match face {
                    Tile::Up => Tile::Right,
                    Tile::Dowm => Tile::Left,
                    Tile::Left => Tile::Up,
                    Tile::Right => Tile::Dowm,
                    _ => unreachable!(),
                };
                inital_position
            } else {
                next_position
            }
        } else {
            next_position
        }
    }
    visited.len() as i32
}

fn will_stuck_in_loop(grid: &Grid, guard: ((isize, isize), Tile)) -> bool {
    let (mut pos, mut face) = guard;
    let mut visited = HashSet::new();

    while grid.contains_key(&pos) {
        if !visited.insert((pos, face)) {
            return true;
        }

        let (next_pos, next_face) = match face {
            Tile::Up => ((pos.0 - 1, pos.1), Tile::Right),
            Tile::Dowm => ((pos.0 + 1, pos.1), Tile::Left),
            Tile::Left => ((pos.0, pos.1 - 1), Tile::Up),
            Tile::Right => ((pos.0, pos.1 + 1), Tile::Dowm),
            _ => unreachable!(),
        };

        (pos, face) = if let Some(tile) = grid.get(&next_pos) {
            if *tile == Tile::Empty {
                (next_pos, face)
            } else {
                (pos, next_face)
            }
        } else {
            (next_pos, face)
        }
    }
    false
}

fn part_2(lines: &String) -> i32 {
    let mut grid = create_grid(lines);

    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();

    let (mut inital_position, mut face) = grid
        .iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Left | Tile::Right | Tile::Up | Tile::Dowm => Some((*pos, *tile)),
            _ => None,
        })
        .expect("Should have initial position");

    grid.insert(inital_position, Tile::Empty);

    let invalid_obs = inital_position;

    while grid.contains_key(&inital_position) {
        visited.insert(inital_position);

        let (next_pos, next_face) = match face {
            Tile::Up => ((inital_position.0 - 1, inital_position.1), Tile::Right),
            Tile::Dowm => ((inital_position.0 + 1, inital_position.1), Tile::Left),
            Tile::Left => ((inital_position.0, inital_position.1 - 1), Tile::Up),
            Tile::Right => ((inital_position.0, inital_position.1 + 1), Tile::Dowm),
            _ => unreachable!(),
        };

        (inital_position, face) = if let Some(tile) = grid.get(&next_pos) {
            if *tile == Tile::Empty {
                if grid.contains_key(&next_pos)
                    && next_pos != invalid_obs
                    && !visited.contains(&next_pos)
                {
                    grid.insert(next_pos, Tile::Obstacle);
                    if will_stuck_in_loop(&grid, (inital_position, next_face)) {
                        obstacles.insert(next_pos);
                    }
                    grid.insert(next_pos, Tile::Empty);
                }
                (next_pos, face)
            } else {
                (inital_position, next_face)
            }
        } else {
            (next_pos, face)
        }
    }
    obstacles.len() as i32
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(6, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
