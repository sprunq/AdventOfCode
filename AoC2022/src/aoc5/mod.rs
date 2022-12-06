use crate::Part;

#[derive(Default)]
pub struct Day5 {}

impl Part for Day5 {
    fn p1(&self) -> String {
        p1()
    }

    fn p2(&self) -> String {
        p2()
    }
}

pub fn p1() -> String {
    let mut boxes = vec![Vec::<u8>::new(); 9];
    include_bytes!("boxes.txt")
        .split(|b| b == &b'\n')
        .for_each(|line| {
            line.iter().enumerate().for_each(|(idx, c)| {
                if c.is_ascii_alphabetic() {
                    boxes[idx / 4].insert(0, *c)
                }
            });
        });

    let moves = include_str!("moves.txt")
        .lines()
        .map(|a| a.split(' '))
        .map(|b| {
            let mut ret: [usize; 3] = [0; 3];
            for (idx, p) in b.enumerate() {
                if let Ok(val) = p.parse::<usize>() {
                    ret[idx / 2] = val
                }
            }
            ret
        })
        .collect::<Vec<_>>();

    for mov in moves {
        let [amount, from, to] = mov;
        for _ in 0..amount {
            let val = boxes[from - 1].pop().unwrap();
            boxes[to - 1].push(val);
        }
    }

    boxes
        .iter()
        .map(|x| format!("{}", x.last().unwrap()))
        .collect::<String>()
}

/// Optimized for very big inputs
/// https://www.reddit.com/r/adventofcode/comments/zd1hqy/comment/iyzvsnp/?utm_source=share&utm_medium=web2x&context=3
/// 6mb and 88mb input
/// 6mb: 374ms
/// 88mb: 236s (1.500.000 moves)
pub fn p2() -> String {
    let mut boxes = vec![Vec::<u8>::new(); 9];
    let mut swap_buffer = vec![0u8; 100_000_000];

    include_bytes!("boxes.txt")
        .split(|b| b == &b'\n')
        .for_each(|line| {
            line.iter().enumerate().for_each(|(idx, c)| {
                if c.is_ascii_alphabetic() {
                    boxes[idx / 4].push(*c)
                }
            });
        });
    for row in &mut boxes {
        row.reverse();
    }

    let moves = include_str!("moves.txt")
        .lines()
        .map(|a| a.split(' '))
        .map(|b| {
            let mut ret: [usize; 3] = [0; 3];
            for (idx, p) in b.enumerate() {
                if let Ok(val) = p.parse::<usize>() {
                    ret[idx / 2] = val
                }
            }
            ret
        })
        .collect::<Vec<_>>();

    for mov in moves.iter() {
        let [amount, from, to] = mov;
        let len_to_swap = boxes[from - 1].len();
        let swap_buffer = &mut swap_buffer[0..*amount];
        swap_buffer.copy_from_slice(&boxes[from - 1][len_to_swap - amount..len_to_swap]);
        boxes[from - 1].truncate(len_to_swap - amount);
        boxes[to - 1].extend(swap_buffer.iter());
    }

    boxes
        .iter()
        .map(|x| format!("{}", *x.last().unwrap() as char))
        .collect::<String>()
}
