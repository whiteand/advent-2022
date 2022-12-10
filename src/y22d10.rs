mod command;
mod cpu;
mod crt;
mod parse;

use cpu::CPU;
use crt::CRT;
use parse::parse_commands;

pub fn solve_task1(file_content: &str) -> i32 {
    CPU::new(parse_commands(file_content))
        .enumerate()
        .filter(|(cycle, _)| (*cycle + 1) % 40 == 20)
        .map(|(cycle, register)| ((cycle + 1) as i32) * register)
        .sum()
}

pub fn solve_task2(file_content: &str) {
    print!(
        "{}",
        CPU::new(parse_commands(file_content))
            .scan(CRT::new(), |c, r| Some(c.draw(r)))
            .collect::<String>()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "13140");
    }
    use std::fs;
    #[test]
    fn test_task1_actual() {
        let str = fs::read_to_string("./benches/y22d10.txt").unwrap_or_default();

        assert_eq!(format!("{}", solve_task1(&str)), "14060");
    }

    #[test]
    fn test_task1_small() {
        assert_eq!(
            format!(
                "{}",
                solve_task1(
                    "noop
addx 3
addx -5"
                )
            ),
            "0"
        );
    }
}
