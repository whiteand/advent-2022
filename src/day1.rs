pub fn solve_part1(file_content: &str) -> i32 {
    file_content
        .split("\n\n")
        .map(|single_str| {
            single_str
                .lines()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn solve_part2(file_content: &str) -> i32 {
    let mut elfes: Vec<i32> = file_content
        .split("\n\n")
        .map(|single_str| {
            single_str
                .lines()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    elfes.sort_by(|a, b| b.cmp(a));

    elfes.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 24000);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 45000);
    }
}
