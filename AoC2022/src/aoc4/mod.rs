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
    let matches = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let l = first.split_once('-').unwrap();
            let r = second.split_once('-').unwrap();
            (
                l.0.parse::<u8>().unwrap(),
                l.1.parse::<u8>().unwrap(),
                r.0.parse::<u8>().unwrap(),
                r.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(x1, x2, y1, y2)| (x1 >= y1 && x2 <= y2) || (x1 <= y1 && x2 >= y2))
        .count();

    format!("{:?}", matches)
}

pub fn p2() -> String {
    let matches = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let l = first.split_once('-').unwrap();
            let r = second.split_once('-').unwrap();
            (
                l.0.parse::<u8>().unwrap(),
                l.1.parse::<u8>().unwrap(),
                r.0.parse::<u8>().unwrap(),
                r.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(x1, x2, y1, y2)| (x1 <= y2) && (y1 <= x2))
        .count();

    format!("{:?}", matches)
}
