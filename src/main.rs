use dialoguer::{theme::ColorfulTheme, Select};
use std::{fs, time::Instant};

pub mod calendar;
pub mod util;

pub use util::*;

const BASE_PATH: &str = "/home/mizkyosia/Projects/aoc2024/src/inputs/day";

fn main() {
    let items: Vec<String> = (1..=5).map(|x| format!("Day {:02}", x)).collect();

    let mut day: Option<usize> = None;
    let mut first = true;

    loop {
        print!("\x1B[2J\x1B[1;1H");

        if let Some(day) = day {
            let day = day as u8 + 1;

            let func = get_day(day);

            let path = BASE_PATH.to_owned() + &day.to_string() + ".txt";

            let input = fs::read_to_string(path).unwrap();

            let start = Instant::now();
            let sol = func(input);
            println!("\n=== Day {:02} ===", day);
            println!("  · Part 1: {}", sol.0);
            println!("  · Part 2: {}", sol.1);
            println!("  · Time : {:?}", Instant::now() - start);
        } else if !first {
            break;
        }

        first = false;

        day = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("\n\nChoose a day (or q to exit) :")
            .items(&items)
            .interact_opt()
            .unwrap();
    }
    println!("Error :(");
}

fn get_day(day: u8) -> AoCFunction {
    match day {
        1 => calendar::day1::solve,
        2 => calendar::day2::solve,
        3 => calendar::day3::solve,
        4 => calendar::day4::solve,
        5 => calendar::day5::solve,
        6 => calendar::day6::solve,
        7 => calendar::day7::solve,
        8 => calendar::day8::solve,
        9 => calendar::day9::solve,
        10 => calendar::day10::solve,
        11 => calendar::day11::solve,
        12 => calendar::day12::solve,
        13 => calendar::day13::solve,
        14 => calendar::day14::solve,
        15 => calendar::day15::solve,
        16 => calendar::day16::solve,
        17 => calendar::day17::solve,
        18 => calendar::day18::solve,
        19 => calendar::day19::solve,
        20 => calendar::day20::solve,
        21 => calendar::day21::solve,
        22 => calendar::day22::solve,
        23 => calendar::day23::solve,
        24 => calendar::day24::solve,

        _ => unimplemented!(),
    }
}
