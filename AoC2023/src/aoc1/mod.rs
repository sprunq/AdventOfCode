use crate::Solution;
use itertools::Itertools;

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
            let first = line.iter().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.iter().rfind(|c| c.is_ascii_digit()).unwrap();
            (first - b'0') as u32 * 10 + (last - b'0') as u32
        })
        .sum();

    format!("{}", sum)
}

pub fn p2() -> String {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for mut line in lines {
        let first = 'out: loop {
            for (word, num) in NUMBER_MAP.iter() {
                if line.starts_with(word) {
                    break 'out *num as u32;
                }
            }
            line = &line[1..];
        };

        let last = 'out: loop {
            for (word, num) in NUMBER_MAP.iter() {
                if line.ends_with(word) {
                    break 'out *num as u32;
                }
            }
            line = &line[..line.len() - 1];
        };
        sum += first * 10 + last;
    }

    format!("{}", sum)
}

const NUMBER_MAP: [(&str, u8); 20] = [
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
