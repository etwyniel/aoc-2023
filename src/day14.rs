use aoc_framework::{
    direction::Direction,
    grid::{Grid, GridView},
    point::Point,
    *,
};

pub struct Day14;

impl_day!(Day14::{part1, part2}: 2023[14], r"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
");

#[aoc(part = 1, example = 136)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let Point([w, h]) = g.size();
    let mut load = 0;
    for x in 0..w {
        let mut wall = -1;
        let mut n_rocks = 0;
        for y in 0..=h {
            match g.get(Point([x, y])) {
                Some(b'O') => n_rocks += 1,
                None | Some(b'#') => {
                    for i in 0..n_rocks {
                        load += h - wall - i - 1;
                    }
                    n_rocks = 0;
                    wall = y;
                }
                _ => (),
            }
        }
    }
    load as u64
}

fn calc_load(g: &GridView<'_, u8, 2>) -> u64 {
    let Point([w, h]) = g.size();
    let mut load = 0;
    for y in 0..h {
        for x in 0..w {
            let b = g[(x, y)];
            if b == b'.' || b == b'#' {
                continue;
            }
            load += h - y;
        }
    }
    load as u64
}

const DIRECTIONS: [Direction<2>; 4] = [
    Direction::SOUTH,
    Direction::EAST,
    Direction::NORTH,
    Direction::WEST,
];

fn run_cycle(g: &mut GridView<'_, u8, 2>) -> u64 {
    let size = g.size();
    for dir_i in DIRECTIONS {
        let delta_i = dir_i.delta();
        let dir_j = dir_i + 1;
        let delta_j = dir_j.delta();
        let mut pj = dir_j.edge(size);
        for _ in 0..size.len_in_dir(dir_j) {
            let mut pos = pj + dir_i.edge(size);
            let mut last_empty = None;
            for _ in 0..size.len_in_dir(dir_i) {
                match g[pos] {
                    b'#' => {
                        last_empty = None;
                    }
                    b'.' => {
                        last_empty.get_or_insert(pos);
                    }
                    _ => {
                        if let Some(empty) = last_empty {
                            g.set(empty, b'O');
                            g.set(pos, b'.');
                            last_empty = Some(empty + delta_i);
                        }
                    }
                }
                pos += delta_i;
            }
            pj += delta_j;
        }
    }
    calc_load(g)
}

fn find_cycle(values: &[u64]) -> Option<usize> {
    let len = values.len();
    for n in 4..(len / 2) {
        let mid = len - n;
        let arr1 = &values[(len - 2 * n)..mid];
        let arr2 = &values[mid..];
        if arr1 == arr2 {
            return Some(n);
        }
    }
    None
}

#[aoc(part = 2, example = 64)]
fn part2(input: Vec<u8>) -> u64 {
    let mut grid = Grid::from_bytes(input);
    let mut values = Vec::new();
    let mut i = 0;
    const TARGET: usize = 1_000_000_000;
    while i < TARGET {
        values.push(run_cycle(&mut grid));
        if let Some(len) = find_cycle(&values) {
            i += len * ((TARGET - i) / len) + 1;
            return values[values.len() - len - 1..][TARGET - i];
        }
        i += 1;
    }
    0
}
