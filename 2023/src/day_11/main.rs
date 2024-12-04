use aoc2023::{get_file_path, read_file};
use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Empty,
    Galaxy,
}

type Coordinates = [i64; 2];

#[derive(Debug)]
struct Cell {
    old_position: Coordinates,
    new_position: Coordinates,
    cell_type: CellType,
}

#[derive(Debug)]
struct Image {
    cells: Vec<Vec<Cell>>,
}

impl Image {
    fn is_row_empty(&self, idx: usize) -> bool {
        self.cells[idx]
            .iter()
            .all(|c| c.cell_type == CellType::Empty)
    }

    fn find_empty_row(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(row_idx, _)| self.is_row_empty(*row_idx))
            .map(|(idx, _)| idx)
            .collect()
    }

    fn is_col_empty(&self, idx: usize) -> bool {
        self.cells
            .iter()
            .all(|row| row[idx].cell_type == CellType::Empty)
    }

    fn find_empty_col(&self) -> Vec<usize> {
        self.cells[0]
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.is_col_empty(*idx))
            .map(|(idx, _)| idx)
            .collect()
    }

    fn expand_rows(&mut self, factor: i64) {
        let empty_rows = self.find_empty_row();
        let galaxies = self.find_galaxies(false);

        for r in empty_rows {
            for galaxy in &galaxies {
                if galaxy[1] > r as i64 {
                    let cell = &mut self.cells[galaxy[1] as usize][galaxy[0] as usize];
                    cell.new_position[1] += factor - 1;
                }
            }
        }
    }

    fn expand_cols(&mut self, factor: i64) {
        let empty_cols = self.find_empty_col();
        let galaxies = self.find_galaxies(false);

        for c in empty_cols {
            for galaxy in &galaxies {
                if galaxy[0] > c as i64 {
                    let cell = &mut self.cells[galaxy[1] as usize][galaxy[0] as usize];
                    cell.new_position[0] += factor - 1;
                }
            }
        }
    }

    fn expand_galaxy(&mut self, factor: i64) {
        self.expand_rows(factor);
        self.expand_cols(factor);
    }

    fn find_galaxies(&self, new: bool) -> Vec<[i64; 2]> {
        let mut galaxies = Vec::new();

        for row in &self.cells {
            for cell in row {
                if cell.cell_type == CellType::Galaxy {
                    if new {
                        galaxies.push(cell.new_position);
                    } else {
                        galaxies.push(cell.old_position);
                    }
                }
            }
        }

        galaxies
    }

    fn find_total_distance(&self) -> i64 {
        let galaxies = &self.find_galaxies(true);

        let mut ans = 0;

        for (i, a) in galaxies.iter().enumerate() {
            for b in galaxies.iter().skip(i + 1) {
                ans += find_distance(a, b);
            }
        }

        ans
    }
}

fn find_distance(a: &Coordinates, b: &Coordinates) -> i64 {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
}

fn parse_input(lines: Lines) -> Image {
    let mut image = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let cell = Cell {
                cell_type: match c {
                    '.' => CellType::Empty,
                    '#' => CellType::Galaxy,
                    _ => unreachable!("Invalid image char"),
                },
                old_position: [x as i64, y as i64],
                new_position: [x as i64, y as i64],
            };

            row.push(cell);
        }

        image.push(row);
    }

    Image { cells: image }
}

fn part_1(lines: Lines) -> i64 {
    let mut image = parse_input(lines);
    image.expand_galaxy(2);
    image.find_total_distance()
}

fn part_2(lines: Lines) -> i64 {
    let mut image = parse_input(lines);
    image.expand_galaxy(1000000);
    image.find_total_distance()
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(11, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}

#[cfg(test)]
mod tests {

    use crate::parse_input;

    static GALAXY: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    static UPDATED_GALAXY: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn check_expension() {
        let mut galaxy = parse_input(GALAXY.lines());
        let u_galaxy = parse_input(UPDATED_GALAXY.lines());

        galaxy.expand_galaxy(2);

        assert_eq!(galaxy.find_galaxies(true), u_galaxy.find_galaxies(false))
    }
}
