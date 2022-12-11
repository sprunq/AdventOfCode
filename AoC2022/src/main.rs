use clap::{command, value_parser, Arg, ArgAction};
use std::{
    ops::Range,
    time::{Duration, Instant},
    vec,
};
pub mod aoc1;
pub mod aoc10;
pub mod aoc11;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
pub mod aoc7;
pub mod aoc8;
pub mod aoc9;
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
        run_benchmark_all(0..27, 1000);
    } else {
        run_one_part(matches);
    }
}

pub trait AocDay {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

fn get_day(day: usize) -> Option<Box<dyn AocDay>> {
    match day {
        1 => Some(Box::new(aoc1::Parts::default())),
        2 => Some(Box::new(aoc2::Parts::default())),
        3 => Some(Box::new(aoc3::Parts::default())),
        4 => Some(Box::new(aoc4::Parts::default())),
        5 => Some(Box::new(aoc5::Parts::default())),
        6 => Some(Box::new(aoc6::Parts::default())),
        7 => Some(Box::new(aoc7::Parts::default())),
        8 => Some(Box::new(aoc8::Parts::default())),
        9 => Some(Box::new(aoc9::Parts::default())),
        10 => Some(Box::new(aoc10::Parts::default())),
        11 => Some(Box::new(aoc11::Parts::default())),
        _ => None,
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
                format!("{}", day),
                format!("{:?}", p1_elapsed),
                format!("{:?}", p2_elapsed),
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
