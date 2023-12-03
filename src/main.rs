use std::env::args;

use aoc_framework::*;

mod day01;
use day01::Day01;

mod day02;
use day02::Day02;

mod day03;
use day03::Day03;

fn main() -> anyhow::Result<()> {
    let days = [Day01::run, Day02::run, Day03::run];

    let token = std::env::var("AOC_TOKEN")?;

    if let Some(day) = args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(|day| days.get(day - 1))
    {
        day(&token);
        return Ok(());
    }

    for day in days {
        day(&token);
    }

    Ok(())
}
