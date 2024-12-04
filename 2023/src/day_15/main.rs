use std::{collections::HashMap, fmt::Debug};

use aoc2023::{get_file_path, read_file};
use itertools::Itertools;

#[derive(Clone)]
struct Step {
    label: String,
    hash: usize,
    sigh: char,
    focal_length: Option<usize>,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Step")
            .field("label", &self.label)
            .field("hash", &self.hash)
            .field("focal_length", &self.focal_length)
            .finish()
    }
}

fn hasher(seq: String) -> usize {
    seq.to_string()
        .as_bytes()
        .iter()
        .fold(0, |acc, s| ((acc + *s as usize) * 17) % 256)
}

fn seq_decoder(seq: String) -> Step {
    if seq.contains("=") {
        let ss = seq.split("=").collect_vec();
        assert_eq!(ss.len(), 2);
        return Step {
            label: ss[0].to_string(),
            hash: hasher(ss[0].to_string()),
            focal_length: Some(
                ss[1]
                    .to_string()
                    .parse::<usize>()
                    .expect("This should be a number"),
            ),
            sigh: '=',
        };
    } else if seq.contains("-") {
        let ss = seq.split("-").collect_vec();

        assert_eq!(ss.len(), 2);

        return Step {
            label: ss[0].to_string(),
            hash: hasher(ss[0].to_string()),
            focal_length: None,
            sigh: '-',
        };
    }

    unreachable!("This should not happen!!!!!!!!!!");
}

fn part_1(line: &String) -> usize {
    line.trim()
        .split(',')
        .map(|s| s.to_string())
        .map(hasher)
        .sum()
}

fn part_2(line: &String) -> usize {
    let sequences = line
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .map(seq_decoder)
        .collect_vec();

    let mut boxes = HashMap::new();

    for seq in &sequences {
        let current_box = boxes.entry(seq.hash).or_insert(Vec::new());
        if seq.sigh == '-' {
            if let Some(position) = current_box.iter().position(|s| s == seq) {
                current_box.remove(position);
            }
        } else if seq.sigh == '=' {
            if let Some(position) = current_box.iter().position(|s| s == seq) {
                *current_box
                    .get_mut(position)
                    .expect("!! Index out of bound") = seq.clone();
            } else {
                current_box.push(seq.clone());
            }
        } else {
            unreachable!("This shoud not happen");
        }
    }

    boxes
        .iter()
        .map(|(idx, b)| {
            b.iter()
                .enumerate()
                .map(|(i, l)| {
                    return ((idx + 1)
                        * (i + 1)
                        * l.focal_length.expect("Focal Len should be there"))
                        as usize;
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    #[allow(unused_variables)]
    let is_sample = true;
    let is_sample = false;
    let path = get_file_path(15, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[cfg(test)]
mod tests {
    use crate::{hasher, seq_decoder, Step};

    #[test]
    fn check_hash() {
        assert_eq!(hasher("HASH".to_string()), 52)
    }

    #[test]
    fn check_seq_decoder() {
        assert_eq!(
            seq_decoder("rn=1".to_string()),
            Step {
                label: "rn".to_string(),
                hash: hasher("rn=1".to_string()),
                sigh: '=',
                focal_length: Some(1)
            }
        );
    }

    #[test]
    fn check_seq_decoder_2() {
        assert_eq!(
            seq_decoder("cm-".to_string()),
            Step {
                label: "cm".to_string(),
                hash: hasher("cm-".to_string()),
                sigh: '-',
                focal_length: None
            }
        )
    }
}
