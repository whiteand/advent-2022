use crate::reduces::Reduces;

fn parse_elfes_calories<'a>(file_content: &'a str) -> impl Iterator<Item = u32> + 'a {
    file_content
        .lines()
        .map(|line| line.trim())
        .reduces(0, |elf_group, line| {
            if line.len() <= 0 {
                return false;
            }
            *elf_group = *elf_group + line.parse::<u32>().unwrap();

            return true;
        })
}

pub fn solve_part1(file_content: &str) -> u32 {
    parse_elfes_calories(file_content).max().unwrap_or_default()
}

pub fn solve_part2(file_content: &str) -> u32 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for elf in parse_elfes_calories(file_content) {
        if elf <= c {
            continue;
        }
        if elf <= b {
            c = elf;
        } else if elf <= a {
            c = b;
            b = elf;
        } else {
            c = b;
            b = a;
            a = elf;
        }
    }

    return a + b + c;
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
