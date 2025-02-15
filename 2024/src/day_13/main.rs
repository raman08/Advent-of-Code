use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};
use regex::Regex;
use std::{env, ops::Mul};

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: u64,
    y: u64,
}

impl Pos {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl Machine {
    fn new(a: Pos, b: Pos, prize: Pos) -> Self {
        Self { a, b, prize }
    }
}

fn parse(lines: &str) -> Vec<Machine> {
    let chunk = lines.split("\n\n").collect_vec();

    chunk
        .iter()
        .map(|c| {
            let c = c.trim();
            let acc = Vec::new();
            let poss = c.split("\n").fold(acc, |mut acc, l| {
                let reg = Regex::new(r"X[+=](?P<x>\d+), Y[+=](?<y>\d+)").expect("Invalid regex");
                let res = reg.captures(l).expect("ddd");

                let x = res
                    .name("x")
                    .expect("Invalid x for button")
                    .as_str()
                    .parse::<u64>()
                    .expect("Converting x to parse faild");
                let y = res
                    .name("y")
                    .expect("Invalid y for button")
                    .as_str()
                    .parse::<u64>()
                    .expect("Converting y to parse faild");

                let pos = Pos::new(x, y);

                acc.push(pos);

                acc
            });

            assert_eq!(poss.len(), 3, "Len not equal");

            Machine::new(poss[0], poss[1], poss[2])
        })
        .collect_vec()
}

fn check_simulation(a: Pos, b: Pos, prize: Pos, t: (u64, u64)) -> bool {
    let lhs1 = a.x.mul(t.0) + b.x.mul(t.1);
    let lhs2 = a.y.mul(t.0) + b.y.mul(t.1);

    (lhs1 == prize.x) && (lhs2 == prize.y)
}

fn part_1(lines: &str) -> u64 {
    let machines = parse(lines);

    machines
        .iter()
        .map(|machine| {
            let a = Matrix2::new(
                machine.a.x as f64,
                machine.b.x as f64,
                machine.a.y as f64,
                machine.b.y as f64,
            );

            let b = Matrix2x1::new(machine.prize.x as f64, machine.prize.y as f64);

            let ans = a.try_inverse().expect("Failed to inverse matrix") * b;

            if ans[0] > 100.0 || ans[1] > 100.0 {
                // println!("{:?}", ans);
                return 0;
            }

            if ans[0] < 0.0 || ans[1] < 00.0 {
                // println!("{:?}", ans);
                return 0;
            }

            let ans_round = (ans[0].round() as u64, ans[1].round() as u64);

            if check_simulation(
                machine.a,
                machine.b,
                machine.prize,
                (ans_round.0, ans_round.1),
            ) {
                return ans_round.0.mul(3) + ans_round.1.mul(1);
            }

            0
        })
        .sum()
}

fn part_2(lines: &str) -> u64 {
    let machines = parse(lines);

    machines
        .iter()
        .map(|machine| {
            let a = Matrix2::new(
                machine.a.x as f64,
                machine.b.x as f64,
                machine.a.y as f64,
                machine.b.y as f64,
            );

            let multiplier = 10000000000000;
            let b = Matrix2x1::new(
                (machine.prize.x + multiplier) as f64,
                (machine.prize.y + multiplier) as f64,
            );

            let ans = a.try_inverse().expect("Failed to inverse matrix") * b;

            if ans[0] < 0.0 || ans[1] < 00.0 {
                return 0;
            }

            let ans_round = (ans[0].round() as u64, ans[1].round() as u64);

            if check_simulation(
                machine.a,
                machine.b,
                Pos::new(machine.prize.x + multiplier, machine.prize.y + multiplier),
                (ans_round.0, ans_round.1),
            ) {
                return ans_round.0.mul(3) + ans_round.1.mul(1);
            }

            0
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(13, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
