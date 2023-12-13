use aoc_framework::{
    grid::{self, Grid},
    point::Point,
    *,
};

pub struct Day13;

impl_day!(Day13::{part1, part2}: 2023[13], r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
");

type GridView = grid::GridView<'static, bool, 2>;

fn mirror(line: isize, max: isize) -> impl Iterator<Item = (isize, isize)> {
    (0..line).rev().zip(line..max)
}

fn col_reflects<F: FnMut(bool, bool) -> bool>(g: &GridView, x: isize, mut f: F) -> bool {
    let Point([w, h]) = g.size();
    (0..h).all(|y| mirror(x, w).all(|(l, r)| f(g[(l, y)], g[(r, y)])))
}

fn row_reflects<F: FnMut(bool, bool) -> bool>(g: &GridView, y: isize, mut f: F) -> bool {
    let Point([w, h]) = g.size();
    (0..w).all(|x| mirror(y, h).all(|(l, r)| f(g[(x, l)], g[(x, r)])))
}

fn find_reflection(g: &GridView) -> u64 {
    let Point([w, h]) = g.size();
    let horizontal = (1..w)
        .find(|&x| col_reflects(g, x, |l, r| l == r))
        .unwrap_or(0);
    let vertical = (1..h)
        .find(|&y| row_reflects(g, y, |l, r| l == r))
        .unwrap_or(0);
    (horizontal + vertical * 100) as u64
}

fn reflects_smudged<const HORIZONTAL: bool>(g: &GridView, axis: isize) -> bool {
    let mut diff = 0;
    let predicate = |l, r| {
        diff += (l != r) as u8;
        diff <= 1
    };
    (if HORIZONTAL {
        col_reflects(g, axis, predicate)
    } else {
        row_reflects(g, axis, predicate)
    }) && diff == 1
}

fn find_reflection_smudged(g: &GridView) -> u64 {
    let Point([w, h]) = g.size();
    let horizontal = (1..w)
        .find(|&x| reflects_smudged::<true>(g, x))
        .unwrap_or(0);
    let vertical = (1..h)
        .find(|&y| reflects_smudged::<false>(g, y))
        .unwrap_or(0);
    (horizontal + vertical * 100) as u64
}

fn solve<F: Fn(&GridView) -> u64>(mut input: impl Iterator<Item = String>, f: F) -> u64 {
    let mut total = 0;
    loop {
        let mut data = Vec::new();
        let mut stride = 0;
        (&mut input).take_while(|ln| !ln.is_empty()).for_each(|ln| {
            stride = ln.len();
            data.extend(ln.bytes().map(|b| b == b'#'))
        });
        if data.is_empty() {
            break;
        }
        total += f(Grid::from_data(data, stride).as_ref());
    }
    total
}

#[aoc(part = 1, example = 405)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    solve(input, find_reflection)
}

#[aoc(part = 2, example = 400)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    solve(input, find_reflection_smudged)
}
