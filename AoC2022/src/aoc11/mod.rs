use std::{collections::VecDeque, str::FromStr};

use crate::AocDay;

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

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_id].inspect_item() {
                let new_worry_level = item;
                let (div_worry_level, target_id) =
                    monkeys[monkey_id].test_with_div(new_worry_level);

                monkeys[target_id as usize].add_item(div_worry_level);
            }
        }
    }
    let mut sums = monkeys
        .iter()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();
    sums.sort();
    let t = sums.iter().rev().take(2).collect::<Vec<_>>();
    format!("{:?}", { t[0] * t[1] })
}

pub fn p2() -> String {
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|monkey| Monkey::from_str(monkey).unwrap())
        .collect::<Vec<_>>();

    let div_prod: usize = monkeys.iter().map(|monkey| monkey.test).product();

    for _round in 0..10_000 {
        for id in 0..monkeys.len() {
            while let Some(item) = monkeys[id].inspect_item() {
                let new_worry_level = item;
                let (div_worry_level, target_id) =
                    monkeys[id].test_with_modulo(new_worry_level, div_prod);

                monkeys[target_id].add_item(div_worry_level);
            }
        }
    }
    let mut sums = monkeys
        .iter()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();
    sums.sort();
    let t = sums.iter().rev().take(2).collect::<Vec<_>>();
    format!("{:?}", { t[0] * t[1] })
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: (String, String, String),
    test: usize,
    if_true: usize,
    if_false: usize,
    inspected_items: usize,
}

impl Monkey {
    pub fn inspect_item(&mut self) -> Option<usize> {
        if let Some(item) = self.items.pop_front() {
            self.inspected_items += 1;
            let (l, operand, r) = &self.operation;
            let l = match l.as_str() {
                "old" => item,
                _ => panic!(),
            };
            let r = match r.as_str() {
                "old" => item,
                _ => r.parse::<usize>().unwrap(),
            };
            let new_worry_level = match operand.as_str() {
                "*" => l * r,
                "+" => l + r,
                _ => panic!(),
            };
            Some(new_worry_level)
        } else {
            None
        }
    }

    pub fn test_with_div(&mut self, worry_level: usize) -> (usize, usize) {
        let relieved_worry = worry_level / 3;

        if relieved_worry % self.test == 0 {
            (relieved_worry, self.if_true)
        } else {
            (relieved_worry, self.if_false)
        }
    }

    pub fn test_with_modulo(&mut self, worry_level: usize, modulo: usize) -> (usize, usize) {
        let relieved_worry = worry_level % modulo;

        if relieved_worry % self.test == 0 {
            (relieved_worry, self.if_true)
        } else {
            (relieved_worry, self.if_false)
        }
    }

    pub fn add_item(&mut self, worry_level: usize) {
        self.items.push_back(worry_level)
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_items = VecDeque::new();
        let mut operation = None;
        let mut test = None;
        let mut if_true = None;
        let mut if_false = None;

        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("Starting items") {
                let parts: Vec<&str> = line.split(":").collect();
                starting_items = parts[1]
                    .split(",")
                    .map(|x| x.trim().parse().unwrap())
                    .collect();
            } else if line.starts_with("Operation") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let p_1 = parts[3].trim().to_string();
                let p_2 = parts[4].trim().to_string();
                let p_3 = parts[5].trim().to_string();
                operation = Some((p_1, p_2, p_3));
            } else if line.starts_with("Test") {
                let parts = line.split_whitespace().last().unwrap();
                test = Some(parts.trim().parse::<usize>().unwrap());
            } else if line.starts_with("If true") {
                let parts: Vec<&str> = line.split(":").collect();
                if_true = Some(
                    parts[1]
                        .trim()
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap(),
                );
            } else if line.starts_with("If false") {
                let parts: Vec<&str> = line.split(":").collect();
                if_false = Some(
                    parts[1]
                        .trim()
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap(),
                );
            }
        }

        if operation.is_none() || test.is_none() || if_true.is_none() || if_false.is_none() {
            return Err("Invalid input".to_string());
        }
        Ok(Monkey {
            items: starting_items,
            operation: operation.unwrap(),
            test: test.unwrap(),
            if_true: if_true.unwrap(),
            if_false: if_false.unwrap(),
            inspected_items: 0,
        })
    }
}
