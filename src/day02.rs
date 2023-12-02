use aoc_framework::*;

pub struct Day02;

impl_day!(Day02::{part1, part2}: 2023[2], r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
");

const MAX: [u64; 3] = [12, 13, 14];

fn color_index(c: &str) -> usize {
    match c {
        "red" => 0,
        "green" => 1,
        _ => 2,
    }
}

fn parse_handful(s: &str) -> [u64; 3] {
    let mut res = [0; 3];
    s.split(", ")
        .flat_map(|subset| {
            let (n, color) = subset.split_once(' ')?;
            Some((n.parse::<u64>().ok()?, color_index(color)))
        })
        .for_each(|(n, color)| res[color] += n);
    res
}

fn parse_game(g: &str) -> [u64; 3] {
    g.split("; ")
        .map(parse_handful)
        .fold([0; 3], |[acc_r, acc_g, acc_b], [r, g, b]| {
            [acc_r.max(r), acc_g.max(g), acc_b.max(b)]
        })
}

fn parse_header(line: &str) -> Option<(u64, &str)> {
    let (header, contents) = line.split_once(": ")?;
    let (_, n) = header.split_once(' ')?;
    Some((n.parse().ok()?, contents))
}

#[aoc(part = 1, example = 8)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .filter_map(|line| {
            let (n, contents) = parse_header(&line)?;
            contents
                .split("; ")
                .map(parse_handful)
                .all(|res| res.iter().zip(MAX).all(|(&val, max)| val <= max))
                .then_some(n)
        })
        .sum()
}

fn parse_line(line: String) -> Option<[u64; 3]> {
    let (_, game) = parse_header(&line)?;
    Some(parse_game(game))
}

#[aoc(part = 2, example = 2286)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(parse_line)
        .map(|mins| mins.iter().product::<u64>())
        .sum()
}
