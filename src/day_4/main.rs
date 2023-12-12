use aoc2023::{get_file_path, read_file};
use std::collections::{HashMap, HashSet};
use std::str::Lines;

fn get_intersections(line: &str) -> usize {
    let nums = line.split(": ").collect::<Vec<_>>()[1].trim();
    let nums: Vec<&str> = nums.split("|").collect();
    let winners: HashSet<_> = nums[0]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let my_nums: HashSet<_> = nums[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    winners.intersection(&my_nums).collect::<Vec<_>>().len()
}

fn part_1(lines: Lines) -> i32 {
    lines
        .into_iter()
        .map(|line| match get_intersections(line) {
            0 => 0,
            x => 2i32.pow((x - 1) as u32),
        })
        .fold(0, |acc, x| acc + x)
}

fn part_2(lines: Lines) -> i32 {
    let mut ans = 0;

    let mut mp: HashMap<i32, i32> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let intersection: i32 = get_intersections(line).try_into().unwrap();
        let card_no = i + 1;
        let card_no = card_no.try_into().unwrap();

        let count = mp.get(&card_no).unwrap_or(&0) + 1;
        ans += count;

        if intersection > 0 {
            for num in card_no + 1..=card_no + intersection {
                mp.insert(num, mp.get(&num).unwrap_or(&0) + count);
            }
        }
    }

    ans
}

fn main() {
    let is_sample = false;
    let path = get_file_path(4, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
