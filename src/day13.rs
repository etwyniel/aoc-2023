use aoc_framework::{
    grid::{Grid, GridView},
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

fn mirror(line: isize, max: isize) -> impl Iterator<Item = (isize, isize)> {
    (0..line).rev().zip(line..max)
}

fn find_reflection(g: &GridView<'_, bool, 2>) -> u64 {
    let mut total = 0;
    let Point([w, h]) = g.size();
    total += (1..w)
        .find(|&x| (0..h).all(|y| mirror(x, w).all(|(l, r)| g[(l, y)] == g[(r, y)])))
        .unwrap_or(0);
    total += (1..h)
        .find(|&y| (0..w).all(|x| mirror(y, h).all(|(l, r)| g[(x, l)] == g[(x, r)])))
        .unwrap_or(0)
        * 100;
    total as u64
}

fn find_reflection_smudged(g: &GridView<'_, bool, 2>) -> u64 {
    let mut total = 0;
    let Point([w, h]) = g.size();
    total += (1..w)
        .find(|&x| {
            let mut diff = 0;
            (0..h).all(|y| {
                mirror(x, w)
                    .map(|(l, r)| (g[(l, y)], g[(r, y)]))
                    .all(|(l, r)| {
                        diff += (l != r) as u8;
                        diff <= 1
                    })
            }) && diff == 1
        })
        .unwrap_or(0);
    total += (1..h)
        .find(|&y| {
            let mut diff = 0;
            (0..w).all(|x| {
                mirror(y, h)
                    .map(|(l, r)| (g[(x, l)], g[(x, r)]))
                    .all(|(l, r)| {
                        diff += (l != r) as u8;
                        diff <= 1
                    })
            }) && diff == 1
        })
        .unwrap_or(0)
        * 100;
    total as u64
}

fn solve<F: Fn(&GridView<'_, bool, 2>) -> u64>(
    mut input: impl Iterator<Item = String>,
    f: F,
) -> u64 {
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
