mod parse;
mod valve;

use parse::parse;
// use valve::Valve;

pub fn solve_task1(file_content: &str, minutes: u32) -> usize {
    let valves = parse(file_content);
    dbg!(valves);
    0
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d16/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d16.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT, 30)), "1651");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL, 30)), "0");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
