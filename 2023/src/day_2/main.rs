use aoc2023::{get_file_path, read_file};
use regex::Regex;
use std::{cmp, str::Lines};

fn part_1(games: Lines) -> i32 {
    let mut ans = 0;
    for game in games {
        let parts: Vec<&str> = game.split(':').collect();

        let game_number: &str = parts[0].split(" ").collect::<Vec<_>>()[1];
        let game_number: i32 = game_number.parse().unwrap();

        let sets: Vec<&str> = parts[1].split(";").collect::<Vec<&str>>();

        let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
        let mut possible = true;

        sets.clone().into_iter().for_each(|set| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cap in re.captures_iter(set) {
                let num: i32 = cap[1].parse().unwrap();

                match &cap[2] {
                    "red" => red += num,
                    "blue" => blue += num,
                    "green" => green += num,
                    &_ => unreachable!(),
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                possible = false;
            }
        });

        if possible {
            ans += game_number;
        }
    }

    ans
}

fn part_2(games: Lines) -> i32 {
    let mut ans = 0;

    for game in games {
        let parts: Vec<&str> = game.split(':').collect();

        let sets: Vec<&str> = parts[1].split(";").collect::<Vec<&str>>();

        let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();

        let mut mred = 0;
        let mut mgreen = 0;
        let mut mblue = 0;

        sets.clone().into_iter().for_each(|set| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cap in re.captures_iter(set) {
                let num: i32 = cap[1].parse().unwrap();

                match &cap[2] {
                    "red" => red += num,
                    "blue" => blue += num,
                    "green" => green += num,
                    &_ => unreachable!(),
                }
            }

            mred = cmp::max(mred, red);
            mgreen = cmp::max(mgreen, green);
            mblue = cmp::max(mblue, blue);
        });

        ans += mred * mgreen * mblue;
    }

    ans
}

fn main() {
    let is_sample = false;
    let file_path = get_file_path(2, is_sample);
    let contents = read_file(file_path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
