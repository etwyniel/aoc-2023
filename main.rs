use std::env::args;

use aoc_framework::*;

mod grid;
mod helpers;
mod point;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
// mod day16;
mod day17;
mod day18;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use day01::Day1;
use day02::Day2;
use day03::Day3;
use day04::Day4;
use day05::Day5;
use day06::Day6;
use day07::Day7;
use day08::Day8;
use day09::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
// use day16::Day16;
use day17::Day17;
use day18::Day18;
// use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;

const DAYS: [fn(&str); 25] = [
    Day1::run,
    Day2::run,
    Day3::run,
    Day4::run,
    Day5::run,
    Day6::run,
    Day7::run,
    Day8::run,
    Day9::run,
    Day10::run,
    Day11::run,
    Day12::run,
    Day13::run,
    Day14::run,
    Day15::run,
    |_| (), // Day16::run,
    Day17::run,
    Day18::run,
    |_| (), // Day19::run,
    Day20::run,
    Day21::run,
    Day22::run,
    Day23::run,
    Day24::run,
    Day25::run,
];

fn main() -> anyhow::Result<()> {
    let token = std::env::var("AOC_TOKEN")?;

    if let Some(day) = args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(|day| DAYS.get(day - 1))
    {
        day(&token);
        return Ok(());
    }

    for day in DAYS {
        day(&token);
    }

    Ok(())
}
