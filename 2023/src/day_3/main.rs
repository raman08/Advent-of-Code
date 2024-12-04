use aoc2023::{get_file_path, read_file};
use regex::Regex;
use std::str::Lines;
use std::collections::HashMap;

fn get_box_corrd(start: i32, end: i32, row: i32) -> Vec<(i32, i32)> {
    let mut r = Vec::new();

    r.push((row - 1, start - 1));
    r.push((row - 1, start));
    r.push((row, start - 1));
    r.push((row + 1, start - 1));
    r.push((row + 1, start));

    for p in start + 1..end {
        r.push((row - 1, p));
        r.push((row + 1, p));
    }

    r.push((row - 1, end));
    r.push((row - 1, end + 1));
    r.push((row, end + 1));
    r.push((row + 1, end + 1));
    r.push((row + 1, end));

    r
}

fn part_1(contents: Lines) -> i32 {
    let lines: Vec<Vec<char>> = contents
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let re = Regex::new(r"\d+").unwrap();

    let mut ans = 0;

    for (row, line) in lines.iter().enumerate() {
        let row = row as i32;
        let temp_line: String = line.iter().collect();

        let mats = re.find_iter(&temp_line);
        for mat in mats {
            let num: i32 = mat.as_str().parse().unwrap();
            let mut already_added = false;
            let start = mat.start() as i32;
            let end = (mat.end() as i32) - 1;

            let bounding_box = get_box_corrd(start, end, row);

            for (y, x) in bounding_box {
                if y < 0 || x < 0 || y >= lines.len() as i32 || x >= lines.len() as i32 {
                    continue;
                }

                let ch = lines[y as usize][x as usize];

                if ch != '.' && !ch.is_numeric() && !already_added {
                    ans += num;
                    already_added = true;
                }
            }
        }
    }
    ans
}

fn part_2(contents: Lines) -> i32 {
    let lines: Vec<Vec<char>> = contents
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let re = Regex::new(r"\d+").unwrap();

    let mut ans = 0;

    let mut mp: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        let row = row as i32;
        let temp_line: String = line.iter().collect();

        let mats = re.find_iter(&temp_line);

        for mat in mats {
            let num: i32 = mat.as_str().parse().unwrap();
            let mut already_added = false;
            let start = mat.start() as i32;
            let end = (mat.end() as i32) - 1;

            let bounding_box = get_box_corrd(start, end, row);

            for (y, x) in bounding_box {
                if y < 0 || x < 0 || y >= lines.len() as i32 || x >= lines.len() as i32 {
                    continue;
                }

                let ch = lines[y as usize][x as usize];

                if ch == '*' && !already_added {
                    mp.entry((y, x)).or_insert_with(Vec::new).push(num);
                    already_added = true;
                }
            }
        }
    }

    for val in mp.values() {
        if val.len() == 2 {
            ans += val[0] * val[1];
        }
    }

    ans
}

fn main() {
    let is_sample = false;
    let path = get_file_path(3, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
