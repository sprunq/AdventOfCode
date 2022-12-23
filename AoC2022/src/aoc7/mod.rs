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
    let mut input = include_str!("input.txt").lines().peekable();
    let mut stack = Vec::<usize>::new();
    let mut sum = 0;
    input.next().unwrap();
    stack.push(0);
    while let Some(l) = input.next().or({
        if stack.len() > 1 {
            Some("$ cd ..")
        } else {
            None
        }
    }) {
        if l.starts_with("$ cd") {
            let (_, dir) = l.split_at(5);
            if dir == ".." {
                let dir_size = stack.pop().unwrap();
                if dir_size <= 100000 {
                    sum += dir_size;
                }
                *stack.last_mut().unwrap() += dir_size;
            } else {
                stack.push(0);
            }
        } else if l.starts_with("$ ls") {
            while !input.peek().unwrap_or(&"$").starts_with('$') {
                let next = input.next().unwrap();
                if !next.starts_with("dir") {
                    let file_size = next.split(' ').next().unwrap();
                    let s = file_size.parse::<usize>().unwrap();
                    *stack.last_mut().unwrap() += s;
                }
            }
        } else {
            panic!()
        }
    }
    format!("{:?}", sum)
}

pub fn p2() -> String {
    let mut input = include_str!("input.txt").lines().peekable();
    let mut stack = Vec::new();
    let mut sizes = Vec::new();
    input.next().unwrap();
    stack.push(0);
    while let Some(l) = input.next().or({
        if stack.len() > 1 {
            Some("$ cd ..")
        } else {
            None
        }
    }) {
        if l.starts_with("$ cd") {
            let (_, dir) = l.split_at(5);
            if dir == ".." {
                let dir_size = stack.pop().unwrap();
                sizes.push(dir_size);
                *stack.last_mut().unwrap() += dir_size;
            } else {
                stack.push(0);
            }
        } else if l.starts_with("$ ls") {
            while !input.peek().unwrap_or(&"$").starts_with('$') {
                let next = input.next().unwrap();
                if !next.starts_with("dir") {
                    let file_size = next.split(' ').next().unwrap();
                    let s = file_size.parse::<usize>().unwrap();
                    *stack.last_mut().unwrap() += s;
                }
            }
        } else {
            panic!()
        }
    }
    format!(
        "{}",
        sizes
            .iter()
            .filter(|&dir| dir >= &(30000000 - (70000000 - stack[0])))
            .min()
            .unwrap()
    )
}
