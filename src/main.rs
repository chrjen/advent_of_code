use solutions_2015 as s15;

fn main() {
    for solution in s15::SOLUTIONS {
        let (part1, part2) = (solution.solve)(solution.input);
        println!("--- {} ---", solution.name);
        println!("(1) {}", part1);
        println!("(2) {}", part2);
    }
}
