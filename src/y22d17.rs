mod figure;
mod get_figures;

use get_figures::get_figures;

pub fn solve_task1<const W: usize>(file_content: &str) -> usize {
    let figures = get_figures();
    dbg!(W, figures);
    0
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d17/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d17.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1::<7>(INPUT)), "3068");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1::<7>(ACTUAL)), "0");
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
