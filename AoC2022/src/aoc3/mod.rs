use crate::Part;

#[derive(Default)]
pub struct Day3 {}

impl Part for Day3 {
    fn p1(&self) -> String {
        p1()
    }

    fn p2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let sum = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            b.iter()
                .filter(|b| a.contains(b))
                .map(|b| {
                    if *b >= b'a' {
                        (b - b'a') as i16 + 1
                    } else {
                        (b - b'A') as i16 + 27
                    }
                })
                .next()
                .unwrap()
        })
        .sum::<i16>();

    format!("{}", sum)
}

pub fn p2() -> String {
    let matches = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .collect::<Vec<_>>();

    let sum: u16 = matches
        .chunks(3)
        .map(|chunk| {
            let x = chunk[0]
                .iter()
                .find(|f| chunk[1].contains(f) && chunk[2].contains(f))
                .unwrap();

            if *x >= b'a' {
                (x - b'a') as u16 + 1
            } else {
                (x - b'A') as u16 + 27
            }
        })
        .sum();

    format!("{}", sum)
}
