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
    let mut mask = 0u128;
    let mut it = numbers.bytes();
    let mut num = 0;
    loop {
        match it.next() {
            None => {
                mask |= 1 << num;
                break;
            }
            Some(b' ') if num != 0 => {
                mask |= 1 << num;
                num = 0;
            }
            Some(b @ b'0'..=b'9') => num = num * 10 + (b - b'0'),
            _ => {}
        }
    }
    mask
}

fn num_winning_numbers(winning: &str, numbers: &str) -> usize {
    let w = to_mask(winning);
    let nums = to_mask(numbers);
    (w & nums).count_ones() as usize
}

#[aoc(part = 1, example = 13)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|line| {
            let (_, card) = line.split_once(": ")?;
            let (winning, numbers) = card.split_once(" | ")?;
            let w = num_winning_numbers(winning, numbers);
            Some(if w == 0 { 0 } else { 1 << (w - 1) })
        })
        .sum()
}

#[aoc(part = 2, example = 30)]
fn part2(input: Vec<String>) -> u64 {
    let mut counts = vec![1; input.len()];
    input.into_iter().enumerate().for_each(|(i, line)| {
        let (winning, numbers) = line
            .split_once(": ")
            .and_then(|(_, numbers)| numbers.split_once(" | "))
            .unwrap_or_default();
        let c = counts[i];
        counts
            .iter_mut()
            .skip(i + 1)
            .take(num_winning_numbers(winning, numbers))
            .for_each(|count| *count += c)
    });
    counts.into_iter().sum()
}
