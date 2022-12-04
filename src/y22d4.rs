use std::str::FromStr;

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    #[inline]
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    #[inline]
    fn overlaps(&self, other: &Range) -> bool {
        if self.start > other.end {
            return false;
        }
        if self.end < other.start {
            return false;
        }
        return true;
    }
}

struct Pair(Range, Range);

impl Pair {
    #[inline]
    fn one_contains_other(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    #[inline]
    fn has_overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        Ok(Range { start: a, end: b })
    }
}
impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        Ok(Self(a, b))
    }
}

fn parse_pairs(file_content: &str) -> impl Iterator<Item = Pair> + '_ {
    file_content.lines().map(|line| line.parse().unwrap())
}

pub fn solve_task1(file_content: &str) -> impl std::fmt::Display {
    let result = parse_pairs(file_content)
        .filter(Pair::one_contains_other)
        .count();
    result
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let result = parse_pairs(file_content).filter(Pair::has_overlaps).count();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "2");
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "4");
    }
}
