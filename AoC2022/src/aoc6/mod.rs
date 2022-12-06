pub fn p1() {
    let input = include_bytes!("input.txt");
    print!("{}", solve(input, 4));
}

pub fn p2() {
    let input = include_bytes!("input.txt");
    print!("{}", solve(input, 14));
}

#[inline]
fn solve(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .position(|w| {
            w.iter()
                .enumerate()
                .all(|(idx, c)| w[idx + 1..].contains(c) == false)
        })
        .unwrap()
        + window_size
}
