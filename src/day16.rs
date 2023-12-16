use aoc_framework::{direction::Direction, grid::{Grid, GridView}, point::Point, *};

pub struct Day16;

impl_day!(Day16::{part1, part2}: 2023[16], r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
");

fn count_energized(g: &GridView<'_, u8, 2>, start_pos: Point<2>, start_dir: Direction<2>) -> u64 {
    let Point([w, h]) = g.size();
    let mut energized = Grid::from_data(vec![false; (w * h) as usize], w as usize);
    let mut beams = vec![(start_pos, start_dir)];
    while let Some((mut pos, mut dir)) = beams.pop() {
        loop {
            let Some(b) = g.get(pos) else {
                break;
            };
            let already_energized = energized[pos];
            energized.set(pos, true);
            let vertical = dir.delta().x() == 0;
            match b {
                b'/' => {
                    if vertical {
                        dir -= 1;
                    } else {
                        dir += 1;
                    }
                }
                b'\\' => {
                    if vertical {
                        dir += 1;
                    } else {
                        dir -= 1;
                    }
                }
                b'-' | b'|' if already_energized => break,
                b'-' if vertical => {
                    let l = dir + 1;
                    beams.push((pos + l, l));
                    let r = dir - 1;
                    beams.push((pos + r, r));
                    break;
                }
                b'|' if !vertical => {
                    let l = dir + 1;
                    beams.push((pos + l, l));
                    let r = dir - 1;
                    beams.push((pos + r, r));
                    break;
                }
                _ => (),
            }
            pos += dir;
        }
    }
    energized.data().iter().filter(|&&b| b).count() as u64
}

#[aoc(part = 1, example = 46)]
fn part1(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    count_energized(&g, Point([0, 0]), Direction::EAST)
}

#[aoc(part = 2, example = 51)]
fn part2(input: Vec<u8>) -> u64 {
    let g = Grid::from_bytes(input);
    let Point([w, h]) = g.size();
    let mut max = 0;
    let mut check = |x, y, dir| {
        let n = count_energized(&g, Point([x, y]), dir);
        if n > max {
            max = n;
        }
    };
    for x in 0..w {
        check(x, 0, Direction::SOUTH);
        check(x, h-1, Direction::NORTH);
    }
    for y in 0..h {
        check(0, y, Direction::EAST);
        check(w-1, y, Direction::WEST);
    }
    max
}
