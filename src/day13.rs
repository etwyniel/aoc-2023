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

fn find_reflection(g: &GridView<'_, bool, 2>) -> u64 {
    let mut total = 0;
    let Point([w, h]) = g.size();
    'outer: for x in 1..w {
        for y in 0..h {
            if !(0..x)
                .rev()
                .zip(x..w)
                .all(|(l, r)| g[Point([l, y])] == g[Point([r, y])])
            {
                continue 'outer;
            }
        }
        total += x;
        break;
    }
    'outer: for y in 1..h {
        for x in 0..w {
            if !(0..y)
                .rev()
                .zip(y..h)
                .all(|(l, r)| g[Point([x, l])] == g[Point([x, r])])
            {
                continue 'outer;
            }
        }
        total += y * 100;
        break;
    }
    total as u64
}

fn find_reflection_smudged(g: &GridView<'_, bool, 2>) -> u64 {
    let mut total = 0;
    let Point([w, h]) = g.size();
    'outer: for x in 1..w {
        let mut smudge_found = false;
        for y in 0..h {
            if !(0..x)
                .rev()
                .zip(x..w)
                .map(|(l, r)| (g[Point([l, y])], g[Point([r, y])]))
                .all(|(l, r)| {
                    if !smudge_found && l != r {
                        smudge_found = true;
                        true
                    } else {
                        l == r
                    }
                })
            {
                continue 'outer;
            }
        }
        if smudge_found {
            total += x;
            break;
        }
    }
    let Point([w, h]) = g.size();
    'outer: for y in 1..h {
        let mut smudge_found = false;
        for x in 0..w {
            if !(0..y)
                .rev()
                .zip(y..h)
                .map(|(l, r)| (g[Point([x, l])], g[Point([x, r])]))
                .all(|(l, r)| {
                    if !smudge_found && l != r {
                        smudge_found = true;
                        true
                    } else {
                        l == r
                    }
                })
            {
                continue 'outer;
            }
        }
        if smudge_found {
            total += y * 100;
            break;
        }
    }
    total as u64
}

#[aoc(part = 1, example = 405)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
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
        total += find_reflection(Grid::from_data(data, stride).as_ref());
    }
    total
}

#[aoc(part = 2, example = 400)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
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
        total += find_reflection_smudged(Grid::from_data(data, stride).as_ref());
    }
    total
}
