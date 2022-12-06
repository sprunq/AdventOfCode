use crate::Part;
use std::fs;

#[derive(Default)]
pub struct Day1 {}

impl Part for Day1 {
    fn p1(&self) -> String {
        p1()
    }

    fn p2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let input = fs::read_to_string("src\\aoc1\\input.txt").expect("Cannot find file");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut int_vec = vec![];
    for elem in split {
        let clean: Vec<_> = elem.split('\n').collect();
        let ints = clean
            .iter()
            .map(|f| f.parse::<i64>().unwrap_or(0))
            .sum::<i64>();
        int_vec.push(ints);
    }
    format!("{:?}", int_vec.iter().max().unwrap())
}

pub fn p2() -> String {
    let input = fs::read_to_string("src\\aoc1\\input.txt").expect("Cannot find file");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut int_vec = vec![];
    for elem in split {
        let clean: Vec<_> = elem.split('\n').collect();
        let ints = clean
            .iter()
            .map(|f| f.parse::<i64>().unwrap_or(0))
            .sum::<i64>();
        int_vec.push(ints);
    }
    int_vec.sort();
    format!("{:?}", int_vec.iter().rev().take(3).sum::<i64>())
}
