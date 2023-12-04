crate::AocDay!(4);

const INPUT: &str = include_str!("input.txt");

#[inline(always)]
pub fn part_1() -> String {
    let score: u32 = INPUT
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let winning_num = parse_numbers::<10>(&bytes[10..39]);
            let guessed_num = parse_numbers::<25>(&bytes[42..116]);
            let correct = get_correct_score(winning_num, guessed_num);
            (2 << correct - 1) >> 1
        })
        .sum();

    format!("{}", score)
}

#[inline(always)]
pub fn part_2() -> String {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let mut copies = vec![1; lines.len()];
    lines.iter().enumerate().for_each(|(idx, line)| {
        let bytes = line.as_bytes();
        let winning_num = parse_numbers::<10>(&bytes[10..39]);
        let guessed_num = parse_numbers::<25>(&bytes[42..116]);
        let correct = get_correct_score(winning_num, guessed_num);

        for underlying_card_idx in idx + 1..idx + 1 + correct {
            copies[underlying_card_idx] += copies[idx];
        }
    });

    let sum = copies.iter().sum::<u64>();
    format!("{}", sum)
}

#[inline(always)]
fn get_correct_score<const N1: usize, const N2: usize>(
    winning_num: [u8; N1],
    guessed_num: [u8; N2],
) -> usize {
    let mut correct = 0;
    for i in 0..N1 {
        for j in 0..N2 {
            if winning_num[i] == guessed_num[j] {
                correct += 1;
            }
        }
    }
    correct
}

#[inline(always)]
fn parse_numbers<const N: usize>(input: &[u8]) -> [u8; N] {
    let mut numbers = [0; N];
    for i in 0..N {
        let mut c0 = input[i * 3];
        if c0 == b' ' {
            c0 = b'0';
        }
        let c1 = input[i * 3 + 1];
        let n = (c0 - b'0') * 10 + (c1 - b'0');
        numbers[i] = n;
    }
    numbers
}
