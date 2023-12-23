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

fn fill(g: &GridView<'_, u8, 2>, start: Point2, second_start: Option<Point2>, target: u64) -> u64 {
    let mut g = g.clone().to_owned();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    queue.extend(second_start.map(|p| (p, 0)));
    let rem = target % 2;
    while let Some((pos, dist)) = queue.pop_front() {
        if dist == target {
            continue;
        }
        for i in 0..4 {
            let neighbor = pos + Direction::new(i);
            if g.get(neighbor) != Some(&b'.') {
                continue;
            }
            if dist % 2 == rem {
                g.set(neighbor, b'o');
            } else {
                g.set(neighbor, b'O');
            }
            queue.push_back((neighbor, dist + 1))
        }
    }
    g.data().iter().filter(|&&b| b == b'O').count() as u64
}

#[aoc(part = 2)]
fn part2(input: Vec<u8>) -> u64 {
    let target = 26501365;
    let mut g = Grid::from_bytes(input);
    let start = g.offset_to_point(g.data().iter().position(|&b| b == b'S').unwrap());
    g.set(start, b'.');
    let Point([w, h]) = g.size();
    let width = w as u64;
    let fully_filled_count = (target - width / 2) / width;
    let left = Point([-1, h / 2]);
    let bottom = Point([w / 2, h]);
    let right = Point([w, h / 2]);
    let top = Point([w / 2, -1]);

    let filled_odds = fill(&g, start, None, target);
    let filled_evens = fill(&g, left, None, target - 65);

    let mut num_odds = fully_filled_count - 1;
    for i in 1..fully_filled_count {
        num_odds += 2 * (fully_filled_count - i - 1);
    }
    let mut num_evens = fully_filled_count;
    for i in 1..fully_filled_count {
        num_evens += 2 * (fully_filled_count - i);
    }
    let mut total = num_odds * filled_odds + num_evens * filled_evens;

    total += fill(&g, left, None, 131);
    total += fill(&g, bottom, None, 131);
    total += fill(&g, right, None, 131);
    total += fill(&g, top, None, 131);

    total += fill(&g, Point([-1, h - 1]), None, 65) * fully_filled_count;
    total += fill(&g, Point([w - 1, h]), None, 65) * fully_filled_count;
    total += fill(&g, Point([-1, 0]), None, 65) * fully_filled_count;
    total += fill(&g, Point([w - 1, -1]), None, 65) * fully_filled_count;

    total += fill(&g, left, Some(bottom), 131) * (fully_filled_count - 1);
    total += fill(&g, bottom, Some(right), 131) * (fully_filled_count - 1);
    total += fill(&g, right, Some(top), 131) * (fully_filled_count - 1);
    total += fill(&g, top, Some(left), 131) * (fully_filled_count - 1);

    total
}
