use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use common::Solution;
use crossterm::execute;
use crossterm::style::{Color, SetForegroundColor};

use solutions_2015 as s15;

const ALL_YEARS: [u32; 1] = [2015];

fn solutions_for_year(year: u32) -> Option<&'static [Solution<'static>]> {
    match year {
        2015 => Some(s15::SOLUTIONS),
        _ => None,
    }
}

/// Advent of Code solutions by Christer Jensen <chr.code@gmail.com>
///
/// Advent of Code is an Advent calendar of small programming puzzles for a
/// variety of skill sets and skill levels that can be solved in any
/// programming language you like.
/// Link: https://adventofcode.com/
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Year to display solutions. Default all years.
    year: Option<u32>,

    /// Day to display solutions. Default all days.
    day: Option<u32>,

    /// Custom solution input.
    input: Option<PathBuf>,
}

fn main() {
    let mut stdout = io::stdout();
    let args = Args::parse();

    if let Some(year) = args.year {
        run_specific_year(year, args.day, args.input);
        exit(0);
    }

    for year in ALL_YEARS {
        execute!(stdout, SetForegroundColor(Color::Cyan)).ok();
        println!("------ YEARS {} ------", year);
        execute!(stdout, SetForegroundColor(Color::Reset)).ok();

        run_specific_year(year, None, None);
    }
}

fn run_specific_year(year: u32, day: Option<u32>, input: Option<PathBuf>) {
    if let Some(solutions) = solutions_for_year(year) {
        if let Some(day) = day {
            run_specific_day(solutions, day, input);
        }

        for solution in solutions {
            println_solution(solution);
        }
    } else {
        eprintln!("solutions for year {year} not found.");
    }
}

fn run_specific_day(solutions: &[Solution], day: u32, path: Option<PathBuf>) -> ! {
    let idx = (day - 1) as usize;

    if (0..solutions.len()).contains(&idx) {
        let mut input: &[u8] = solutions[idx].input;
        let mut buf = Vec::new();

        if let Some(path) = path {
            // Read in custom input from user.
            if path == PathBuf::from("-") {
                // Read input from stdin.
                let result = io::stdin().read_to_end(&mut buf);
                if result.is_err() {
                    println!("failed to read input file '{}'", &path.to_string_lossy());
                    exit(1);
                }
            } else {
                // Read input from file.
                let file = File::open(&path);
                if file.is_err() {
                    println!("failed to open input file '{}'", &path.to_string_lossy());
                    exit(1);
                }

                let result = file.unwrap().read_to_end(&mut buf);
                if result.is_err() {
                    println!("failed to read input file '{}'", &path.to_string_lossy());
                    exit(1);
                }
            }
            input = &buf;
        }

        println_solution_with_input(&solutions[idx], input);
        exit(0);
    } else {
        eprintln!("solution for day {day} not found.");
        exit(2);
    }
}

fn println_solution(solution: &Solution) {
    println_solution_with_input(solution, solution.input);
}

fn println_solution_with_input(solution: &Solution, input: &[u8]) {
    let mut stdout = io::stdout();

    let (part1, part2) = (solution.solve)(input);
    execute!(stdout, SetForegroundColor(Color::Green)).ok();
    println!("--- {} ---", solution.name);
    execute!(stdout, SetForegroundColor(Color::White)).ok();
    println!("(1) {}", part1);
    println!("(2) {}", part2);
    execute!(stdout, SetForegroundColor(Color::Reset)).ok();
}
