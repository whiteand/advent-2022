use std::collections::HashSet;

use state::State;

mod moves;
mod parse;
mod state;

pub fn solve_task1(file_content: &str) -> usize {
    State::default()
        .get_path(parse::parse_moves(file_content))
        .inspect(|s| println!("{}\n", s))
        .map(|s| s.tail)
        .collect::<HashSet<_>>()
        .len()
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn test_y22_d9_t1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "13");
    }
    #[test]
    fn test_y22_d9_t1_actual() {
        let str = fs::read_to_string("./benches/y22d9.txt").unwrap_or_default();
        let res = solve_task1(&str);
        assert_eq!(res, 0);
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }
}
