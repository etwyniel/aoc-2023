use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use aoc_framework::{
    direction::Direction,
    grid::{Grid, GridView},
    point::{Point, Point2},
    *,
};

pub struct Day17;

impl_day!(Day17::{part1, part2}: 2023[17], r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
", r"
111111111111
999999999991
999999999991
999999999991
999999999991
");

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    p: Point2,
    dir: Direction<2>,
    count: u8,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.p.0.hash(state);
        self.dir.delta().0.hash(state);
        self.count.hash(state);
    }
}

struct KeyedBy<T, O: Ord> {
    v: T,
    o: O,
}

impl<T, O: Ord> PartialEq for KeyedBy<T, O> {
    fn eq(&self, other: &Self) -> bool {
        self.o == other.o
    }
}

impl<T, O: Ord> Eq for KeyedBy<T, O> {}

impl<T, O: Ord> PartialOrd for KeyedBy<T, O> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, O: Ord> Ord for KeyedBy<T, O> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.o.cmp(&other.o)
    }
}

fn total_cost(g: &GridView<'_, u8, 2>, last: &Node, came_from: &HashMap<Node, Node>) -> u64 {
    let mut total = (g[last.p] - b'0') as u64;
    let mut cur = *last;
    while let Some(node) = came_from.get(&cur) {
        if node.p.0 == [0, 0] {
            break;
        }
        total += (g[node.p] - b'0') as u64;
        cur = *node;
    }
    return total;
}

fn find_path(
    input: Vec<u8>,
    valid: impl Fn(&Node, isize) -> bool,
    end_is_valid: impl Fn(&Node) -> bool,
) -> u64 {
    let g = Grid::from_bytes(input);
    let end_pos = g.size() - Point([1, 1]);
    let mut open_set = BinaryHeap::new();
    let start_node = Node {
        p: Point([0, 0]),
        dir: Direction::EAST,
        count: 0,
    };
    open_set.push(KeyedBy {
        v: start_node,
        o: Reverse(0),
    });
    let mut came_from = HashMap::<Node, Node>::new();
    let mut g_score = HashMap::new();
    g_score.insert(start_node, 0);

    while let Some(KeyedBy { v: current, .. }) = open_set.pop() {
        if current.p == end_pos && end_is_valid(&current) {
            return total_cost(&g, &current, &came_from);
        }
        for d in 0..3 {
            if !valid(&current, d) {
                continue;
            }
            let dir = current.dir - 1 + d;
            let neighbor = current.p + dir;
            if !g.in_bounds(neighbor) {
                continue;
            }
            let neighbor_node = Node {
                p: neighbor,
                dir,
                count: if dir == current.dir {
                    current.count + 1
                } else {
                    1
                },
            };
            let tentative_score =
                g_score.get(&current).unwrap_or(&1000000) + (g[neighbor] - b'0') as u64;
            if &tentative_score < g_score.get(&neighbor_node).unwrap_or(&1000000) {
                came_from.insert(neighbor_node, current);
                g_score.insert(neighbor_node, tentative_score);
                let f = tentative_score * 100 + neighbor.dist_manhattan(end_pos) as u64;
                open_set.push(KeyedBy {
                    v: neighbor_node,
                    o: Reverse(f),
                });
            }
        }
    }
    0
}

#[aoc(part = 1, example = 102)]
fn part1(input: Vec<u8>) -> u64 {
    find_path(input, |current, d| d != 1 || current.count < 3, |_| true)
}

#[aoc(part = 2, example = 71)]
fn part2(input: Vec<u8>) -> u64 {
    find_path(
        input,
        |current, d| {
            current.count == 0 || (d == 1 && current.count < 10) || (d != 1 && current.count >= 4)
        },
        |end| end.count >= 4,
    )
}
