use std::env::args;

use aoc_framework::*;

mod day01;
use day01::Day01;

mod day02;
use day02::Day02;

mod day03;
use day03::Day03;

mod day04;
use day04::Day04;

mod day05;
use day05::Day05;

mod day06;
use day06::Day06;

mod day07;
use day07::Day07;

mod day08;
use day08::Day08;

mod day09;
use day09::Day09;

mod day10;
use day10::Day10;

mod day11;
use day11::Day11;

fn main() -> anyhow::Result<()> {
    let days = [
        Day01::run,
        Day02::run,
        Day03::run,
        Day04::run,
        Day05::run,
        Day06::run,
        Day07::run,
        Day08::run,
        Day09::run,
        Day10::run,
        Day11::run,
    ];

    let token = std::env::var("AOC_TOKEN").ok();

    if let Some(day) = args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(|day| days.get(day - 1))
    {
        day(token.as_deref());
        return Ok(());
    }

    for day in days {
        day(token.as_deref());
    }

    Ok(())
}
