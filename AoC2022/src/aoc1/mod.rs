use std::fs;

pub fn p1() {
    let input = fs::read_to_string("src\\aoc1\\input.txt").expect("Cannot find file");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut int_vec = vec![];
    for elem in split {
        let clean: Vec<_> = elem.split('\n').collect();
        let ints = clean
            .iter()
            .map(|f| f.parse::<i64>().unwrap_or(0))
            .sum::<i64>();
        int_vec.push(ints);
    }
    println!("{:?}", int_vec.iter().max().unwrap());
}

pub fn p2() {
    let input = fs::read_to_string("src\\aoc1\\input.txt").expect("Cannot find file");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut int_vec = vec![];
    for elem in split {
        let clean: Vec<_> = elem.split('\n').collect();
        let ints = clean
            .iter()
            .map(|f| f.parse::<i64>().unwrap_or(0))
            .sum::<i64>();
        int_vec.push(ints);
    }
    int_vec.sort();
    println!("{:?}", int_vec.iter().rev().take(3).sum::<i64>());
}
