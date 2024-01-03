use aoc2023::{get_file_path, read_file};
use std::{
    collections::{HashMap, VecDeque},
    i32,
    ops::Add,
    str::Lines,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Debug)]
enum Artifect {
    Empty,

    // /
    MirrorAsc,

    // \
    MirrorDesc,

    // |
    SplitterV,

    // -
    SplitterH,
}

#[derive(Hash, Clone, Debug, Copy, PartialEq, Eq)]
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

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    point: Point,
    direction: Direction,
}

#[derive(Clone, Debug)]
struct Tile {
    artifect: Artifect,
    beams: Vec<Beam>,
}

#[derive(Clone)]
struct Grid {
    cells: HashMap<Point, Tile>,
}

impl Grid {
    fn new(lines: Lines) -> Self {
        let mut cells = HashMap::new();

        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                let tile = Tile {
                    artifect: match char {
                        '\\' => Artifect::MirrorDesc,
                        '/' => Artifect::MirrorAsc,
                        '|' => Artifect::SplitterV,
                        '-' => Artifect::SplitterH,
                        '.' => Artifect::Empty,
                        _ => unreachable!("Unknown Artifect"),
                    },
                    beams: vec![],
                };

                cells.insert(
                    Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    tile,
                );
            }
        }

        Self { cells }
    }

    fn traverse(&self, start: Beam) -> Grid {
        let mut clone: Grid = self.clone();

        let mut queue = VecDeque::new();

        queue.push_back(start);

        while let Some(beam) = queue.pop_front() {
            if let Some(tile) = clone.cells.get_mut(&beam.point) {
                if tile.beams.contains(&beam) {
                    continue;
                }

                tile.beams.push(beam);
            } else {
                continue;
            }

            let state = clone.get_next_state(beam);

            for s in state {
                queue.push_back(s);
            }
        }

        clone
    }

    fn get_next_state(&self, beam: Beam) -> Vec<Beam> {
        let tile = self.cells.get(&beam.point);

        if tile.is_none() {
            return vec![];
        }

        let tile = tile.unwrap();

        match tile.artifect {
            Artifect::Empty => match beam.direction {
                Direction::North => {
                    let new_beam = Beam {
                        direction: beam.direction,
                        point: beam.point + Point { x: 0, y: -1 },
                    };
                    return vec![new_beam];
                }
                Direction::East => {
                    let new_beam = Beam {
                        direction: beam.direction,
                        point: beam.point + Point { x: 1, y: 0 },
                    };
                    return vec![new_beam];
                }
                Direction::West => {
                    let new_beam = Beam {
                        direction: beam.direction,
                        point: beam.point + Point { x: -1, y: 0 },
                    };
                    return vec![new_beam];
                }
                Direction::South => {
                    let new_beam = Beam {
                        direction: beam.direction,
                        point: beam.point + Point { x: 0, y: 1 },
                    };
                    return vec![new_beam];
                }
            },
            Artifect::MirrorAsc => match beam.direction {
                Direction::North => {
                    let new_beam = Beam {
                        direction: Direction::East,
                        point: beam.point + Point { x: 1, y: 0 },
                    };
                    return vec![new_beam];
                }
                Direction::East => {
                    let new_beam = Beam {
                        direction: Direction::North,
                        point: beam.point + Point { x: 0, y: -1 },
                    };
                    return vec![new_beam];
                }
                Direction::West => {
                    let new_beam = Beam {
                        direction: Direction::South,
                        point: beam.point + Point { x: 0, y: 1 },
                    };
                    return vec![new_beam];
                }
                Direction::South => {
                    let new_beam = Beam {
                        direction: Direction::West,
                        point: beam.point + Point { x: -1, y: 0 },
                    };
                    return vec![new_beam];
                }
            },
            Artifect::MirrorDesc => match beam.direction {
                Direction::North => {
                    let new_beam = Beam {
                        direction: Direction::West,
                        point: beam.point + Point { x: -1, y: 0 },
                    };
                    return vec![new_beam];
                }
                Direction::East => {
                    let new_beam = Beam {
                        direction: Direction::South,
                        point: beam.point + Point { x: 0, y: 1 },
                    };
                    return vec![new_beam];
                }
                Direction::West => {
                    let new_beam = Beam {
                        direction: Direction::North,
                        point: beam.point + Point { x: 0, y: -1 },
                    };
                    return vec![new_beam];
                }
                Direction::South => {
                    let new_beam = Beam {
                        direction: Direction::East,
                        point: beam.point + Point { x: 1, y: 0 },
                    };
                    return vec![new_beam];
                }
            },
            Artifect::SplitterV => match beam.direction {
                Direction::North | Direction::South => {
                    let next_point = if beam.direction == Direction::North {
                        beam.point + Point { x: 0, y: -1 }
                    } else {
                        beam.point + Point { x: 0, y: 1 }
                    };

                    return vec![Beam {
                        direction: beam.direction,
                        point: next_point,
                    }];
                }
                Direction::East | Direction::West => {
                    let beam_up = Beam {
                        direction: Direction::North,
                        point: beam.point + Point { x: 0, y: -1 },
                    };
                    let beam_down = Beam {
                        direction: Direction::South,
                        point: beam.point + Point { x: 0, y: 1 },
                    };
                    return vec![beam_up, beam_down];
                }
            },
            Artifect::SplitterH => match beam.direction {
                Direction::North | Direction::South => {
                    let beam_left = Beam {
                        direction: Direction::West,
                        point: beam.point + Point { x: -1, y: 0 },
                    };
                    let beam_right = Beam {
                        direction: Direction::East,
                        point: beam.point + Point { x: 1, y: 0 },
                    };
                    return vec![beam_left, beam_right];
                }
                Direction::East | Direction::West => {
                    let next_point = if beam.direction == Direction::East {
                        beam.point + Point { x: 1, y: 0 }
                    } else {
                        beam.point + Point { x: -1, y: 0 }
                    };

                    return vec![Beam {
                        direction: beam.direction,
                        point: next_point,
                    }];
                }
            },
        }
    }

    fn count_energized(&self) -> usize {
        self.cells.values().filter(|x| x.beams.len() != 0).count()
    }
}

fn part_1(lines: Lines) -> i32 {
    let grid = Grid::new(lines);

    let enerzied_grid = grid.traverse(Beam {
        point: Point { x: 0, y: 0 },
        direction: Direction::East,
    });
    enerzied_grid.count_energized() as i32
}

fn part_2(lines: Lines) -> i32 {
    let grid = Grid::new(lines);

    let height = grid.cells.keys().map(|c| c.y).max().unwrap();
    let width = grid.cells.keys().map(|c| c.x).max().unwrap();

    let mut best = 0;

    let mut run_it = |x, y, direction| {
        let cloned = grid.traverse(Beam {
            direction,
            point: Point {
                x: x as isize,
                y: y as isize,
            },
        });

        // first time using max
        best = best.max(cloned.count_energized());
    };

    for x in 0..=width {
        run_it(x, 0, Direction::South);
        run_it(x, height, Direction::North);
    }

    for y in 0..=height {
        run_it(0, y, Direction::East);
        run_it(width, y, Direction::West);
    }

    best as i32
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(16, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
