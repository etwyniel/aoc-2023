use std::{cmp::Ordering, fmt::Debug};

use aoc_framework::*;

pub struct Day07;

impl_day!(Day07::{part1, part2}: 2023[7], r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
");

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Type {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

struct Hand {
    ty: Type,
    cards: [u8; 13],
    bid: u64,
    value: u64,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MAP: [char; 13] = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        for (i, n) in self.cards.into_iter().enumerate() {
            let c = MAP[i];
            for _ in 0..n {
                write!(f, "{c}")?;
            }
        }
        write!(f, " {:?} {}", self.ty, self.value)
    }
}

impl Hand {
    fn ty(cards: &[u8]) -> Type {
        let mut has_pair = false;
        let mut has_triplet = false;
        for n in cards {
            match n {
                5 => return Type::FiveOfAKind,
                4 => return Type::FourOfAKind,
                3 if has_pair => return Type::FullHouse,
                3 => has_triplet = true,
                2 if has_triplet => return Type::FullHouse,
                2 if has_pair => return Type::TwoPairs,
                2 => has_pair = true,
                _ => (),
            }
        }
        if has_triplet {
            Type::ThreeOfAKind
        } else if has_pair {
            Type::Pair
        } else {
            Type::HighCard
        }
    }

    fn card_value<const PART2: bool>(c: u8) -> Option<usize> {
        Some(match c {
            b'A' => 0,
            b'K' => 1,
            b'Q' => 2,
            b'J' => {
                if PART2 {
                    12
                } else {
                    3
                }
            }
            b'T' => {
                if PART2 {
                    3
                } else {
                    4
                }
            }

            b'2'..=b'9' => (b'9' - c) as usize + if PART2 { 4 } else { 5 },
            _ => return None,
        })
    }

    fn change_jokers(cards: &mut [u8]) {
        if cards[12] == 0 {
            return;
        }
        let n_j = cards[12];
        cards[12] = 0;
        let (max_pos, _) = cards.iter().enumerate().max_by_key(|(_, n)| *n).unwrap();
        cards[max_pos] += n_j;
    }

    fn parse<const PART2: bool>(s: &str) -> Option<Hand> {
        let (hand, bid) = s.split_once(' ')?;
        let mut cards = [0; 13];
        let mut value = 0;
        hand.bytes()
            .flat_map(Hand::card_value::<PART2>)
            .for_each(|ndx| {
                cards[ndx] += 1;
                value = value * 13 + (13 - ndx) as u64
            });
        if PART2 {
            Hand::change_jokers(&mut cards);
        }
        let ty = Hand::ty(&cards);
        let bid = bid.parse().ok()?;
        Some(Hand {
            ty,
            cards,
            bid,
            value,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ty_ord = self.ty.cmp(&other.ty);
        let Ordering::Equal = ty_ord else {
            return Some(ty_ord);
        };
        Some(self.value.cmp(&other.value))
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve<const PART2: bool>(input: impl Iterator<Item = String>) -> u64 {
    input
        .flat_map(|ln| Hand::parse::<PART2>(&ln))
        .sorted()
        .enumerate()
        .map(|(i, Hand { bid, .. })| bid * (i + 1) as u64)
        .sum()
}

#[aoc(part = 1, example = 6440)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    solve::<false>(input)
}

#[aoc(part = 2, example = 5905)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    solve::<true>(input)
}
