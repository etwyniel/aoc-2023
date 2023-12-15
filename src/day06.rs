use aoc_framework::*;

pub struct Day06;

impl_day!(Day06::{part1, part2}: 2023[6], r"
Time:      7  15   30
Distance:  9  40  200
");

#[derive(Debug, Clone, Copy, Default)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn count_ways(self) -> u64 {
        let discr = self.time.pow(2) - 4 * self.dist;
        let discr_root = (discr as f64).sqrt();
        let r1 = (self.time as f64 - discr_root) / 2. + 1e-10;
        let r2 = (self.time as f64 + discr_root) / 2. - 1e-10;
        (r2.floor() as u64 - r1.ceil() as u64) + 1
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<u64>().unwrap_or(0))
        })
        .tuples()
        .flat_map(|(l1, l2)| l1.zip(l2).map(|(time, dist)| Race { time, dist }))
        .collect()
}

#[aoc(part = 1, example = 288, benchmark = 1000)]
fn part1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(Race::count_ways)
        .product()
}

#[aoc(part = 2, example = 71503, benchmark = 1000)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|ln| {
            ln.bytes()
                .filter(|b| b.is_ascii_digit())
                .map(|b| b - b'0')
                .fold(0, |acc, d| acc * 10 + d as u64)
        })
        .tuples()
        .map(|(time, dist)| Race { time, dist }.count_ways())
        .next()
        .unwrap_or_default()
}
