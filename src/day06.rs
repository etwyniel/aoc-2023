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
        (1..self.time)
            .map(|t| t * (self.time - t))
            .filter(|&d| d > self.dist)
            .count() as u64
    }
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Race> {
    let mut out = Vec::new();
    input.for_each(|s| {
        s.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u64>().unwrap_or(0))
            .enumerate()
            .for_each(|(j, n)| {
                if out.len() < j + 1 {
                    out.push(Race { time: n, dist: 0 });
                } else {
                    out[j].dist = n;
                }
            })
    });
    out
}

#[aoc(part = 1, example = 288)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let races = parse_input(input);
    races.into_iter().map(Race::count_ways).product()
}

#[aoc(part = 2, example = 71503)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut it = input.map(|ln| {
        ln.bytes().fold(0, |acc, b| {
            if let b'0'..=b'9' = b {
                acc * 10 + (b - b'0') as u64
            } else {
                acc
            }
        })
    });
    let r = Race {
        time: it.next().unwrap_or_default(),
        dist: it.next().unwrap_or_default(),
    };
    r.count_ways()
}
