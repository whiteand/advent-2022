use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<char> for Choice {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Choice::Rock),
            'B' => Ok(Choice::Paper),
            'C' => Ok(Choice::Scissors),
            'X' => Ok(Choice::Rock),
            'Y' => Ok(Choice::Paper),
            'Z' => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Choice::*;
        match (self, other) {
            (Rock, Paper) => Some(Ordering::Less),
            (Rock, Scissors) => Some(Ordering::Greater),
            (Paper, Rock) => Some(Ordering::Greater),
            (Paper, Scissors) => Some(Ordering::Less),
            (Scissors, Rock) => Some(Ordering::Less),
            (Scissors, Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

impl From<(Choice, Choice)> for Outcome {
    fn from((a, b): (Choice, Choice)) -> Self {
        match a.partial_cmp(&b) {
            Some(Ordering::Greater) => Outcome::Loss,
            Some(Ordering::Less) => Outcome::Win,
            Some(Ordering::Equal) => Outcome::Draw,
            None => unreachable!(),
        }
    }
}

fn get_score((a, b): (Choice, Choice)) -> u32 {
    Outcome::from((a, b)) as u32 + b as u32
}

pub fn solve_part1(file_content: &str) -> u32 {
    let games = file_content.lines().map(|line| -> (Choice, Choice) {
        let mut chars = line.chars();
        let first_char = chars.next().unwrap();
        chars.next();
        let second_char = chars.next().unwrap();

        (
            first_char.try_into().unwrap(),
            second_char.try_into().unwrap(),
        )
    });
    let scores = games.map(get_score);

    scores.sum::<u32>()
}

pub fn solve_part2(file_content: &str) -> u32 {
    let games = file_content.lines().map(|line| -> (Choice, Outcome) {
        let mut chars = line.chars();
        let first_char = chars.next().unwrap();
        chars.next();
        let second_char = chars.next().unwrap();

        (
            first_char.try_into().unwrap(),
            second_char.try_into().unwrap(),
        )
    });
    let scores = games
        .map(|(opponent, outcome)| -> (Choice, Choice) {
            (
                opponent,
                match (opponent, outcome) {
                    (Choice::Rock, Outcome::Win) => Choice::Paper,
                    (Choice::Rock, Outcome::Loss) => Choice::Scissors,
                    (Choice::Paper, Outcome::Win) => Choice::Scissors,
                    (Choice::Paper, Outcome::Loss) => Choice::Rock,
                    (Choice::Scissors, Outcome::Win) => Choice::Rock,
                    (Choice::Scissors, Outcome::Loss) => Choice::Paper,
                    _ => opponent,
                },
            )
        })
        .map(get_score);

    scores.sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 15);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 12);
    }
}
