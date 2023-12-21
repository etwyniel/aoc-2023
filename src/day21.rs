use std::collections::VecDeque;

use aoc_framework::{
    direction::Direction,
    grid::{Grid, GridView},
    point::{Point, Point2},
    *,
};

pub struct Day21;

impl_day!(Day21::{part1, part2}: 2023[21], r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
");

fn fill(g: &GridView<'_, u8, 2>, start: Point2, second_start: Option<Point2>, target: u64) -> (u64, u64) {
    let mut g = g.clone();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    queue.extend(second_start.map(|p| (p, 0)));
    let mut max_dist = 0;
    while let Some((pos, dist)) = queue.pop_front() {
        if dist > max_dist {
            max_dist = dist;
        }
        if dist == target {
            continue;
        }
        for i in 0..4 {
            let neighbor = pos + Direction::new(i);
            if g.get(neighbor) != Some(&b'.') {
                continue;
            }
            if dist % 2 == 0 {
                g.set(neighbor, b'o');
            } else {
                g.set(neighbor, b'O');
            }
            queue.push_back((neighbor, dist + 1))
        }
    }
    g.print_bytes();
    eprintln!();
    (
        g.data().iter().filter(|&&b| b == b'O').count() as u64 + 1,
        max_dist,
    )
}

#[aoc(part = 1, example = 16)]
fn part1(input: Vec<u8>) -> u64 {
    let target = if input.len() < 1000 { 6 } else { 64 };
    let mut g = Grid::from_bytes(input);
    let start = g.offset_to_point(g.data().iter().position(|&b| b == b'S').unwrap());
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if dist == target {
            continue;
        }
        for i in 0..4 {
            let neighbor = pos + Direction::new(i);
            if g.get(neighbor) != Some(&b'.') {
                continue;
            }
            if dist % 2 == 0 {
                g.set(neighbor, b'o');
            } else {
                g.set(neighbor, b'O');
            }
            queue.push_back((neighbor, dist + 1))
        }
    }
    g.data().iter().filter(|&&b| b == b'O').count() as u64 + 1
}

#[aoc(part = 2)]
fn part2(input: Vec<u8>) -> u64 {
    let target = 26501365;
    let mut g = Grid::from_bytes(input);
    let start = g.offset_to_point(g.data().iter().position(|&b| b == b'S').unwrap());
    g.set(start, b'.');
    let Point([w, h]) = g.size();
    let width = w as u64;
    let (fully_filled_steps, _) = fill(&g, start, None, target);
    let fully_filled_count = (target - w as u64 / 2) / w as u64;
    let remaining = (target - width) % width;
    let mut total = ((fully_filled_count + 1) * (fully_filled_count * 2 + 1 + 1) - (2 * fully_filled_count + 1))* fully_filled_steps;
    let left = Point([0, h / 2]);
    let bottom = Point([w / 2, h - 1]);
    let right = Point([w - 1, h / 2]);
    let top = Point([w / 2, 0]);
    total += fill(&g, left, None, remaining).0;
    total += fill(&g, bottom, None, remaining).0;
    total += fill(&g, right, None, remaining).0;
    total += fill(&g, top, None, remaining).0;

    total += fill(&g, left, Some(bottom), remaining).0 * fully_filled_count;
    total += fill(&g, bottom, Some(right), remaining).0 * fully_filled_count;
    total += fill(&g, right, Some(top), remaining).0 * fully_filled_count;
    total += fill(&g, top, Some(left), remaining).0 * fully_filled_count;
    total
}
