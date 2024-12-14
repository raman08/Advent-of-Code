use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;

fn part_1(lines: &String) -> i32 {
    let search_string = "XMAS";
    let reverse_search_string = "SAMX";

    let lines = lines.lines().collect_vec();
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut ans = 0;

    // Horizontal
    for line in &lines {
        for idx in 0..(line.len() - 3) {
            let s = &line[idx..(idx + 4)];
            if s == search_string || s == reverse_search_string {
                ans += 1;
            }
        }
    }

    // Vertical
    for row in 0..(height - 3) {
        for col in 0..(width) {
            let mut s = String::new();

            for i in 0..4 {
                let ch = lines[row + i].chars().nth(col).unwrap();
                s.push(ch);
            }

            if s == search_string || s == reverse_search_string {
                ans += 1;
            }
        }
    }

    // Digonal RIght
    for row in 0..(height - 3) {
        for col in 0..(width - 3) {
            let mut s = String::new();

            for i in 0..4 {
                let ch = lines[row + i].chars().nth(col + i).unwrap();
                s.push(ch);
            }

            if s == search_string || s == reverse_search_string {
                ans += 1;
            }
        }
    }

    // Digona Left
    for row in 0..(height - 3) {
        for col in 3..width {
            let mut s = String::new();

            for i in 0..4 {
                let ch = lines[row + i].chars().nth(col - i).unwrap();
                s.push(ch);
            }

            if s == search_string || s == reverse_search_string {
                ans += 1;
            }
        }
    }

    ans
}

fn part_2(lines: &String) -> i32 {
    let lines = lines.lines().collect_vec();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let mut ans = 0;

    for row in 0..height {
        for col in 0..width {
            if row <= 0 || col <= 0 || row >= height - 1 || col >= width - 1 {
                continue;
            }

            let ch = lines[row]
                .chars()
                .nth(col)
                .expect("Out of bound, {row}, {col}");

            if ch != 'A' {
                continue;
            }

            let a = lines[row - 1]
                .chars()
                .nth(col - 1)
                .expect("Out of bound, {row}, {col}");

            let b = lines[row - 1]
                .chars()
                .nth(col + 1)
                .expect("Out of bound, {row}, {col}");

            let d = lines[row + 1]
                .chars()
                .nth(col - 1)
                .expect("Out of bound, {row}, {col}");

            let c = lines[row + 1]
                .chars()
                .nth(col + 1)
                .expect("Out of bound, {row}, {col}");

            if ((a == 'M' && c == 'S') && (b == 'S' && d == 'M'))
                || ((a == 'M' && c == 'S') && (b == 'M' && d == 'S'))
                || ((a == 'S' && c == 'M') && (b == 'S' && d == 'M'))
                || ((a == 'S' && c == 'M') && (b == 'M' && d == 'S'))
            {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(4, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
