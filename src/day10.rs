use aoc_framework::{
    grid::{Grid, GridView},
    point::Point2,
    *,
};

pub struct Day10;

impl_day!(Day10::{part1, part2}: 2023[10], r"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
", r"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
");

const PIPES_DELTAS: [([isize; 2], [u8; 3]); 4] = [
    ([1, 0], [b'J', b'-', b'7']),
    ([0, 1], [b'J', b'|', b'L']),
    ([-1, 0], [b'L', b'-', b'F']),
    ([0, -1], [b'7', b'|', b'F']),
];

#[aoc(part = 1, example = 8)]
fn part1(input: Vec<u8>) -> u64 {
    let start_ndx = input.iter().position(|&b| b == b'S').unwrap_or(0);
    let grid = Grid::from_bytes(input);
    let mut pos = grid.offset_to_point(start_ndx);
    let mut delta = Point2::default();
    for (d, pipes) in PIPES_DELTAS {
        let neighbor = pos + Point2::from(d);
        let Some(&b) = grid.get(neighbor) else {
            continue;
        };
        if pipes.contains(&b) {
            delta = d.into();
            break;
        }
    }
    if delta == Point2::default() {
        return 0;
    }
    let mut len = 1;
    loop {
        let next = pos + delta;
        delta = match (grid.get(next).unwrap(), delta.x(), delta.y()) {
            (b'S', _, _) => break,
            (b'-' | b'|', dx, dy) => [dx, dy],
            (b'F', 0, -1) | (b'L', 0, 1) => [1, 0],
            (b'7', 1, 0) | (b'F', -1, 0) => [0, 1],
            (b'7', 0, -1) | (b'J', 0, 1) => [-1, 0],
            (b'J', 1, 0) | (b'L', -1, 0) => [0, -1],
            _ => unreachable!(),
        }
        .into();
        len += 1;
        pos = next;
    }
    len / 2
}

fn flood_fill(grid: &mut GridView<'_, u8, 2>) {
    let start = Point2::default();
    grid.set(start, b'.');
    let mut stack = vec![Point2::new(0, 0)];
    while let Some(pos) = stack.pop() {
        const DELTAS: [Point2; 4] = [
            Point2::new(1, 0),
            Point2::new(0, 1),
            Point2::new(-1, 0),
            Point2::new(0, -1),
        ];
        for d in DELTAS {
            let neigh = pos + d;
            if let Some(b' ') = grid.get(neigh) {
                grid.set(neigh, b'.');
                stack.push(neigh);
            }
        }
    }
}

#[aoc(part = 2, example = 10)]
fn part2(input: Vec<u8>) -> u64 {
    let start_ndx = input.iter().position(|&b| b == b'S').unwrap_or(0);
    let grid = Grid::from_bytes(input);
    let size = grid.size();
    let mut fill_grid = Grid::from_data(
        vec![b' '; (size.x() * size.y()) as usize * 9],
        size.x() as usize * 3,
    );
    let mut pos = grid.offset_to_point(start_ndx);
    let mut delta = Point2::default();
    for (d, pipes) in PIPES_DELTAS {
        let neighbor = pos + Point2::from(d);
        let Some(&b) = grid.get(neighbor) else {
            continue;
        };
        if pipes.contains(&b) {
            delta = d.into();
            break;
        }
    }
    if delta == Point2::default() {
        return 0;
    }
    loop {
        let next = pos + delta;
        let new_delta = match (grid.get(next).unwrap(), delta.x(), delta.y()) {
            (b'S', _, _) => break,
            (b'-' | b'|', dx, dy) => [dx, dy],
            (b'F', 0, -1) | (b'L', 0, 1) => [1, 0],
            (b'7', 1, 0) | (b'F', -1, 0) => [0, 1],
            (b'7', 0, -1) | (b'J', 0, 1) => [-1, 0],
            (b'J', 1, 0) | (b'L', -1, 0) => [0, -1],
            _ => unreachable!(),
        }
        .into();
        for i in 1..=3 {
            let fill_pos = pos * 3 + delta * i + Point2::new(1, 1);
            fill_grid.set(fill_pos, b'#');
        }
        for i in 1..=3 {
            let fill_pos = next * 3 + new_delta * i + Point2::new(1, 1);
            fill_grid.set(fill_pos, b'#');
        }
        pos = next;
        delta = new_delta;
    }
    flood_fill(&mut fill_grid);
    grid.points_iter()
        .flat_map(|p| fill_grid.get(p * 3 + Point2::new(1, 1)))
        .filter(|&&b| b == b' ')
        .count() as u64
}
