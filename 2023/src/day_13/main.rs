use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use std::str::Lines;

type Mirror = Vec<Vec<char>>;

// fn find_eq_cols(mirror: &Mirror) -> Vec<usize> {
//     let mut ret = Vec::new();

//     for col_i in 0..mirror[0].len() - 1 {
//         if mirror.iter().all(|row| row[col_i] == row[col_i + 1]) {
//             ret.push(col_i);
//         }
//     }

//     ret
// }

// fn find_eq_rows(mirror: &Mirror) -> Vec<usize> {
//     let mut ret = Vec::new();

//     for row_i in 0..mirror.len() - 1 {
//         if mirror[row_i] == mirror[row_i + 1] {
//             ret.push(row_i);
//         }
//     }

//     ret
// }

// fn is_reflection_vertical(mirror: &Mirror, col: usize) -> bool {
//     if col == 0 || col == mirror[0].len() - 2 {
//         return true;
//     }

//     let mut left = col - 1;
//     let mut right = col + 2;

//     loop {
//         if mirror.iter().any(|row| row[left] != row[right]) {
//             return false;
//         }

//         if left == 0 || right >= mirror[0].len() - 2 {
//             return true;
//         }

//         left -= 1;
//         right += 1;
//     }
// }

// fn is_reflection_horizontal(mirror: &Mirror, row: usize) -> bool {
//     if row == 0 || row == mirror.len() - 2 {
//         return true;
//     }

//     let mut top = row - 1;
//     let mut bottom = row + 2;

//     loop {
//         if mirror[top] != mirror[bottom] {
//             return false;
//         }

//         if top == 0 || bottom >= mirror.len() - 2 {
//             return true;
//         }

//         top -= 1;
//         bottom += 1;
//     }
// }

// fn find_vertical_reflection(mirror: &Mirror) -> Option<usize> {
//     let cols = find_eq_cols(mirror);

//     for col in cols {
//         if is_reflection_vertical(mirror, col) {
//             return Some(col);
//         }
//     }

//     None
// }

// fn find_horizontal_reflection(mirror: &Mirror) -> Option<usize> {
//     let rows = find_eq_rows(mirror);

//     for col in rows {
//         if is_reflection_horizontal(mirror, col) {
//             return Some(col);
//         }
//     }

//     None
// }

// fn find_reflection(mirror: &Mirror) -> i64 {
//     if let Some(col) = find_vertical_reflection(mirror) {
//         return col as i64 + 1;
//     }

//     if let Some(row) = find_horizontal_reflection(mirror) {
//         return (row as i64 + 1) * 100;
//     }

//     unreachable!("No reflection found");
// }
type PatternType = Vec<Vec<char>>;
type InputType = Vec<PatternType>;
type SolutionType = u32;
fn find_equal_columns(pattern: &PatternType) -> Vec<usize> {
    let mut equal_columns = Vec::new();
    for col_num in 0..pattern[0].len() - 1 {
        if pattern.iter().all(|row| row[col_num] == row[col_num + 1]) {
            equal_columns.push(col_num);
        }
    }
    equal_columns
}

fn find_equal_rows(pattern: &PatternType) -> Vec<usize> {
    let mut equal_rows = Vec::new();
    for row_num in 0..pattern.len() - 1 {
        if pattern[row_num] == pattern[row_num + 1] {
            equal_rows.push(row_num);
        }
    }
    equal_rows
}

fn is_reflection_vertical(pattern: &PatternType, col: usize) -> bool {
    if col == 0 || col >= pattern[0].len() - 2 {
        return true;
    }
    let mut left_col = col - 1;
    let mut right_col = col + 2;
    loop {
        if pattern.iter().any(|row| row[left_col] != row[right_col]) {
            return false;
        }
        if left_col == 0 || right_col >= pattern[0].len() - 2 {
            return true;
        }
        left_col -= 1;
        right_col += 1;
    }
}

fn is_reflection_horizontal(pattern: &PatternType, row: usize) -> bool {
    if row == 0 || row >= pattern.len() - 2 {
        return true;
    }
    let mut top_row = row - 1;
    let mut bottom_row = row + 2;
    loop {
        if pattern[top_row] != pattern[bottom_row] {
            return false;
        }
        if top_row == 0 || bottom_row >= pattern.len() - 2 {
            return true;
        }
        top_row -= 1;
        bottom_row += 1;
    }
}

fn find_vertical_reflection(pattern: &PatternType) -> Option<usize> {
    let equal_columns = find_equal_columns(pattern);
    for col in equal_columns {
        if is_reflection_vertical(pattern, col) {
            return Some(col);
        }
    }
    None
}

fn find_horizontal_reflection(pattern: &PatternType) -> Option<usize> {
    let equal_rows = find_equal_rows(pattern);
    for row in equal_rows {
        if is_reflection_horizontal(pattern, row) {
            return Some(row);
        }
    }
    None
}

fn summarize_pattern(pattern: &PatternType) -> SolutionType {
    if let Some(col) = find_vertical_reflection(pattern) {
        return (col + 1) as SolutionType;
    }
    if let Some(row) = find_horizontal_reflection(pattern) {
        return (row + 1) as SolutionType * 100;
    }
    panic!("No reflection found");
}

fn parse_input(input: String) -> Vec<Mirror> {
    input.split("\n\n").fold(Vec::new(), |mut acc, mirror| {
        let mirror: Mirror = mirror
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        acc.push(mirror);

        acc
    })
}

fn part_1(lines: String) -> u32 {
    let mirrors = parse_input(lines);

    let reflections = mirrors
        .iter()
        .map(|mirror| summarize_pattern(mirror))
        .collect_vec();

    debug_assert_eq!(&mirrors.len(), &reflections.len());

    reflections.iter().sum()
}

fn part_2(lines: Lines) -> i32 {
    0
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(13, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.clone()));
    println!("Part 2: {}", part_2(contents.lines()));
}
