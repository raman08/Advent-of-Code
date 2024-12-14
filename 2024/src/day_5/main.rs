use std::{cmp::Ordering, collections::HashSet};

use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;

fn part_1(lines: &String) -> i32 {
    let (ordering, pages) = lines
        .split_once("\n\n")
        .expect("Failed to split the input!!!!!!");

    let ordering: HashSet<(usize, usize)> = ordering
        .lines()
        .map(|line| {
            let (first, second) = line.split_once("|").expect("Failed to split at ordering");

            (
                first.parse::<usize>().expect("failed to parse").into(),
                second.parse::<usize>().expect("failed to parse").into(),
            )
        })
        .collect();

    pages.lines().fold(0, |acc, page| {
        let page = page
            .split(',')
            .map(|x| x.parse::<usize>().expect("Failed to parse {x}"))
            .collect_vec();

        if page.is_sorted_by(|a, b| !(ordering.contains(&(*b, *a)))) {
            acc + (page[page.len() / 2] as i32)
        } else {
            acc
        }
    })
}

fn part_2(lines: &String) -> i32 {
    let (ordering, pages) = lines
        .split_once("\n\n")
        .expect("Failed to split the input!!!!!!");

    let ordering: HashSet<(usize, usize)> = ordering
        .lines()
        .map(|line| {
            let (first, second) = line.split_once("|").expect("Failed to split at ordering");

            (
                first.parse::<usize>().expect("failed to parse").into(),
                second.parse::<usize>().expect("failed to parse").into(),
            )
        })
        .collect();

    let compare = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if ordering.contains(&(x, y)) {
            Ordering::Less
        } else if ordering.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    pages.lines().fold(0, |acc, page| {
        let mut page = page
            .split(',')
            .map(|x| x.parse::<usize>().expect("Failed to parse {x}"))
            .collect_vec();

        if page.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
            acc
        } else {
            page.sort_by(compare);

            acc + (page[page.len() / 2] as i32)
        }
    })
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(5, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
