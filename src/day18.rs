use aoc_framework::{
    direction::Direction,
    grid::{Grid, GridView},
    point::{Point},
    *,
};

pub struct Day18;

impl_day!(Day18::{part1, part2}: 2023[18], r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
");

fn flood_fill(g: &mut GridView<'_, u8, 2>) -> isize {
    let mut stack = vec![Point([0, 0])];
    g.set(Point([0, 0]), b' ');
    let mut count = 1;
    while let Some(p) = stack.pop() {
        for i in 0..4 {
            let neighbor = p + Direction::new(i);
            let Some(b'.') = g.get(neighbor) else {
                continue;
            };
            g.set(neighbor, b' ');
            stack.push(neighbor);
            count += 1;
        }
    }
    count
}

#[aoc(part = 1, example = 62)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut current = Point([0, 0]);
    let mut min = current;
    let mut max = current;
    let segments = input
        .flat_map(|ln| {
            let (dir, rem) = ln.split_once(' ')?;
            let (len, _) = rem.split_once(' ')?;
            let dir = match dir {
                "R" => Direction::EAST,
                "D" => Direction::SOUTH,
                "L" => Direction::WEST,
                _ => Direction::NORTH,
            };
            let len = len.parse::<isize>().ok()?;
            current += dir.delta() * (len);
            if current.x() < min.x() {
                min.0[0] = current.0[0];
            }
            if current.y() < min.y() {
                min.0[1] = current.0[1];
            }
            if current.x() > max.x() {
                max.0[0] = current.0[0];
            }
            if current.y() > max.y() {
                max.0[1] = current.0[1];
            }
            Some((dir, len))
        })
        .collect_vec();
    let Point([w, h]) = max - min;
    let (w, h) = (w + 3, h + 3);
    let mut g = Grid::from_data(vec![b'.'; (h * (w)) as usize], w as usize);
    segments
        .into_iter()
        .fold(-min + Point([1, 1]), |cur, (dir, len)| {
            for n in 1..=len {
                g.set(cur + dir.delta() * n, b'#');
            }
            cur + dir.delta() * len
        });
    let outside = flood_fill(&mut g);
    ((w) * h - outside) as u64
}

#[aoc(part = 2, example = 952408144115)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let start = Point::default();
    let mut current = start;
    let mut perimeter = 0;
    let points = input
        .flat_map(|ln| {
            let (_, hex_code) = ln.split_once('#')?;
            let len = isize::from_str_radix(&hex_code[..5], 16).ok()?;
            let dir = Direction::<2>::new(hex_code.as_bytes()[5] - b'0');
            perimeter += len;
            Some((dir, len))
        })
        .map(|(dir, len)| {
            current += dir.delta() * len;
            current
        })
        .collect_vec();
    points
        .iter()
        .chain(points.first())
        .tuple_windows()
        .map(|(l, r)| l.x() * r.y() - r.x() * l.y())
        .sum::<isize>()
        .abs() as u64
        / 2
        + (perimeter / 2) as u64
        + 1
}
