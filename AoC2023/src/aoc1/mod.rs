use std::fmt::format;

use itertools::Itertools;

use crate::Solution;

#[derive(Default)]
pub struct Parts {}

impl Solution for Parts {
    fn part_1(&self) -> String {
        p1()
    }

    fn part_2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let res = include_bytes!("input.txt")
        .split(|&c| c == b'\n')
        .collect_vec();

    let sum: u32 = res
        .into_iter()
        .map(|line| {
            let first = line
                .iter()
                .find(|c| c.is_ascii_digit())
                .map(|c| *c - b'0')
                .unwrap();
            let last = line
                .iter()
                .rfind(|c| c.is_ascii_digit())
                .map(|c| *c - b'0')
                .unwrap();
            first as u32 * 10 + last as u32
        })
        .sum();

    format!("{}", sum)
}

pub fn p2() -> String {
    let res = include_str!("input.txt").lines();

    let mut sum = 0;
    for mut line in res {
        let mut first = None;
        let mut last = None;
        loop {
            for (word, num) in NUMBER_MAP.iter() {
                if first.is_none() && line.starts_with(word) {
                    first = Some(num);
                }

                if last.is_none() && line.ends_with(word) {
                    last = Some(num);
                }
            }
            match (first, last) {
                (Some(_), Some(_)) => break,
                (Some(_), None) => {
                    line = &line[..line.len() - 1];
                }
                (None, Some(_)) => {
                    line = &line[1..];
                }
                (None, None) => {
                    line = &line[1..];
                    line = &line[..line.len() - 1];
                }
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }

    format!("{}", sum)
}

const NUMBER_MAP: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
