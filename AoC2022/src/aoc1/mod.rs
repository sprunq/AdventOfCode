use crate::Part;

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
