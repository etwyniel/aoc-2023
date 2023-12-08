use aoc_framework::*;

pub struct Day01;

impl_day!(Day01::{part1, part2}: 2023[1], r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");

fn find_digit(mut it: impl Iterator<Item = u8>) -> Option<u8> {
    it.find(|b| b.is_ascii_digit()).map(|b| b - b'0')
}

#[aoc(part = 1, example = 142)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|line| {
            find_digit(line.bytes())
                .into_iter()
                .chain(find_digit(line.bytes().rev()))
                .fold(0, |acc, val| acc * 10 + val as u64)
        })
        .sum()
}

fn match_digit(s: &str) -> Option<u8> {
    if let Some(b @ b'0'..=b'9') = s.as_bytes().first() {
        return Some(b - b'0');
    }
    const NUMS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (d, num) in NUMS.iter().enumerate() {
        if s.starts_with(num) {
            return Some(d as u8);
        }
    }
    None
}

#[aoc(part = 2, example = 281)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|line| {
            let mut l = None;
            let mut r = None;
            for i in 0..line.len() {
                if l.is_none() {
                    if let Some(n) = match_digit(&line[i..]) {
                        l = Some(n as u64);
                    }
                }
                if r.is_none() {
                    if let Some(n) = match_digit(&line[(line.len() - 1 - i)..]) {
                        r = Some(n as u64);
                    }
                }

                if let (Some(l), Some(r)) = (l, r) {
                    return l * 10 + r;
                }
            }
            0
        })
        .sum()
}
