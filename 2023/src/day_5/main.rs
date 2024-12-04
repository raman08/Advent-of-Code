use aoc2023::{get_file_path, read_file};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Mappings {
    dest: i64,
    src: i64,
    range: i64,
}

impl Mappings {
    fn in_src(&self, a: i64) -> bool {
        self.src <= a && a < (self.src + self.range)
    }

    fn get_val(&self, a: i64) -> i64 {
        if self.src <= a && a < (self.src + self.range) {
            return a - self.src + self.dest;
        }

        a
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn parse_sections(line: &str) -> Vec<Mappings> {
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    re.captures_iter(line)
        .filter_map(|c| {
            let nums = (1..=3)
                .filter_map(|i| c.get(i).and_then(|m| m.as_str().parse::<i64>().ok()))
                .collect::<Vec<_>>();

            if nums.len() == 3 {
                Some(Mappings {
                    src: nums[1],
                    dest: nums[0],
                    range: nums[2],
                })
            } else {
                None
            }
        })
        .collect()
}

fn part_1(lines: String) -> i64 {
    let mut sections = lines.split("\n\n");

    let seeds = parse_seeds(sections.next().unwrap());

    let maps: Vec<_> = sections.map(|s| parse_sections(s)).collect();

    let mut ans = i64::MAX;

    for mut seed in seeds {
        for (idx, map) in maps.iter().enumerate() {
            let val = match map.iter().find(|m| m.in_src(seed)) {
                Some(m) => m.get_val(seed),
                None => seed,
            };

            if idx == maps.len() - 1 {
                ans = ans.min(val);
            }
            seed = val;
        }
    }

    ans
}

fn part_2(lines: String) -> i64 {
    let mut sections = lines.split("\n\n");

    let seeds = parse_seeds(sections.next().unwrap());

    let maps: Vec<_> = sections.map(|s| parse_sections(s)).collect();

    let mut ans = i64::MAX;

    for s in seeds.chunks(2).into_iter() {
        println!("Processing {:?}", s);

        for mut seed in s[0]..s[0] + s[1] {
            for (idx, map) in maps.iter().enumerate() {
                let val = match map.iter().find(|m| m.in_src(seed)) {
                    Some(m) => m.get_val(seed),
                    None => seed,
                };

                if idx == maps.len() - 1 {
                    ans = ans.min(val);
                }
                seed = val;
            }
        }
    }

    ans
}

fn main() {
    let is_sample = false;
    let path = get_file_path(5, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.clone()));
    println!("Part 2: {}", part_2(contents.clone()));
}
