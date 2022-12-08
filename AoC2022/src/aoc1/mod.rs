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
    let res = include_str!("input.txt")
        .split("\n\n")
        .map(|elem| elem.lines().map(|f| f.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
    format!("{:?}", res)
}

pub fn p2() -> String {
    let mut v = include_str!("input.txt")
        .split("\n\n")
        .map(|elem| elem.lines().map(|f| f.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();
    v.sort();
    format!("{:?}", v.into_iter().rev().take(3).sum::<u32>())
}
