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
        for y in 0..h {
            let b = g[(x, y)];
            if b == b'O' {
                n_rocks += 1;
                continue;
            }
            if b == b'.' {
                continue;
            }
            for i in 0..n_rocks {
                load += h - wall - i - 1;
            }
            n_rocks = 0;
            wall = y;
        }
        for i in 0..n_rocks {
            load += h - wall - i - 1;
        }
    }
    load as u64
}

fn calc_load(g: &GridView<'_, u8, 2>) -> u64 {
    let Point([w, h]) = g.size();
    let mut load = 0;
    for x in 0..w {
        for y in 0..h {
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
    for dir_i in DIRECTIONS {
        let delta_i = dir_i.delta();
        let dir_j = dir_i + 1;
        let delta_j = dir_j.delta();
        let mut pj = dir_j.edge(g.size());
        while g.in_bounds(pj) {
            let mut pi = dir_i.edge(g.size());
            let mut last_empty = None;
            while g.in_bounds(pi) {
                let pos = pj + pi;
                match g[pos] {
                    b'#' => {
                        last_empty = None;
                    }
                    b'.' if last_empty.is_none() => {
                        last_empty = Some(pos);
                    }
                    b'.' => (),
                    _ => {
                        if let Some(empty) = last_empty {
                            g.set(empty, b'O');
                            g.set(pos, b'.');
                            last_empty = Some(empty + delta_i);
                        }
                    }
                }
                pi += delta_i;
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
    let grid = Grid::from_bytes(input);
    let mut grid = grid.clone();
    let mut values = Vec::new();
    let mut i = 0;
    const TARGET: usize = 1_000_000_000;
    let mut cycle_found = false;
    while i < TARGET {
        values.push(run_cycle(&mut grid));
        if !cycle_found {
            if let Some(len) = find_cycle(&values) {
                i += len * ((TARGET - i) / len) + 1;
                cycle_found = true;
                continue;
            }
        }
        i += 1;
    }
    calc_load(&grid)
}
