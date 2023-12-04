crate::AocDay!(4);

const INPUT: &[u8] = include_bytes!("input.txt");

#[inline(always)]
pub fn part_1() -> String {
    let mut score = 0;
    for line in INPUT.split(|b| *b == b'\n') {
        let winning_num = parse_numbers::<10>(&line[10..39]);
        let guessed_num = parse_numbers::<25>(&line[42..116]);

        let mut correct = 0;
        for i in 0..winning_num.len() {
            for j in 0..guessed_num.len() {
                if winning_num[i] == guessed_num[j] {
                    correct += 1;
                }
            }
        }
        if correct > 0 {
            let line_score = 2_i32.pow(correct - 1);
            score += line_score;
        }
    }

    format!("{}", score)
}

#[inline(always)]
pub fn part_2() -> String {
    let lines = INPUT.split(|b| *b == b'\n').collect::<Vec<_>>();
    let mut copies = vec![0; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let winning_num = parse_numbers::<10>(&line[10..39]);
        let guessed_num = parse_numbers::<25>(&line[42..116]);

        let mut correct = 0;
        for i in 0..winning_num.len() {
            for j in 0..guessed_num.len() {
                if winning_num[i] == guessed_num[j] {
                    correct += 1;
                }
            }
        }

        if correct > 0 {
            let current_copy_count = copies[idx];

            // add copy count to next cards
            for x in idx + 1..idx + correct + 1 {
                copies[x] += 1 * current_copy_count;
            }
        }
    }

    let sum = copies.iter().sum::<u64>();
    format!("{}", sum)
}

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
