use std::{array, collections::{HashMap, HashSet}, iter, ops::Range};

use aoc_framework::*;
use smallvec::SmallVec;

pub struct Day19;

impl_day!(Day19::{part1, part2}: 2023[19], r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
");

#[derive(Debug, Clone, Copy)]
struct Record([u16; 4]);

#[derive(Clone, Copy)]
struct Cond {
    elem: u8,
    gt: bool,
    value: u16,
    tgt: u32,
}

struct Node {
    conds: SmallVec<[Cond; 4]>,
    default: u32,
}

impl Cond {
    fn matches(&self, rec: Record) -> bool {
        let v = rec.0[self.elem as usize];
        if self.gt {
            v > self.value
        } else {
            v < self.value
        }
    }
}

impl Node {
    fn output(&self, rec: Record) -> u32 {
        self.conds
            .iter()
            .find(|cond| cond.matches(rec))
            .map(|cond| cond.tgt)
            .unwrap_or(self.default)
    }
}

fn parse_label(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| acc << 8 | (b as u32))
}

fn elem_to_offset(b: u8) -> u8 {
    match b {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        _ => 3,
    }
}

fn parse_cond(cond: &str) -> Option<Cond> {
    let (cond, tgt) = cond.split_once(':')?;
    let elem = elem_to_offset(cond.as_bytes()[0]);
    let gt = cond.as_bytes()[1] == b'>';
    let value = cond[2..].parse().ok()?;
    Some(Cond {
        elem,
        gt,
        value,
        tgt: parse_label(tgt),
    })
}

fn parse_rec(rec: &str) -> Record {
    let mut arr = [0; 4];
    rec[1..rec.len() - 1]
        .split(',')
        .map(|elem| elem[2..].parse::<u16>().unwrap())
        .enumerate()
        .for_each(|(i, n)| arr[i] = n);
    Record(arr)
}

fn parse_nodes(input: impl Iterator<Item = String>) -> HashMap<u32, Node> {
    input
        .take_while(|ln| !ln.is_empty())
        .flat_map(|ln| {
            let pos = ln.find('{')?;
            let label = parse_label(&ln[..pos]);
            let (conds, default) = ln[pos + 1..ln.len() - 1].rsplit_once(',')?;
            let conds = conds.split(',').flat_map(parse_cond).collect();
            let node = Node {
                conds,
                default: parse_label(default),
            };
            Some((label, node))
        })
        .collect()
}

#[aoc(part = 1, example = 19114)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let nodes = parse_nodes(&mut input);
    let a = parse_label("A");
    let r = parse_label("R");
    let start = parse_label("in");
    let mut sum = 0;
    input.map(|ln| parse_rec(&ln)).for_each(|rec| {
        let mut current = start;
        while current != a && current != r {
            current = nodes[&current].output(rec);
        }
        if current == a {
            sum += rec.0.iter().map(|&v| v as u64).sum::<u64>();
        }
    });
    sum
}

#[derive(Clone, Debug)]
struct RangeRec([Range<u16>; 4]);

impl RangeRec {
    fn split(&self, elem: u8, at: u16, gt: bool) -> (Option<Self>, Option<Self>) {
        let range = &self.0[elem as usize];
        if at > range.end {
            return (Some(self.clone()), None);
        }
        if at < range.start {
            return (None, Some(self.clone()));
        }
        let mut l = self.clone();
        let l_range = &mut l.0[elem as usize];
        l_range.end = at + gt as u16;
        let mut r = self.clone();
        let r_range = &mut r.0[elem as usize];
        r_range.start = at + gt as u16;
        (
            (l_range.len() > 0).then_some(l),
            (r_range.len() > 0).then_some(r),
        )
    }
}

#[derive(Clone, Copy)]
enum Input {
    Default,
    Cond(Cond),
}

#[aoc(part = 2, example = 167409079868000)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let nodes = parse_nodes(input);
    let mut rev_nodes = HashMap::<u32, Vec<(SmallVec<[Cond; 4]>,Input, u32)>>::new();
    nodes
        .iter()
        .flat_map(|(&id, node)| {
            node.conds
                .iter()
                .enumerate()
                .map(move |(i, cond)| (cond.tgt, node.conds[..i].into(), Input::Cond(*cond), id))
                .chain(iter::once((node.default, node.conds.clone(), Input::Default, id)))
        })
        .for_each(|(dst, prev_nodes, input, src)| rev_nodes.entry(dst).or_default().push((prev_nodes, input, src)));
    let start = parse_label("in");
    let end = parse_label("A");
    let mut in_path = HashSet::new();
    in_path.insert(end);
    let mut stack = vec![end];
    while let Some(current) = stack.pop() {
        if current == start {
            continue;
        }
        for (_, _, id) in &rev_nodes[&current] {
            if in_path.contains(id) {
                continue;
            }
            in_path.insert(*id);
            stack.push(*id);
        }
    }

    let mut stack = vec![(start, RangeRec(array::from_fn(|_| 1..4001)))];
    let mut total = 0;
    'outer: while let Some((current, mut rec)) = stack.pop() {
        if current == end {
            total += rec.0.into_iter().fold(1, |acc, range| acc * range.len() as u64);
            continue;
        }
        let node = &nodes[&current];
        for cond in &node.conds {
            let (l, r) = rec.split(cond.elem, cond.value, cond.gt);
            let (yes, no) = if cond.gt {(r, l)} else {(l, r)};
            if in_path.contains(&cond.tgt) {
                if let Some(next_rec) = yes {
                    stack.push((cond.tgt, next_rec));
                }
            }
            let Some(next_rec) = no else {
                continue 'outer;
            };
            rec = next_rec;
        }
        if in_path.contains(&node.default) {
            stack.push((node.default, rec));
        }
    }

    total
}
