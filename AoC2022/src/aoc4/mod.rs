pub fn p1() {
    let matches = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let f_r = first.split_once('-').unwrap();
            let s_r = second.split_once('-').unwrap();
            (
                f_r.0.parse::<u8>().unwrap(),
                f_r.1.parse::<u8>().unwrap(),
                s_r.0.parse::<u8>().unwrap(),
                s_r.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(x1, x2, y1, y2)| (x1 >= y1 && x2 <= y2) || (x1 <= y1 && x2 >= y2))
        .count();

    println!("{:?}", matches);
}

pub fn p2() {
    let matches = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let f_r = first.split_once('-').unwrap();
            let s_r = second.split_once('-').unwrap();
            (
                f_r.0.parse::<u8>().unwrap(),
                f_r.1.parse::<u8>().unwrap(),
                s_r.0.parse::<u8>().unwrap(),
                s_r.1.parse::<u8>().unwrap(),
            )
        })
        .filter(|(x1, x2, y1, y2)| (x1 <= y2) && (y1 <= x2))
        .count();

    println!("{:?}", matches);
}
