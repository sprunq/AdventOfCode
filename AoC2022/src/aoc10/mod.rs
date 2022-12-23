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
    let mut register = 1;
    let mut cycle = 1;
    let mut sig_stengths: i32 = 0;
    include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .for_each(|l| {
            if (cycle + 20) % 40 == 0 {
                sig_stengths += cycle * register;
            }
            cycle += 1;
            if &l[0..4] == b"addx" {
                if (cycle + 20) % 40 == 0 {
                    sig_stengths += cycle * register;
                }
                cycle += 1;
                register += atoi::atoi::<i8>(&l[5..]).unwrap() as i32;
            }
        });

    format!("{:?}", sig_stengths)
}

pub fn p2() -> String {
    let mut register = 1;
    let mut cycle = 0;
    let mut pixels = vec![false; 240];
    include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .for_each(|l| {
            if register - 1 <= cycle % 40 && register + 1 >= cycle % 40 {
                pixels[cycle as usize] = true;
            }
            cycle += 1;
            if &l[0..4] == b"addx" {
                if register - 1 <= cycle % 40 && register + 1 >= cycle % 40 {
                    pixels[cycle as usize] = true;
                }
                cycle += 1;
                register += atoi::atoi::<i8>(&l[5..]).unwrap() as i32;
            }
        });

    format!(
        "\n{}",
        pixels
            .chunks(40)
            .map(|c| c
                .iter()
                .map(|b| if *b { "#" } else { "." })
                .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
