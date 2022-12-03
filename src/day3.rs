fn parse_lines(input: &str) -> impl Iterator<Item = Vec<char>> + '_ {
    return input.lines().map(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        chars
    });
}

fn get_value(char: char) -> u32 {
    match char {
        'a'..='z' => char as u32 - 'a' as u32 + 1,
        'A'..='Z' => char as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

pub fn solve_part1(file_content: &str) -> u32 {
    let mut res = 0;
    let mut present = [false; 53];
    let mut added = [false; 53];
    for line in parse_lines(file_content) {
        present.fill(false);
        added.fill(false);
        let second_start = line.len() / 2;
        for (i, char) in line.into_iter().enumerate() {
            let value = get_value(char);
            if i < second_start {
                present[value as usize] = true;
                continue;
            }
            if !present[value as usize] {
                continue;
            }
            if added[value as usize] {
                continue;
            }
            res += value;
            added[value as usize] = true;
        }
    }
    res
}

pub fn solve_part2(file_content: &str) -> u32 {
    let mut score = 0;

    let mut added_to_score = [false; 53];
    let mut added_to_count = [false; 53];
    let mut count = [0; 53];
    for (line_index, line) in parse_lines(file_content).enumerate() {
        if line_index % 3 == 0 {
            added_to_score.fill(false);
            count.fill(0);
        }
        if line_index % 3 == 2 {
            for char in line {
                let value = get_value(char);
                let cnt = count[value as usize];
                if cnt >= 2 {
                    if added_to_score[value as usize] {
                        continue;
                    }
                    score += value;
                    added_to_score[value as usize] = true;
                }
            }
            continue;
        }
        added_to_count.fill(false);
        for char in line {
            let value = get_value(char);
            if added_to_count[value as usize] {
                continue;
            }
            added_to_count[value as usize] = true;
            count[value as usize] += 1;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(get_value('a'), 1);
        assert_eq!(get_value('A'), 27);
        assert_eq!(get_value('b'), 2);
        assert_eq!(get_value('B'), 28);
        assert_eq!(get_value('z'), 26);
        assert_eq!(get_value('Z'), 52);
        assert_eq!(get_value(' '), 0);
    }

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 157);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 70);
    }
}
