mod parse;
mod part1;
mod step;
mod valve;

pub fn solve_task1(file_content: &str, minutes: usize) -> usize {
    part1::solve_task1(file_content, minutes)
}
pub fn solve_task2(file_content: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d16/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d16.txt");
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT, 30)), "1651");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL, 30)), "1728");
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
