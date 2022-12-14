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
    let input = include_bytes!("input.txt");
    format!("{}", solve(input, 4))
}

pub fn p2() -> String {
    let input = include_bytes!("input.txt");
    format!("{}", solve(input, 14))
}

#[inline]
fn solve(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .position(|w| {
            w.iter()
                .enumerate()
                .all(|(idx, c)| !w[idx + 1..].contains(c))
        })
        .unwrap()
        + window_size
}
