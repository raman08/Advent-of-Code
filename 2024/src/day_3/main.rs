use itertools::Itertools;
use regex::Regex;

use aoc_2024::{get_file_path, read_file};

fn part_1(lines: String) -> i32 {
    let re =
        Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").expect("Regex is wrong u dumb idiot!!!!!!!!!!");

    re.captures_iter(&lines)
        .map(|c| {
            let (_, [_, a, b]) = c.extract();

            let (a, b) = (
                a.parse::<i32>().ok().unwrap(),
                b.parse::<i32>().ok().unwrap(),
            );

            a * b
        })
        .sum()
}

fn part_2(lines: String) -> i32 {
    let re = Regex::new(r"(don't\(\)|do\(\))|(mul\((\d{1,3}),(\d{1,3})\))")
        .expect("Regex is wrong u dumb idiot!!!!!!!!!!");

    let captures = re
        .captures_iter(&lines)
        .into_iter()
        .map(|c| {
            let c = c.iter().collect_vec();
            (c[0].unwrap().as_str(), c[3], c[4])
        })
        .collect_vec();

    let mut ans = 0;

    let mut skip = false;
    for (name, a, b) in captures.iter() {
        if name == &"don\'t()" {
            skip = true;
            continue;
        }

        if name == &"do()" {
            skip = false;
            continue;
        }

        if skip {
            continue;
        }

        let a = a
            .expect("a should be number")
            .as_str()
            .parse::<i32>()
            .unwrap();

        let b = b
            .expect("b should be number")
            .as_str()
            .parse::<i32>()
            .unwrap();

        ans += a * b;
    }

    ans
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(3, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.clone()));
    println!("Part 2: {}", part_2(contents.clone()));
}
