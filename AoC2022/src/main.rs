use std::time::Instant;

pub mod aoc1;
pub mod aoc2;
pub mod aoc3;

fn main() {
    let now = Instant::now();
    aoc3::p2();
    println!("{:?}", now.elapsed())
}
