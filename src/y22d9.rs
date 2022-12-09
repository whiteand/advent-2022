use std::collections::BTreeSet;

mod moves;
mod parse;

fn follow(tail_x: &mut i32, tail_y: &mut i32, head_x: &i32, head_y: &i32) {
    if (*head_x - *tail_x).abs() <= 1 && (*head_y - *tail_y).abs() <= 1 {
        return;
    }
    let dx = (*head_x - *tail_x).signum();
    let dy = (*head_y - *tail_y).signum();
    *tail_x += dx;
    *tail_y += dy;
}

fn solve<const N: usize>(file_content: &str) -> usize {
    let mut rope_x = [0; N];
    let mut rope_y = [0; N];
    let mut s = BTreeSet::new();

    for mut m in parse::parse_moves(file_content) {
        while !m.is_empty() {
            m.apply(&mut rope_x[0], &mut rope_y[0]);
            for i in 1..N {
                if (rope_x[i - 1] - rope_x[i]).abs() <= 1 && (rope_y[i - 1] - rope_y[i]).abs() <= 1
                {
                    continue;
                }
                let dx = (rope_x[i - 1] - rope_x[i]).signum();
                let dy = (rope_y[i - 1] - rope_y[i]).signum();
                rope_x[i] += dx;
                rope_y[i] += dy;
            }
            let tail_pos_x = *rope_x.last().unwrap();
            let tail_pos_y = *rope_y.last().unwrap();
            s.insert((tail_pos_x, tail_pos_y));
        }
    }
    s.len()
}

pub fn solve_task1(file_content: &str) -> usize {
    solve::<2>(file_content)
}
pub fn solve_task2(file_content: &str) -> usize {
    solve::<10>(file_content)
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
