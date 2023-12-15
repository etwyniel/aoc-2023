use std::array;

use aoc_framework::*;

pub struct Day15;

impl_day!(Day15::{part1, part2}: 2023[15], r"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
");

fn hash(s: &str) -> u64 {
    s.bytes().fold(0, |acc, b| ((acc + b as u64) * 17) % 256)
}

#[aoc(part = 1, example = 1320)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input.map(|ln| ln.split(',').map(hash).sum::<u64>()).sum()
}

#[aoc(part = 2, example = 145)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut map: [Vec<u64>; 256] = array::from_fn(|_| Vec::new());
    input.for_each(|ln| {
        ln.split(',').for_each(|step| {
            let len = step.len();
            let op = step.as_bytes()[len - 1];
            let label = if op == b'-' {
                &step[..len - 1]
            } else {
                &step[..len - 2]
            };
            let (h, label_num) = label.bytes().fold((0, 0), |(hash, label), b| {
                (((hash + b as u64) * 17) % 256, (label << 8) + b as u64)
            });
            let v = &mut map[h as usize];
            let ndx = v.iter().position(|entry| entry >> 8 == label_num);
            if op == b'-' {
                if let Some(ndx) = ndx {
                    v.remove(ndx);
                }
            } else {
                let val = op - b'0';
                let entry = label_num << 8 | val as u64;
                if let Some(ndx) = ndx {
                    v[ndx] = entry;
                } else {
                    v.push(entry);
                }
            }
        })
    });
    map.into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(j, entry)| (i + 1) * (j + 1) * (entry & 0xff) as usize)
        })
        .sum::<usize>() as u64
}
