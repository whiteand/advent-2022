use std::collections::BTreeSet;

use state::State;

mod moves;
mod parse;
mod state;

fn solve(file_content: &str, size: usize) -> usize {
    State::new(size)
        .get_path(parse::parse_moves(file_content))
        .map(|s| s.tail())
        .collect::<BTreeSet<_>>()
        .len()
}

pub fn solve_task1(file_content: &str) -> usize {
    solve(file_content, 2)
}
pub fn solve_task2(file_content: &str) -> usize {
    solve(file_content, 10)
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
    #[ignore]
    fn test_y22_d9_t1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "13");
    }
    #[test]
    #[ignore]
    fn test_y22_d9_t1_actual() {
        let str = fs::read_to_string("./benches/y22d9.txt").unwrap_or_default();
        let res = solve_task1(&str);
        assert_eq!(res, 6067);
    }
    #[test]
    #[ignore]
    fn test_y22_d9_t2_actual() {
        let str = fs::read_to_string("./benches/y22d9.txt").unwrap_or_default();
        let res = solve_task2(&str);
        assert_eq!(res, 2471);
    }
    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(
            format!(
                "{}",
                solve_task2(
                    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                )
            ),
            "36"
        );
    }
    #[test]
    #[ignore]
    fn test_task2_2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "1");
    }
}
