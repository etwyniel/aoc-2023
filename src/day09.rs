use aoc_framework::*;

pub struct Day09;

impl_day!(Day09::{part1, part2}: 2023[9], r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
");

fn extrapolate_back(arr: &mut [i64]) -> i64 {
    let mut len = arr.len();
    loop {
        let mut next = arr[len - 1];
        let arr = &mut arr[..len - 1];
        let mut all_zeroes = true;
        arr.iter_mut().rev().for_each(|n| {
            let b = next;
            next = *n;
            *n = b - *n;
            all_zeroes &= *n == 0;
        });
        len = arr.len();
        if all_zeroes {
            break;
        }
    }
    let mut last_val = 0;
    for n in &arr[len..] {
        last_val += n;
    }
    last_val
}

fn extrapolate_front(arr: &mut [i64]) -> i64 {
    let mut pos = 0;
    loop {
        let mut prev = arr[pos];
        let arr = &mut arr[pos + 1..];
        let mut all_zeroes = true;
        arr.iter_mut().for_each(|n| {
            let b = prev;
            prev = *n;
            *n -= b;
            all_zeroes &= *n == 0;
        });
        pos += 1;
        if all_zeroes {
            break;
        }
    }
    let mut last_val = 0;
    for n in arr[..pos].iter().rev() {
        last_val = n - last_val;
    }
    last_val
}

#[aoc(part = 1, example = 114)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|ln| ln.split(' ').flat_map(|s| s.parse().ok()).collect())
        .map(|mut values: Vec<i64>| extrapolate_back(&mut values))
        .sum::<i64>() as u64
}

#[aoc(part = 2, example = 2)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|ln| ln.split(' ').flat_map(|s| s.parse().ok()).collect())
        .map(|mut values: Vec<i64>| extrapolate_front(&mut values))
        .sum::<i64>() as u64
}
