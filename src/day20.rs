use std::{
    array,
    collections::{HashMap, VecDeque},
};

use aoc_framework::*;
use smallvec::SmallVec;

pub struct Day20;

impl_day!(Day20::{part1, part2}: 2023[20], r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
");

#[derive(Default)]
struct InOut {
    inputs: SmallVec<[u8; 5]>,
    outputs: SmallVec<[u8; 5]>,
}

#[derive(Debug, Clone, Default)]
enum Module {
    Broadcaster,
    #[default]
    Simple,
    FlipFlop(bool),
    Conjunction(u64),
}

use Module::*;

impl Module {
    fn handle_pulse(&mut self, pulse: bool, src: u8) -> Option<bool> {
        match self {
            Broadcaster => Some(pulse),
            Simple => None,
            FlipFlop(_) if pulse => None,
            FlipFlop(state) => {
                *state = !*state;
                Some(*state)
            }
            Conjunction(state) => {
                let offset = 1 << src;
                let mask = offset * pulse as u64;
                *state = (*state & !(offset)) | mask;
                Some(*state != u64::MAX)
            }
        }
    }
}

fn name_to_id(name: &str, name_map: &mut HashMap<String, u8>) -> u8 {
    if let Some(id) = name_map.get(name) {
        return *id;
    }
    let id = name_map.len() as u8;
    name_map.insert(name.to_string(), id);
    id
}

fn parse_module(
    ln: &str,
    name_map: &mut HashMap<String, u8>,
    inout: &mut [InOut; 64],
) -> Option<(u8, Module)> {
    let (module, outputs) = ln.split_once(" -> ")?;
    let (typ, name) = match module.as_bytes()[0] {
        typ @ (b'%' | b'&') => (typ, &module[1..]),
        _ => (0, module),
    };
    let id = name_to_id(name, name_map);
    for output in outputs.split(", ") {
        let output_id = name_to_id(output, name_map);
        inout[output_id as usize].inputs.push(id);
        inout[id as usize].outputs.push(output_id);
    }
    Some((
        id,
        match (typ, name) {
            (_, "broadcaster") => Broadcaster,
            (b'%', _) => FlipFlop(false),
            (b'&', _) => Conjunction(u64::MAX),
            _ => Simple,
        },
    ))
}

fn setup(input: impl Iterator<Item = String>) -> (HashMap<String, u8>, [InOut; 64], [Module; 64]) {
    let mut name_map = HashMap::new();
    let mut inout = array::from_fn(|_| InOut::default());
    let mut modules: [Module; 64] = array::from_fn(|_| Default::default());
    input
        .flat_map(|ln| parse_module(&ln, &mut name_map, &mut inout))
        .for_each(|(id, module)| modules[id as usize] = module);

    for (module, inout) in modules.iter_mut().zip(&inout) {
        if let Conjunction(state) = module {
            inout.inputs.iter().for_each(|module| *state &= !(1 << *module));
        }
    }
    (name_map, inout, modules)
}

#[aoc(part = 1, example = 11687500)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (name_map, inout, mut modules) = setup(input);

    let broadcaster_id = name_map["broadcaster"];
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        queue.push_back((false, broadcaster_id, 0));

        while let Some((pulse, id, src)) = queue.pop_front() {
                if pulse {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
            let result = modules[id as usize]
                .handle_pulse(pulse, src);
            let Some(pulse) = result else {
                continue;
            };
            let mod_inout = &inout[id as usize];
            for output in &mod_inout.outputs {
                queue.push_back((pulse, *output, id));
            }
        }
    }
    low_pulses * high_pulses
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

#[aoc(part = 2)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let (name_map, inout, mut modules) = setup(input);

    let broadcaster_id = name_map["broadcaster"];
    let rx_id = name_map["rx"];
    let rx_input = &inout[rx_id as usize].inputs;
    assert_eq!(rx_input.len(), 1);
    assert!(matches!(modules[rx_input[0] as usize], Conjunction(_)));
    let inputs = &inout[rx_input[0] as usize].inputs;
    let mut is_conj_input = [false; 64];
    inputs.iter().for_each(|&module| is_conj_input[module as usize] = true);
    let mut input_cycles = vec![0; inputs.len()];
    let mut queue = VecDeque::new();

    for i in 1.. {
        queue.push_back((false, broadcaster_id, 0));

        while let Some((pulse, id, src)) = queue.pop_front() {
            let result = modules[id as usize].handle_pulse(pulse, src);
            let Some(pulse) = result else {
                continue;
            };
            if pulse && is_conj_input[id as usize] {
                let pos = inputs.iter().position(|inp| *inp == id).unwrap();
                input_cycles[pos] = i;
                if input_cycles.iter().all(|cycle| *cycle != 0) {
                    return input_cycles.into_iter().reduce(lcm).unwrap_or(0);
                }
            }
            let mod_inout = &inout[id as usize];
            for output in &mod_inout.outputs {
                queue.push_back((pulse, *output, id));
            }
        }
    }
    0
}
