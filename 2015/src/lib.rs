pub mod day1;

type Solvers = fn(&[u8]) -> (String, String);
pub const SOLUTIONS: &[Solvers] = &[day1::solve];
