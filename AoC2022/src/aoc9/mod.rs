use crate::AocDay;
use std::collections::HashSet;

#[derive(Default)]
pub struct Parts {}

impl AocDay for Parts {
    fn part_1(&self) -> String {
        solve_rope(2)
    }

    fn part_2(&self) -> String {
        solve_rope(10)
    }
}

fn solve_rope(len: usize) -> String {
    let moves = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|f| {
            let moves = atoi::atoi(&f[2..]).unwrap();
            (f[0], moves)
        })
        .collect::<Vec<(u8, u8)>>();

    let mut visited = HashSet::new();
    let mut rope = vec![(0, 0); len];
    visited.insert((0, 0));

    for mov in moves {
        for _ in 0..mov.1 {
            // update head pos
            match mov.0 {
                b'L' => rope[0].0 -= 1,
                b'R' => rope[0].0 += 1,
                b'U' => rope[0].1 += 1,
                b'D' => rope[0].1 -= 1,
                _ => panic!(),
            };

            // update tail pos
            let mut last = rope[0];
            rope.iter_mut().skip(1).for_each(|k| {
                let dx: i32 = last.0 - k.0;
                let dy: i32 = last.1 - k.1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    k.0 += dx.signum();
                    k.1 += dy.signum();
                }
                last = *k;
            });
            visited.insert(*rope.last().unwrap());
        }
    }

    format!("{:?}", visited.len())
}
