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

fn num_winning_numbers(winning: &str, numbers: &str) -> usize {
    let mut winning_mask = [false; 100];
    winning
        .split(' ')
        .flat_map(|s| s.parse::<usize>().ok())
        .for_each(|n| winning_mask[n] = true);
    numbers
        .split(' ')
        .flat_map(|s| s.parse::<usize>().ok())
        .filter(|&n| winning_mask[n])
        .count()
}

#[aoc(part = 1, example = 13)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|line| {
            line.split_once(": ")
                .and_then(|(_, numbers)| numbers.split_once(" | "))
                .map(|(winning, numbers)| {
                    let w = num_winning_numbers(winning, numbers);
                    if w == 0 {
                        0
                    } else {
                        1 << (w - 1)
                    }
                })
        })
        .sum()
}

#[aoc(part = 2, example = 30)]
fn part2(input: Vec<String>) -> u64 {
    let mut counts = vec![1; input.len()];
    input.into_iter().enumerate().for_each(|(i, line)| {
        if let Some((winning, numbers)) = line
            .split_once(": ")
            .and_then(|(_, numbers)| numbers.split_once(" | "))
        {
            let w = num_winning_numbers(winning, numbers);
            let c = counts[i];
            for j in 0..w {
                if let Some(count) = counts.get_mut(i + j + 1) {
                    *count += c;
                }
            }
        };
    });
    counts.into_iter().sum()
}
