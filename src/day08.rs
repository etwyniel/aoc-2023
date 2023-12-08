use std::collections::HashMap;

use aoc_framework::*;

pub struct Day08;

impl_day!(Day08::{part1, part2}: 2023[8], r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");

fn parse_id(s: &str) -> u64 {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' => b - b'A',
            b'0'..=b'9' => b - b'0' + 26,
            _ => 0,
        } as u64)
        .fold(0, |acc, n| acc * 36 + n)
}

fn parse_map(input: impl Iterator<Item = String>) -> HashMap<u64, [u64; 2]> {
    input
        .flat_map(|ln| {
            let (src, dsts) = ln.split_once(" = ")?;
            let src = parse_id(src);
            let (l, r) = dsts
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .map(parse_id)
                .tuples()
                .next()?;
            Some((src, [l, r]))
        })
        .collect()
}

fn count_steps<F: Fn(u64) -> bool>(
    map: &HashMap<u64, [u64; 2]>,
    directions: &[u8],
    src: u64,
    is_end: F,
) -> u64 {
    let mut pos = src;
    let mut i = 0;
    while !is_end(pos) {
        let dir = directions[i % directions.len()] as usize;
        pos = map[&pos][dir];
        i += 1;
    }
    i as u64
}

#[aoc(part = 1)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let mut directions = input.next().unwrap().into_bytes();
    directions
        .iter_mut()
        .for_each(|b| if *b == b'L' { *b = 0 } else { *b = 1 });
    input.next();
    let map = parse_map(input);
    let src = parse_id("AAA");
    let dst = parse_id("ZZZ");
    count_steps(&map, &directions, src, |p| p == dst)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

#[aoc(part = 2, example = 6)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let mut directions = input.next().unwrap().into_bytes();
    directions
        .iter_mut()
        .for_each(|b| if *b == b'L' { *b = 0 } else { *b = 1 });
    input.next();
    let map = parse_map(input);
    map.keys()
        .filter(|&p| p % 36 == 0)
        .map(|&src| count_steps(&map, &directions, src, |p| p % 36 == 25))
        .fold(0, |acc, n| if acc == 0 { n } else { lcm(acc, n) })
}
