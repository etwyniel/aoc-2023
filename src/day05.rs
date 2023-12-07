use std::collections::VecDeque;

use aoc_framework::*;

pub struct Day05;

impl_day!(Day05::{part1, part2}: 2023[5], r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
");

#[derive(Clone, Copy, Debug)]
struct SeedRange {
    src: usize,
    len: usize,
}

#[derive(Clone, Copy, Debug)]
struct Range {
    src: usize,
    dst: usize,
    len: usize,
}

impl Range {
    fn map(self, n: usize) -> Option<usize> {
        if n < self.src {
            return None;
        }
        let m = n - self.src;
        if m > self.len {
            return None;
        }
        Some(m + self.dst)
    }

    fn map_range(self, seeds: SeedRange, stack: &mut VecDeque<SeedRange>) -> Option<SeedRange> {
        let end = seeds.src + seeds.len;
        if end < self.src {
            stack.push_back(seeds);
            return None;
        }
        let range_end = self.src + self.len;
        if seeds.src > range_end {
            stack.push_back(seeds);
            return None;
        }
        let m = seeds.src as isize - self.src as isize;
        let (m, len) = if m < 0 {
            stack.push_back(SeedRange {
                src: seeds.src,
                len: (-m) as usize,
            });
            (0, (seeds.len as isize + m) as usize)
        } else {
            (m as usize, seeds.len)
        };
        let len = if end > range_end {
            stack.push_back(SeedRange {
                src: range_end,
                len: end - range_end,
            });
            len - (end - range_end)
        } else {
            len
        };
        Some(SeedRange {
            src: m + self.dst,
            len,
        })
    }
}

struct RangeList(Vec<Range>);

impl RangeList {
    fn map(&self, n: usize) -> usize {
        self.0.iter().find_map(|range| range.map(n)).unwrap_or(n)
    }

    fn map_range(&self, seeds: SeedRange) -> Vec<SeedRange> {
        let mut stack = VecDeque::new();
        let mut out = Vec::new();
        stack.push_back(seeds);
        for range in &self.0 {
            if stack.is_empty() {
                break;
            }
            let n = stack.len();
            for _ in 0..n {
                let seeds = stack.pop_front().unwrap();
                if let Some(mapped) = range.map_range(seeds, &mut stack) {
                    out.push(mapped);
                }
            }
        }
        out.extend(stack);
        out
    }
}

fn parse_range(line: &str) -> Range {
    let (dst, src, len) = line
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .tuples()
        .next()
        .unwrap();
    Range { src, dst, len }
}

#[aoc(part = 1, example = 35)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let seeds = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    input.next();
    let mut range_lists = Vec::new();
    while let Some(_) = input.next() {
        let list = (&mut input)
            .take_while(|ln| !ln.is_empty())
            .map(|ln| parse_range(&ln))
            .collect::<Vec<_>>();
        range_lists.push(RangeList(list));
    }
    seeds
        .into_iter()
        .map(|seed| range_lists.iter().fold(seed, |n, lst| lst.map(n)))
        .min()
        .unwrap() as u64
}

#[aoc(part = 2, example = 46)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let seeds = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .tuples()
        .map(|(src, len)| SeedRange { src, len })
        .collect_vec();
    input.next();
    let mut range_lists = Vec::new();
    while let Some(_) = input.next() {
        let list = (&mut input)
            .take_while(|ln| !ln.is_empty())
            .map(|ln| parse_range(&ln))
            .collect::<Vec<_>>();
        range_lists.push(RangeList(list));
    }
    seeds
        .into_iter()
        .flat_map(|seed_range| {
            range_lists.iter().fold(vec![seed_range], |ranges, lst| {
                let mut out = Vec::new();
                for range in ranges {
                    out.extend(&lst.map_range(range));
                }
                out
            })
        })
        .min_by(|rng1, rng2| rng1.src.cmp(&rng2.src))
        .unwrap()
        .src as u64
}
