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

fn calculate_score(parts: &[&[char]]) -> u32 {
    let mut score = 0;
    let mut count = [0; 53];
    let mut added_to_count = [false; 53];
    for part in &parts[0..parts.len() - 1] {
        added_to_count.fill(false);
        for char in part.iter() {
            let value = get_value(*char);
            if added_to_count[value as usize] {
                continue;
            }
            count[value as usize] += 1;
            added_to_count[value as usize] = true;
        }
    }

    let mut added_to_score = [false; 53];
    for char in parts[parts.len() - 1] {
        let value = get_value(*char);
        if added_to_score[value as usize] {
            continue;
        }
        if count[value as usize] >= parts.len() - 1 {
            added_to_score[value as usize] = true;
            score += value;
        }
    }
    score
}

pub fn solve_part1(file_content: &str) -> u32 {
    let mut res = 0;
    for line in parse_lines(file_content) {
        let compartments = vec![&line[..line.len() / 2], &line[line.len() / 2..]];
        res += calculate_score(&compartments);
    }
    res
}

pub fn solve_part2(file_content: &str) -> u32 {
    let mut score = 0;

    let mut lines = parse_lines(file_content);
    loop {
        let first = lines.next();
        if first.is_none() {
            break;
        }
        let first = first.unwrap();
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        let group: Vec<&[char]> = vec![&first, &second, &third];
        score += calculate_score(&group);
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
