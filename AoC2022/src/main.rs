use aoc1::Day1;
use aoc2::Day2;
use aoc3::Day3;
use aoc4::Day4;
use aoc5::Day5;
use aoc6::Day6;
use std::{
    time::{Duration, Instant},
    vec,
};
pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

const BENCHMARK: bool = true;

fn main() {
    if BENCHMARK {
        benchmark_all();
    } else {
        let now = Instant::now();
        let part = Day6::default();
        print!("{}", part.p2());
        println!("\n{:?}", now.elapsed())
    }
}

fn benchmark_all() {
    let all_parts: Vec<Box<dyn Part>> = vec![
        Box::new(Day1::default()),
        Box::new(Day2::default()),
        Box::new(Day3::default()),
        Box::new(Day4::default()),
        Box::new(Day5::default()),
        Box::new(Day6::default()),
    ];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "Part 1", "Part 2", "Combined"]);
    let mut total_duration_p1 = Duration::new(0, 0);
    let mut total_duration_p2 = Duration::new(0, 0);
    for (day, part) in all_parts.iter().enumerate() {
        let now = Instant::now();
        part.p1();
        let p1_elapsed = now.elapsed();
        let now = Instant::now();
        part.p2();
        let p2_elapsed = now.elapsed();
        total_duration_p1 += p1_elapsed;
        total_duration_p2 += p2_elapsed;
        table.add_row(vec![
            format!("{day}"),
            format!("{p1_elapsed:?}"),
            format!("{p2_elapsed:?}"),
            format!("{:?}", p1_elapsed + p2_elapsed),
        ]);
    }

    table.add_row(vec![
        format!("Total"),
        format!("{total_duration_p1:?}"),
        format!("{total_duration_p2:?}"),
        format!("{:?}", total_duration_p1 + total_duration_p2),
    ]);

    println!("\n{table}");
}

pub trait Part {
    fn p1(&self) -> String;
    fn p2(&self) -> String;
}
