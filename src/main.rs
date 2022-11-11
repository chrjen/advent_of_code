use std::io;

use crossterm::execute;
use crossterm::style::{Color, SetForegroundColor};

use solutions_2015 as s15;

fn main() {
    let mut stdout = io::stdout();

    for solution in s15::SOLUTIONS {
        let (part1, part2) = (solution.solve)(solution.input);
        execute!(stdout, SetForegroundColor(Color::Green)).ok();
        println!("--- {} ---", solution.name);
        execute!(stdout, SetForegroundColor(Color::White)).ok();
        println!("(1) {}", part1);
        println!("(2) {}", part2);
        execute!(stdout, SetForegroundColor(Color::Reset)).ok();
    }
}
