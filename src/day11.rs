use aoc_framework::{grid::Grid, point::Point, *};

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

fn to_cumulative(arr: &mut [isize]) {
    arr.iter_mut().reduce(|acc, elem| {
        *elem += *acc;
        elem
    });
}

fn solve(input: Vec<u8>, added_dist: isize) -> u64 {
    let grid = Grid::from_bytes(input);
    let mut galaxies = Vec::new();
    let size = grid.size();
    let mut doubled_rows = vec![added_dist; size.y() as usize];
    let mut doubled_cols = vec![added_dist; size.x() as usize];
    for y in 0..size.y() {
        for x in 0..size.x() {
            let p = Point([x, y]);
            if grid.get(p) == Some(&b'#') {
                galaxies.push(p);
                doubled_cols[x as usize] = 1;
                doubled_rows[y as usize] = 1;
            }
        }
    }
    to_cumulative(&mut doubled_cols);
    to_cumulative(&mut doubled_rows);
    galaxies
        .into_iter()
        .map(|Point([x, y])| {
            let new_y = doubled_rows[y as usize];
            let new_x = doubled_cols[x as usize];
            Point([new_x, new_y])
        })
        .tuple_combinations()
        .map(|(p1, p2)| p1.dist_manhattan(p2))
        .sum::<usize>() as u64
}

#[aoc(part = 1, example = 374)]
fn part1(input: Vec<u8>) -> u64 {
    solve(input, 2)
}

#[aoc(part = 2, example = 82000210)]
fn part2(input: Vec<u8>) -> u64 {
    solve(input, 1000000)
}
