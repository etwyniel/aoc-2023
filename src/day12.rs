use std::{collections::HashMap, iter};

use aoc_framework::*;

pub struct Day12;

impl_day!(Day12::{part1, part2}: 2023[12], r"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");

fn byte_is<const B: char>(b: &&u8) -> bool {
    **b as char == B
}

fn potential_run_length(row: &[u8], run_target: usize) -> usize {
    row.iter()
        .take_while(|b| byte_is::<'#'>(b) || byte_is::<'?'>(b))
        .take(run_target)
        .count()
}

fn solve_part<'a>(
    row: &'a [u8],
    rle: &'a [usize],
    memo: &mut HashMap<(&'a [u8], &'a [usize]), Option<u64>>,
) -> Option<u64> {
    let skip = row.iter().take_while(byte_is::<'.'>).count();
    let row = &row[skip..];
    // check if result has already been computed and memoized
    if let Some(res) = memo.get(&(row, rle)) {
        return *res;
    }
    let (first, run_target) = match (row, rle) {
        ([], []) => return Some(1),
        ([], [_, ..]) => return None,
        ([_, ..], []) => return (!row.contains(&b'#')).then_some(1),
        ([b, ..], [target, ..]) => (*b, *target),
    };
    // count the length of a potential run starting at the first character of row
    let run = potential_run_length(row, run_target);
    let l = if first == b'#' {
        None
    } else {
        // count possinle permutations if the next ? isn't replaced
        solve_part(&row[1..], rle, memo)
    };
    if run != run_target || row.get(run) == Some(&b'#') {
        // not a valid run (too short or too long)
        return l;
    }

    // check the next run with the rest of the array.
    // skip one additional byte since a run cannot start immediately
    // after another one (which would make them the same run)
    let next_start = (run + 1).min(row.len());
    let r = solve_part(&row[next_start..], &rle[1..], memo);
    let res = match (l, r) {
        (Some(l), Some(r)) => Some(l + r),
        (Some(n), None) | (None, Some(n)) => Some(n),
        (None, None) => None,
    };
    memo.insert((row, rle), res);
    res
}

#[aoc(part = 1, example = 21)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| {
            let mut memo = HashMap::new();
            let (row, rle) = ln.split_once(' ')?;
            let rle = rle
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let count = solve_part(row.as_bytes(), &rle, &mut memo);
            count
        })
        .sum()
}

#[aoc(part = 2, example = 525152)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| {
            let (row, rle) = ln.split_once(' ')?;
            let row = iter::repeat(row).take(5).join("?");
            let rle = iter::repeat(rle)
                .take(5)
                .join(",")
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let mut memo = HashMap::new();
            solve_part(row.as_bytes(), &rle, &mut memo)
        })
        .sum()
}
