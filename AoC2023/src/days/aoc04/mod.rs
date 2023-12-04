crate::AocDay!(4);

const INPUT: &[u8] = include_bytes!("input.txt");

#[inline(always)]
pub fn part_1() -> String {
    let mut score = 0;
    for line in INPUT.split(|b| *b == b'\n') {
        let winning_num = parse_numbers::<10>(&line[10..39]);
        let guessed_num = parse_numbers::<25>(&line[42..116]);

        let correct = get_correct_score(winning_num, guessed_num);

        if correct > 0 {
            let line_score = 2_i32.pow(correct as u32 - 1);
            score += line_score;
        }
    }

    format!("{}", score)
}

#[inline(always)]
pub fn part_2() -> String {
    let lines = INPUT.split(|b| *b == b'\n').collect::<Vec<_>>();
    let mut copies = vec![1; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let winning_num = parse_numbers::<10>(&line[10..39]);
        let guessed_num = parse_numbers::<25>(&line[42..116]);

        let correct = get_correct_score(winning_num, guessed_num);

        for underlying_card_idx in idx + 1..idx + 1 + correct {
            copies[underlying_card_idx] += copies[idx];
        }
    }

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
