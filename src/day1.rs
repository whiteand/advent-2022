use crate::split_by::SplitByTrait;

fn parse_elfes_calories<'a>(file_content: &'a str) -> impl Iterator<Item = u32> + 'a {
    file_content
        .lines()
        .map(|line| line.trim())
        .split_by(|line| line.is_empty())
        .map(|lines| {
            lines
                .into_iter()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
}

pub fn solve_part1(file_content: &str) -> u32 {
    parse_elfes_calories(file_content).max().unwrap_or_default()
}

pub fn solve_part2(file_content: &str) -> u32 {
    let mut elfes: Vec<_> = parse_elfes_calories(file_content).collect();

    elfes.sort_by(|a, b| b.cmp(a));

    elfes.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 24000);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 45000);
    }
}
