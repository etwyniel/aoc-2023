use std::io::BufRead;

use aoc_framework::*;

pub struct Day04;

impl_day!(Day04::{part1, part2}: 2023[4], r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");

fn to_mask(numbers: &str) -> u128 {
    numbers
        .as_bytes()
        .split(|&b| b == b' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.iter().fold(0, |acc, b| acc * 10 + b - b'0'))
        .fold(0u128, |mask, n| mask | (1 << n))
}

fn num_winning_numbers(winning: &str, numbers: &str) -> usize {
    let w = to_mask(winning);
    let nums = to_mask(numbers);
    (w & nums).count_ones() as usize
}

#[aoc(part = 1, example = 13)]
fn part1(mut input: impl BufRead) -> u64 {
    let mut line = String::with_capacity(100);
    let mut sum = 0;
    while let Ok(1..) = input.read_line(&mut line) {
        let card = line.trim_end().splitn(3, ' ').last().unwrap();
        let (winning, numbers) = card.split_once(" | ").unwrap();
        let w = num_winning_numbers(winning, numbers);
        sum += if w == 0 { 0 } else { 1 << (w - 1) };
        line.clear();
    }
    sum
}

#[aoc(part = 2, example = 30)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut counts = vec![1];
    input.enumerate().for_each(|(i, line)| {
        let (winning, numbers) = line
            .split_once(": ")
            .and_then(|(_, numbers)| numbers.split_once(" | "))
            .unwrap_or_default();
        let c = counts.get(i).copied().unwrap_or(1);
        let w = num_winning_numbers(winning, numbers);
        while counts.len() < i + w + 1 {
            counts.push(1);
        }
        counts
            .iter_mut()
            .skip(i + 1)
            .take(w)
            .for_each(|count| *count += c)
    });
    counts.into_iter().sum()
}
