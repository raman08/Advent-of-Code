use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};
use std::{
    collections::HashMap,
    str::Lines,
    sync::{Arc, Mutex},
};

fn rec<'a>(
    springs: &'a [char],
    damange_count: &[usize],
    damange_in_group: usize,
    mp: &mut HashMap<(Vec<char>, Vec<usize>, usize), usize>,
) -> usize {
    if let Some(x) = mp.get(&(springs.to_vec(), damange_count.to_vec(), damange_in_group)) {
        return *x;
    }

    let ans = rec_mem(&springs, &damange_count, damange_in_group, mp);

    mp.insert(
        (springs.to_vec(), damange_count.to_vec(), damange_in_group),
        ans,
    );
    ans
}

fn rec_mem(
    springs: &[char],
    damange_count: &[usize],
    damange_in_group: usize,
    mp: &mut HashMap<(Vec<char>, Vec<usize>, usize), usize>,
) -> usize {
    if springs.is_empty() {
        if damange_count.is_empty() && damange_in_group == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let damaged = springs.iter().filter(|c| c == &&'#').count();
    let total_danage_count = damange_count.iter().sum();

    if damaged > total_danage_count {
        return 0;
    }

    let ans = match springs[0] {
        '?' => ['.', '#'].iter().fold(0, |acc, c| {
            let mut new_spring: Vec<char> = vec![*c];
            new_spring.append(&mut springs[1..].to_vec());
            acc + rec(&new_spring, &damange_count, damange_in_group, mp)
        }),
        '.' => {
            let mut c = 0;
            if damange_count.len() > 0 && damange_in_group == damange_count[0] {
                c = rec(&springs[1..], &damange_count[1..], 0, mp);
            }

            if damange_in_group == 0 {
                c += rec(&springs[1..], &damange_count, 0, mp);
            }
            c
        }
        '#' => rec(&springs[1..], &damange_count, damange_in_group + 1, mp),

        _ => unreachable!("Not a valid spring"),
    };

    ans
}

fn part_1(lines: Lines) -> i32 {
    let mut mp = HashMap::new();
    lines
        .map(|line| line.split_once(" ").unwrap())
        .map(|(springs, correctness)| {
            let mut springs = springs.chars().collect_vec();
            let correctness = correctness
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            springs.push('.');
            (springs, correctness)
        })
        .map(|(springs, damage_count)| rec(&springs[..], &damage_count[..], 0, &mut mp))
        .fold(0, |acc, v| {
            // println!("{}", v);
            acc + v as i32
        })
}

fn part_2(lines: String) -> usize {
    let mp = Arc::new(Mutex::new(HashMap::new()));

    let result: usize = lines
        .par_lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(springs, correctness)| {
            let mut sp = springs.to_string();
            let mut co = correctness.to_string();

            for _ in 0..4 {
                sp.push('?');
                sp.push_str(&springs);
                co.push(',');
                co.push_str(&correctness);
            }
            (sp, co)
        })
        .map(|(springs, correctness)| {
            let mut springs = springs.chars().collect_vec();
            let correctness = correctness
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            springs.push('.');
            (springs, correctness)
        })
        .map(|(springs, damage_count)| {
            let mut mp = mp.lock().unwrap();
            rec(&springs[..], &damage_count[..], 0, &mut mp)
        })
        .sum();

    result
}
fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(12, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents));
}
