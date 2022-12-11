use crate::AocDay;
use std::{collections::BinaryHeap, str::FromStr};
use strength_reduce::StrengthReducedU64;

#[derive(Default)]
pub struct Parts {}

impl AocDay for Parts {
    fn part_1(&self) -> String {
        p1()
    }

    fn part_2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect::<Vec<_>>();

    let div = StrengthReducedU64::new(3);
    for _round in 0..20 {
        for id in 0..monkeys.len() {
            while let Some(item_worry_level) = monkeys[id].inspect_item() {
                let (div_worry_level, target_id) = monkeys[id].test_with_div(item_worry_level, div);
                monkeys[target_id].items.push(div_worry_level);
            }
        }
    }
    let mut heap = BinaryHeap::from_iter(monkeys.iter().map(|m| m.inspected_items));
    format!("{:?}", { heap.pop().unwrap() * heap.pop().unwrap() })
}

pub fn p2() -> String {
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect::<Vec<_>>();

    let div_prod: u64 = monkeys.iter().map(|monkey| monkey.test).product();
    let mod_fa = StrengthReducedU64::new(div_prod);

    for _round in 0..10_000 {
        for id in 0..monkeys.len() {
            while let Some(item_worry_level) = monkeys[id].inspect_item() {
                let (div_worry_level, target_id) =
                    monkeys[id].test_with_modulo(item_worry_level, mod_fa);
                monkeys[target_id].items.push(div_worry_level);
            }
        }
    }

    let mut heap = BinaryHeap::from_iter(monkeys.iter().map(|m| m.inspected_items));
    format!("{:?}", { heap.pop().unwrap() * heap.pop().unwrap() })
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum OpR {
    Old,
    Constant(u8),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operand: Op,
    operand_r: OpR,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected_items: u64,
}

impl Monkey {
    pub fn inspect_item(&mut self) -> Option<u64> {
        if let Some(item) = self.items.pop() {
            self.inspected_items += 1;
            let l = item;
            let r = match self.operand_r {
                OpR::Old => item,
                OpR::Constant(con) => con as u64,
            };
            let new_worry_level = match self.operand {
                Op::Add => l + r,
                Op::Mul => l * r,
            };
            Some(new_worry_level)
        } else {
            None
        }
    }

    pub fn test_with_div(&mut self, worry_level: u64, divisor: StrengthReducedU64) -> (u64, usize) {
        let relieved_worry = worry_level / divisor;

        if relieved_worry % self.test == 0 {
            (relieved_worry, self.if_true)
        } else {
            (relieved_worry, self.if_false)
        }
    }

    pub fn test_with_modulo(
        &mut self,
        worry_level: u64,
        modulo: StrengthReducedU64,
    ) -> (u64, usize) {
        let relieved_worry = worry_level % modulo;

        if relieved_worry % self.test == 0 {
            (relieved_worry, self.if_true)
        } else {
            (relieved_worry, self.if_false)
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_items = Vec::new();
        let mut test = None;
        let mut if_true = None;
        let mut if_false = None;
        let mut op_op = None;
        let mut op_r = None;

        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("Starting items") {
                let parts: Vec<&str> = line.split(':').collect();
                starting_items = parts[1]
                    .split(',')
                    .map(|x| x.trim().parse().unwrap())
                    .collect();
            } else if line.starts_with("Operation") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                op_op = Some(match parts[4].trim() {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!(),
                });
                op_r = Some(match parts[5].trim() {
                    "old" => OpR::Old,
                    val => OpR::Constant(val.parse::<u8>().unwrap()),
                });
            } else if line.starts_with("Test") {
                let parts = line.split_whitespace().last().unwrap();
                test = Some(parts.trim().parse::<u64>().unwrap());
            } else if line.starts_with("If true") {
                let parts: Vec<&str> = line.split(':').collect();
                if_true = Some(parts[1].split_whitespace().last().unwrap().parse().unwrap());
            } else if line.starts_with("If false") {
                let parts: Vec<&str> = line.split(':').collect();
                if_false = Some(parts[1].split_whitespace().last().unwrap().parse().unwrap());
            }
        }

        Ok(Monkey {
            items: starting_items,
            test: test.unwrap(),
            if_true: if_true.unwrap(),
            if_false: if_false.unwrap(),
            inspected_items: 0,
            operand: op_op.unwrap(),
            operand_r: op_r.unwrap(),
        })
    }
}
