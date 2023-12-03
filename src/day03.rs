use aoc_framework::{point::Point2, *};
use grid::Grid;

type GridView<'a> = grid::GridView<'a, u8, 2>;

pub struct Day03;

impl_day!(Day03::{part1, part2}: 2023[3], r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");

fn extract_number(g: &mut GridView<'_>, mut pos: Point2) -> u64 {
    let dx = Point2::new(1, 0);
    while let Some(b'0'..=b'9') = g.get(pos - dx) {
        pos -= dx;
    }
    let mut out = 0;
    while let Some(b @ b'0'..=b'9') = g.get(pos) {
        out = out * 10 + (b - b'0') as u64;
        g.set(pos, b'.');
        pos += dx;
    }
    out
}

#[aoc(part = 1, example = 4361)]
fn part1(input: Vec<String>) -> u64 {
    let mut grid = Grid::from_lines(input, |x| x);
    let mut sum = 0;
    for pos in grid.points_iter() {
        match grid.get(pos) {
            None | Some(b'.' | b'0'..=b'9') => continue,
            Some(_) => {}
        }

        for neighbor in pos.neighbors_diag() {
            if grid.get(neighbor).is_some_and(|b| b.is_ascii_digit()) {
                sum += extract_number(&mut grid, neighbor);
            }
        }
    }
    sum
}

#[aoc(part = 2, example = 467835)]
fn part2(input: Vec<String>) -> u64 {
    let mut grid = Grid::from_lines(input, |x| x);
    let mut sum = 0;
    for pos in grid.points_iter() {
        let Some(b'*') = grid.get(pos) else { continue };
        let mut nums = Vec::with_capacity(2);
        for neighbor in pos.neighbors_diag() {
            if grid.get(neighbor).is_some_and(|b| b.is_ascii_digit()) {
                nums.push(extract_number(&mut grid, neighbor));
            }
        }
        if nums.len() == 2 {
            sum += nums.into_iter().product::<u64>();
        }
    }
    sum
}
