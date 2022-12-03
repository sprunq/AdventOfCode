use std::fs;

pub fn p1() {
    let input = fs::read_to_string("src\\aoc3\\input.txt").expect("Cannot find file");
    let matches: Vec<(&str, &str)> = input.split('\n').map(|f| f.split_at(f.len() / 2)).collect();

    let mut sum = 0;
    for str_tup in &matches {
        let mut b0 = str_tup.0.as_bytes().to_vec();
        let mut b1 = str_tup.1.as_bytes().to_vec();
        b0.sort();
        b0.dedup();
        b1.sort();
        b1.dedup();

        'outer: for c1 in &b0 {
            for c2 in &b1 {
                if c1 == c2 {
                    let ch = *c1 as char;
                    let mut char_prio = ch.to_digit(36).unwrap() - 9;
                    if ch.is_uppercase() {
                        char_prio += 26;
                    }
                    sum += char_prio;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", sum);
}

pub fn p2() {
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

    println!("{}", sum);
}
