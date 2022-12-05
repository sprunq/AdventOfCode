use std::time::Instant;

pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;

fn main() {
    let now = Instant::now();
    aoc5::p2();
    println!("\n{:?}", now.elapsed())
}
