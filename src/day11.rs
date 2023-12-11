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

fn expand(pos: isize, mask: &[bool], added: isize) -> isize {
    pos + mask.iter().take(pos as usize).filter(|&&b| b).count() as isize * added
}

fn solve(input: Vec<u8>, added_dist: isize) -> u64 {
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
        .map(|p| {
            let new_y = expand(p.y(), &doubled_rows, added_dist);
            let new_x = expand(p.x(), &doubled_cols, added_dist);
            Point2::new(new_x, new_y)
        })
        .tuple_combinations()
        .map(|(p1, p2)| p1.dist_manhattan(p2))
        .sum::<usize>() as u64
}

#[aoc(part = 1, example = 374)]
fn part1(input: Vec<u8>) -> u64 {
    solve(input, 1)
}

#[aoc(part = 2)]
fn part2(input: Vec<u8>) -> u64 {
    solve(input, 999999)
}
