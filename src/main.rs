use std::{fs, time::Instant};
use dialoguer::{theme::ColorfulTheme, Select};

pub mod calendar;
pub mod util;

pub use util::*;

const BASE_PATH:&str = "/home/mizkyosia/Projects/aoc2024/src/inputs/day";

fn main() {
    let items: Vec<String> = (1..=2).map(|x| format!("Day {:02}", x)).collect();

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

        day = Select::with_theme(&ColorfulTheme::default()).with_prompt("\n\nChoose a day (or q to exit) :").items(&items).interact_opt().unwrap();   
    }
    println!("Error :(");

}

fn get_day(day:u8) -> AoCFunction {
    match day {
        1 => calendar::day1::solve,
        2 => calendar::day2::solve,
        _ => unimplemented!()
    }
}