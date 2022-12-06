use crate::Part;

#[derive(Default)]
pub struct Day4 {}

impl Part for Day4 {
    fn p1(&self) -> String {
        p1()
    }

    fn p2(&self) -> String {
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
