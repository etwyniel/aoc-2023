use aoc_framework::{grid::Grid, point::Point2, *};

pub struct Day11;

impl_day!(Day11::{part1, part2}: 2023[11], r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
");

fn solve(input: Vec<u8>, added_dist: u64) -> u64 {
    let grid = Grid::from_bytes(input);
    let mut galaxies = Vec::new();
    let size = grid.size();
    let mut doubled_rows = vec![true; size.y() as usize];
    let mut doubled_cols = vec![true; size.x() as usize];
    for y in 0..size.y() {
        for x in 0..size.x() {
            let p = Point2::new(x, y);
            if grid.get(p) == Some(&b'#') {
                galaxies.push(p);
                doubled_cols[x as usize] = false;
                doubled_rows[y as usize] = false;
            }
        }
    }
    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            let mut len = 0;
            for y in p1.y().min(p2.y())..p1.y().max(p2.y()) {
                if doubled_rows[y as usize] {
                    len += added_dist;
                } else {
                    len += 1;
                }
            }
            for x in p1.x().min(p2.x())..p1.x().max(p2.x()) {
                if doubled_cols[x as usize] {
                    len += added_dist;
                } else {
                    len += 1;
                }
            }
            len
        })
        .sum()
}

#[aoc(part = 1, example = 374)]
fn part1(input: Vec<u8>) -> u64 {
    solve(input, 2)
}

#[aoc(part = 2)]
fn part2(input: Vec<u8>) -> u64 {
    solve(input, 1000000)
}
