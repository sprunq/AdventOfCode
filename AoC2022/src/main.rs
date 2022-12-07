use aoc1::Day1;
use aoc2::Day2;
use aoc3::Day3;
use aoc4::Day4;
use aoc5::Day5;
use aoc6::Day6;
use clap::{command, value_parser, Arg, ArgAction};
use std::{
    ops::RangeInclusive,
    time::{Duration, Instant},
    vec,
};
pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
pub mod aoc7;
use crate::aoc7::Day7;
use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Table,
};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("part")
                .short('p')
                .long("part")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set),
        )
        .get_matches();

    if matches.get_flag("benchmark") {
        run_benchmark_all(1..=7, 1000);
    } else {
        run_one_part(matches);
    }
}

pub trait AocDay {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

fn get_day(day: usize) -> Box<dyn AocDay> {
    match day {
        1 => Box::new(Day1::default()),
        2 => Box::new(Day2::default()),
        3 => Box::new(Day3::default()),
        4 => Box::new(Day4::default()),
        5 => Box::new(Day5::default()),
        6 => Box::new(Day6::default()),
        7 => Box::new(Day7::default()),
        _ => panic!("Invalid day"),
    }
}

fn run_part(part: usize, day_ex: Box<dyn AocDay>) -> String {
    match part {
        1 => day_ex.part_1(),
        2 => day_ex.part_2(),
        _ => panic!("Invalid part"),
    }
}

fn run_one_part(matches: clap::ArgMatches) {
    let day: usize = *matches.get_one("day").expect("day is required");
    let day_ex = get_day(day);
    let part: usize = *matches.get_one("part").expect("part is required");
    let now = Instant::now();
    let result = run_part(part, day_ex);
    let elapsed = now.elapsed();
    println!("Result:\t{}", result);
    println!("Time:  \t{:?}", elapsed)
}

fn run_benchmark_all(days: RangeInclusive<usize>, runs_per_part: u32) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "Part 1", "Part 2", "Combined", "Relative"]);
    let mut total_duration_p1 = Duration::new(0, 0);
    let mut total_duration_p2 = Duration::new(0, 0);
    let mut time_data = Vec::new();
    let all_days = days.map(get_day).collect::<Vec<_>>();
    for (day, part) in all_days.iter().enumerate() {
        let now1 = Instant::now();
        for _ in 0..runs_per_part {
            part.part_1();
        }
        let p1_elapsed = now1.elapsed() / runs_per_part;
        let now2 = Instant::now();
        for _ in 0..runs_per_part {
            part.part_2();
        }
        let p2_elapsed = now2.elapsed() / runs_per_part;
        total_duration_p1 += p1_elapsed;
        total_duration_p2 += p2_elapsed;
        time_data.push((
            format!("{}", day + 1),
            format!("{:?}", p1_elapsed),
            format!("{:?}", p2_elapsed),
            p1_elapsed + p2_elapsed,
        ))
    }

    let combined_duration = total_duration_p1 + total_duration_p2;
    for data in time_data {
        let percentage = data.3.as_secs_f64() / combined_duration.as_secs_f64() * 100.0;
        table.add_row(vec![
            data.0,
            data.1,
            data.2,
            format!("{:#?}", data.3),
            format!("{:.1}%", percentage),
        ]);
    }

    table.add_row(vec![
        format!("Total"),
        format!("{:?}", total_duration_p1),
        format!("{:?}", total_duration_p2),
        format!("{:?}", combined_duration),
        "100%".to_string(),
    ]);

    println!("\n{table}");
}
