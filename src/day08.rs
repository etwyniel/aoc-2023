use std::{array, collections::HashMap};

use aoc_framework::*;

pub struct Day08;

impl_day!(Day08::{part1, part2}: 2023[8], r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
r"
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

#[derive(Clone, Copy, Default)]
struct Id {
    id: u32,
    ndx: u32,
}

#[derive(Clone, Copy, Default)]
struct MapEntry {
    id: Id,
    dests: [Id; 2],
}

fn parse_id(s: &str) -> u32 {
    s.bytes()
        .flat_map(|b| {
            Some(match b {
                b'A'..=b'Z' => b - b'A',
                b'0'..=b'9' => b - b'0' + 26,
                _ => return None,
            } as u32)
        })
        .fold(0, |acc, n| acc * 36 + n)
}

fn parse_map(input: impl Iterator<Item = String>) -> Vec<MapEntry> {
    let map = input
        .enumerate()
        .flat_map(|(i, ln)| {
            let (src, dsts) = ln.split_once(" = ")?;
            let src = parse_id(src);
            let (l, r) = dsts.split(", ").map(parse_id).tuples().next()?;
            Some((src, (i, [l, r])))
        })
        .collect::<HashMap<_, _>>();
    let mut simplified_map = vec![MapEntry::default(); map.len()];
    map.iter().enumerate().for_each(|(i, (k, &(pos, dests)))| {
        simplified_map[pos] = MapEntry {
            id: Id {
                id: *k,
                ndx: i as u32,
            },
            dests: array::from_fn(|i| Id {
                id: dests[i],
                ndx: map[&dests[i]].0 as u32,
            }),
        };
    });
    simplified_map
}

fn count_steps<F: Fn(u32) -> bool>(
    map: &[MapEntry],
    directions: &[u8],
    src: usize,
    is_end: F,
) -> u64 {
    let mut pos = 0;
    let mut ndx = src;
    let mut i = 0;
    while !is_end(pos) {
        let dir = directions[i % directions.len()] as usize;
        let entry = map[ndx].dests[dir];
        pos = entry.id;
        ndx = entry.ndx as usize;
        i += 1;
    }
    i as u64
}

#[aoc(part = 1, example = 2)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let mut directions = input.next().unwrap().into_bytes();
    directions
        .iter_mut()
        .for_each(|b| if *b == b'L' { *b = 0 } else { *b = 1 });
    input.next();
    let map = parse_map(input);
    let src_pos = parse_id("AAA");
    let dst = parse_id("ZZZ");
    let src_ndx = map.iter().position(|entry| entry.id.id == src_pos).unwrap();
    count_steps(&map, &directions, src_ndx, |p| p == dst)
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
    map.iter()
        .enumerate()
        .filter(|(_, &p)| p.id.id % 36 == 0)
        .map(|(src, _)| count_steps(&map, &directions, src, |p| p % 36 == 25))
        .fold(0, |acc, n| if acc == 0 { n } else { lcm(acc, n) })
}
