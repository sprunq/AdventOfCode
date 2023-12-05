pub mod days;

use crate::days::*;
use clap::{command, value_parser, Arg, ArgAction};
use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Table,
};
use std::{
    ops::Range,
    time::{Duration, Instant},
    vec,
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
        run_benchmark_all(1..26, 10000);
    } else {
        run_one_part(matches);
    }
}

pub trait Solution {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

#[macro_export]
macro_rules! AocDay {
    ($day:expr) => {
        use paste::paste;
        use $crate::Solution;

        paste! {
                #[derive(Default)]
                pub struct [<AocDay $day>];

                impl Solution for [<AocDay $day>] {
                    fn part_1(&self) -> String {
                        part_1()
                    }

                    fn part_2(&self) -> String {
                        part_2()
                    }
            }
        }
    };
}

fn get_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(aoc01::AocDay1)),
        2 => Some(Box::new(aoc02::AocDay2)),
        3 => Some(Box::new(aoc03::AocDay3)),
        4 => Some(Box::new(aoc04::AocDay4)),
        5 => Some(Box::new(aoc05::AocDay5)),
        _ => None,
    }
}

fn run_part(part: usize, day_ex: Box<dyn Solution>) -> String {
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
    let result = run_part(part, day_ex.unwrap_or_else(|| panic!("Invalid Day")));
    let elapsed = now.elapsed();
    println!("Result:\t{}", result);
    println!("Time:  \t{:?}", elapsed)
}

fn run_benchmark_all(days: Range<usize>, runs_per_part: u32) {
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
        if let Some(part) = part {
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
                format!("{:.1}µs", p1_elapsed.as_secs_f64() * 1000000.0),
                format!("{:.1}µs", p2_elapsed.as_secs_f64() * 1000000.0),
                p1_elapsed + p2_elapsed,
            ))
        }
    }

    let combined_duration = total_duration_p1 + total_duration_p2;
    for data in time_data {
        let percentage = data.3.as_secs_f64() / combined_duration.as_secs_f64() * 100.0;
        table.add_row(vec![
            data.0,
            data.1,
            data.2,
            format!("{:.3}ms", data.3.as_secs_f64() * 1000.0),
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
