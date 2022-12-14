use crate::Solution;
use std::{fs, str::FromStr};

#[derive(Default)]
pub struct Parts {}

impl Solution for Parts {
    fn part_1(&self) -> String {
        p1_fast()
    }

    fn part_2(&self) -> String {
        p2_fast()
    }
}

pub fn p1_fast() -> String {
    let score = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16))
        .map(|(opp_move, own_move)| 1 + own_move + 3 * ((1 + own_move - opp_move).rem_euclid(3)))
        .sum::<i16>();
    format!("{}", score)
}

pub fn p2_fast() -> String {
    let score = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16))
        .map(|(opp_move, own_move)| 1 + own_move + 3 * ((2 + own_move + opp_move) % 3))
        .sum::<i16>();
    format!("{}", score)
}

// old solution

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RPS {
    type Err = ();
    fn from_str(input: &str) -> Result<RPS, Self::Err> {
        match input {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MatchResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for MatchResult {
    type Err = ();
    fn from_str(input: &str) -> Result<MatchResult, Self::Err> {
        match input {
            "X" => Ok(MatchResult::Lose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
        }
    }
}

fn match_res(own: RPS, opponent: RPS) -> MatchResult {
    match (own, opponent) {
        (RPS::Rock, RPS::Rock) => MatchResult::Draw,
        (RPS::Rock, RPS::Paper) => MatchResult::Lose,
        (RPS::Rock, RPS::Scissors) => MatchResult::Win,
        (RPS::Paper, RPS::Rock) => MatchResult::Win,
        (RPS::Paper, RPS::Paper) => MatchResult::Draw,
        (RPS::Paper, RPS::Scissors) => MatchResult::Lose,
        (RPS::Scissors, RPS::Rock) => MatchResult::Lose,
        (RPS::Scissors, RPS::Paper) => MatchResult::Win,
        (RPS::Scissors, RPS::Scissors) => MatchResult::Draw,
    }
}

fn move_for_result(opponent: RPS, expected_result: MatchResult) -> RPS {
    match expected_result {
        MatchResult::Lose => match opponent {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        MatchResult::Draw => opponent,
        MatchResult::Win => match opponent {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
    }
}

pub fn p1() -> String {
    let input = fs::read_to_string("src\\aoc2\\input.txt").expect("Cannot find file");
    let matches: Vec<&str> = input.split('\n').collect();
    let mut scores = 0;
    for rps_match in matches {
        let x = rps_match.split(' ').collect::<Vec<_>>();
        assert!(x.len() == 2);
        let my_move = RPS::from_str(x[1]).unwrap();
        let opp_move = RPS::from_str(x[0]).unwrap();
        let res = match_res(my_move, opp_move);
        let score = res as u32 + my_move as u32;
        scores += score;
    }
    format!("{}", scores)
}

pub fn p2() -> String {
    let input = fs::read_to_string("src\\aoc2\\input.txt").expect("Cannot find file");
    let matches: Vec<&str> = input.split('\n').collect();
    let mut scores = 0;
    for rps_match in matches {
        let x = rps_match.split(' ').collect::<Vec<_>>();
        assert!(x.len() == 2);
        let expected_result = MatchResult::from_str(x[1]).unwrap();
        let opp_move = RPS::from_str(x[0]).unwrap();
        let move_to_make = move_for_result(opp_move, expected_result);
        let score = expected_result as u32 + move_to_make as u32;
        scores += score;
    }
    format!("{}", scores)
}
