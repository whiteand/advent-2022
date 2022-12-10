use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(i32),
}

type CpuRegisters = [i32; 1];
const X: usize = 0;
#[derive(Clone, Copy, Debug)]
struct Current {
    command: Command,
    start: usize,
}

struct CPU<Cmds>
where
    Cmds: Iterator<Item = Command>,
{
    commands: Cmds,
    cycle: usize,
    current: Option<Current>,
    registers: CpuRegisters,
}

impl<Cmd> CPU<Cmd>
where
    Cmd: Iterator<Item = Command>,
{
    pub fn new(cmds: Cmd) -> Self {
        Self {
            commands: cmds,
            cycle: 0,
            current: None,
            registers: [1],
        }
    }
}

impl<Cmds> Iterator for CPU<Cmds>
where
    Cmds: Iterator<Item = Command>,
{
    type Item = CpuRegisters;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(Current { command, start }) => match command {
                Command::Noop => {
                    self.cycle += 1;
                    let before = self.registers.clone();
                    self.current = None;
                    Some(before)
                }
                Command::Addx(delta) => {
                    self.cycle += 1;
                    let before = self.registers.clone();
                    if start + 2 == self.cycle {
                        self.registers[X] += delta;
                        self.current = None;
                    }

                    Some(before)
                }
            },
            None => match self.commands.next() {
                Some(cmd) => {
                    self.current = Some(Current {
                        command: cmd,
                        start: self.cycle,
                    });
                    self.next()
                }
                None => None,
            },
        }
    }
}

fn parse_command(line: &str) -> IResult<&str, Command> {
    nom::branch::alt((
        nom::combinator::map(nom::bytes::complete::tag("noop"), |_| Command::Noop),
        nom::combinator::map(
            nom::sequence::preceded(
                nom::bytes::complete::tag("addx "),
                nom::character::complete::i32,
            ),
            |res| Command::Addx(res),
        ),
    ))(line)
}

fn parse_commands(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(|line| parse_command(line).unwrap().1)
}

pub fn solve_task1(file_content: &str) -> i32 {
    CPU::new(parse_commands(file_content))
        .enumerate()
        .filter(|(cycle, _)| (*cycle + 1) % 40 == 20)
        .map(|(cycle, [register])| ((cycle + 1) as i32) * register)
        .sum()
}

struct CRT {
    pub row: usize,
    pub col: usize,
}

impl CRT {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
    pub fn draw(&mut self, registers: &CpuRegisters) -> &str {
        let position = registers[X];
        let is_filled = (position - self.col as i32).abs() <= 1;
        let is_new_line = self.col == 39;

        self.col += 1;
        if self.col >= 40 {
            self.col = 0;
            self.row += 1;
        }

        match (is_filled, is_new_line) {
            (true, true) => "#\n",
            (true, false) => "#",
            (false, true) => ".\n",
            (false, false) => ".",
        }
    }
}

pub fn solve_task2(file_content: &str) {
    let mut cpu = CPU::new(parse_commands(file_content));
    let mut crt = CRT::new();
    for registers in cpu {
        print!("{}", crt.draw(&registers))
    }
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
