use aoc2023::{get_file_path, read_file};
use std::str::Lines;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[default]
    NE,
    NW,
    SE,
    SW,
    SN,
    EW,
}

type Coordinates = [i32; 2];

#[derive(Default, Debug, Clone, Copy)]
struct Tile {
    direction: Direction,
    gates: [Coordinates; 2],
    is_edge: bool,
    is_start: bool,
    is_loop: bool,
    position: Coordinates,
}

impl Tile {
    fn new(gates: [Coordinates; 2], position: Coordinates, direction: Direction) -> Tile {
        Tile {
            direction,
            gates,
            is_edge: false,
            is_start: false,
            position,
            is_loop: false,
        }
    }

    fn to(&self, from: Coordinates) -> Coordinates {
        if self.gates[0] == from {
            self.gates[1]
        } else {
            self.gates[0]
        }
    }
}

#[derive(Default, Debug)]
struct Maze {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn fine_start_gates(&mut self) {
        let x_range = [
            (self.start.position[0] - 1).max(0),
            (self.start.position[0] + 1).min(self.tiles[0].len() as i32 - 1),
        ];
        let y_range = [
            (self.start.position[1] - 1).max(0),
            (self.start.position[1] + 1).min(self.tiles.len() as i32 - 1),
        ];

        let mut entries: Vec<Coordinates> = Vec::new();

        // Enumerate over every tile in the tiles Vec that is within our defined range
        for i in y_range[0]..=y_range[1] {
            for j in x_range[0]..=x_range[1] {
                if i == self.start.position[1] && j == self.start.position[0] {
                    continue;
                }

                if self.tiles[i as usize][j as usize]
                    .gates
                    .contains(&self.start.position)
                {
                    entries.push([j, i])
                }
            }
        }

        entries.sort();

        if entries.len() > 1 {
            self.start.gates[0] = entries[0];
            self.start.gates[1] = entries[1];
        }

        let direction = match (self.start.gates[0], self.start.gates[1]) {
            ([x, y], [x2, y2]) if x == x2 && y < y2 => Direction::SN,
            ([x, y], [x2, y2]) if x == x2 && y > y2 => Direction::SN,
            ([x, y], [x2, y2]) if x < x2 && y == y2 => Direction::EW,
            ([x, y], [x2, y2]) if (x < x2 && y > y2) && (self.start.position[1] < y) => {
                Direction::SE
            }
            ([x, y], [x2, y2]) if (x < x2 && y < y2) && (self.start.position[0] > x) => {
                Direction::SW
            }
            ([x, y], [x2, y2]) if x < x2 && y > y2 => Direction::NW,
            ([x, y], [x2, y2]) if x < x2 && y < y2 => Direction::NE,
            _ => Direction::default(),
        };
        self.start.direction = direction;
        self.tiles[self.start.position[1] as usize][self.start.position[0] as usize].direction =
            direction;
    }

    fn walk_loop(&mut self) -> Vec<Coordinates> {
        let mut walk = Vec::new();
        let mut next = self.start.gates[0];
        walk.push(next);

        let mut prev = self.start.position;

        while next != self.start.position {
            let tile = &mut self.tiles[next[1] as usize][next[0] as usize];

            tile.is_loop = true;
            let curr = next;
            next = tile.to(prev);
            prev = curr;
            walk.push(next);
        }

        walk
    }

    fn ray_cast_tile(&self, from: Coordinates) -> i32 {
        let mut count = 0;
        for i in 0..from[0] {
            let tile = &self.tiles[from[1] as usize][i as usize];
            if tile.is_loop
                && (tile.direction == Direction::SN
                    || tile.direction == Direction::SW
                    || tile.direction == Direction::SE)
            {
                count += 1;
            }
        }
        if count % 2 == 0 {
            0
        } else {
            1
        }
    }

    fn count_enclosed(&self) -> i32 {
        let mut count = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if !tile.is_loop && !tile.is_edge {
                    count += self.ray_cast_tile([x as i32, y as i32]);
                }
            }
        }
        count
    }
}

fn parse_lines(lines: Lines) -> Maze {
    let mut tiles = Vec::new();
    let mut maze = Maze::default();

    for (y, line) in lines.clone().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let mut tile = match c {
                '-' => Tile::new(
                    [[x as i32 - 1, y as i32], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::EW,
                ),
                '|' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32, y as i32 + 1]],
                    [x as i32, y as i32],
                    Direction::SN,
                ),
                'L' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::NE,
                ),
                'J' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::NW,
                ),
                '7' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::SW,
                ),
                'F' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::SE,
                ),
                '.' => Tile::new(
                    Default::default(),
                    [x as i32, y as i32],
                    Direction::default(),
                ),
                'S' => Tile {
                    direction: Direction::default(),
                    gates: [[0; 2], [0; 2]],
                    is_edge: false,
                    is_loop: true,
                    is_start: true,
                    position: [x as i32, y as i32],
                },

                _ => unreachable!("Not a valid tile"),
            };

            if tile.is_start {
                maze.start = tile;
            }

            if x == 0 || x == line.len() - 1 || y == 0 || y == lines.clone().count() - 1 {
                tile.is_edge = true;
            }

            row.push(tile);
        }

        tiles.push(row);
    }

    maze.tiles = tiles;
    maze.fine_start_gates();

    maze
}

fn part_1(lines: Lines) -> i32 {
    let mut maze = parse_lines(lines);
    let walk = maze.walk_loop();

    (walk.len() / 2) as i32
}

fn part_2(lines: Lines) -> i32 {
    let mut maze = parse_lines(lines);
    maze.walk_loop();

    maze.count_enclosed()
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(10, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn part1_test_1() {
        let contents_1 = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part_1(contents_1.lines()), 4);
    }

    #[test]
    fn part1_test_2() {
        let contents_2 = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        assert_eq!(part_1(contents_2.lines()), 8);
    }
}
