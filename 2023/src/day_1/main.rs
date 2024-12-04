use aoc2023::{get_file_path, read_file};
use std::str::Lines;

fn find_digit(line: String) -> i32 {
    let mut digits = line.trim().chars().filter(|c| c.is_numeric());

    let first = digits.next();
    let last = digits.last();

    let mut s = String::new();

    if first.is_some() && last.is_some() {
        s.push(first.unwrap());
        s.push(last.unwrap());
    } else {
        s.push(first.unwrap());
        s.push(first.unwrap());
    }

    s.parse::<i32>().unwrap()
}

fn part_1(contents: Lines) -> i32 {
    let mut ans = 0;

    for line in contents {
        ans += find_digit(line.to_string());
    }

    ans
}

fn part_2(contents: Lines) -> i32 {
    let mut ans = 0;

    for line in contents {
        let new_line = line
            .replace("nine", "n9e")
            .replace("eight", "e8t")
            .replace("seven", "s7n")
            .replace("six", "s6x")
            .replace("five", "f5e")
            .replace("four", "f4r")
            .replace("three", "t3e")
            .replace("two", "t2o")
            .replace("one", "o1e");

        ans += find_digit(new_line.to_string());
    }

    ans
}

fn main() {
    let is_sample = false;
    let file_path = get_file_path(1, is_sample);
    let contents = read_file(file_path.to_string());

    let part_1_ans = part_1(contents.lines());
    let part_2_ans = part_2(contents.lines());

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {}", part_2_ans);
}
