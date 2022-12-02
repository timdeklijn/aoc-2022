use std::{env, fs, io, path::Path};

mod aoc_trait;

// Import all days. Each day adheres to the `AocDay` trait which
// helps using all 'days' as inputs for `run`.
mod day0;
mod day1;
mod day2;

use aoc_trait::AocDay;

// Use these consts to give color + bold to print statements.
const RESET: &str = "\u{001b}[0m";
const GREEN: &str = "\u{001b}[32m";
const MAGENTA: &str = "\u{001b}[35m";
const RED: &str = "\u{001b}[31m";
const BOLD: &str = "\u{001b}[1m";

/// Create data path using the day number. Then read the file and
/// return the result of reading the file.
fn read_data(num: usize) -> Result<String, io::Error> {
    let path = format!("data/day_{num}.txt");
    let path = Path::new(&path);
    fs::read_to_string(path)
}

/// Run part one of the AocDay input.
fn runner(day: &Box<dyn AocDay>, part: usize) {
    println!("{BOLD}{MAGENTA}==== Running Part {part} ===={RESET}");

    // Run the test for a specific part
    let (answer, ok) = match part {
        1 => day.test_part_1(),
        2 => day.test_part_2(),
        _ => unreachable!(),
    };

    // If the test has failed, print the result, and return
    if !ok {
        println!("{BOLD}{RED}- part 1 not done yet: {RESET}{BOLD}{answer}{RESET}");
        return;
    }
    println!("{BOLD}{GREEN}- Example works{RESET}");

    // Load the data
    let data = match read_data(day.num()) {
        Ok(d) => d,
        Err(e) => panic!("error reading data: {}", e),
    };

    // Run the solution for the part.
    let answer = match part {
        1 => day.part_1(data),
        2 => day.part_2(data),
        _ => unreachable!(),
    };

    // Print the answer
    println!("part {part}: {BOLD}{answer}{RESET}");
}

/// Depending on `part` run part 1, part 2 or both parts of the solution.
fn run(day: Box<dyn AocDay>, part: usize) {
    match part {
        0 => {
            runner(&day, 1);
            runner(&day, 2);
        }
        1 => runner(&day, 1),
        2 => runner(&day, 2),
        _ => unreachable!(),
    }
}

/// Parse cli args and run the day/part that has been selected.
fn main() {
    // Match the cli arg to a Day struct
    let day: Box<dyn AocDay> = match env::args().nth(1).expect("This should work").as_str() {
        "0" => Box::new(day0::Day {}),
        "1" => Box::new(day1::Day {}),
        "2" => Box::new(day2::Day {}),
        _ => panic!("Do not forget to add a day number to the main function"),
    };

    // Match the cli arg to a part. If nothing is found, run both.
    let part = match env::args().nth(2) {
        Some(n) => match n.as_str() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            _ => panic!("Unexpected input for part"),
        },
        None => 0,
    };

    // Show some intro and run the command
    let day_num = day.num();
    println!("\n===============================================================================");
    println!("{BOLD}DAY {day_num}{RESET}");
    println!("===============================================================================");
    run(day, part);
    println!("===============================================================================");
}
