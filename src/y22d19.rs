use nom::{
    bytes::complete::tag,
    character::{self},
    sequence::preceded,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

fn parse_blueprint(line: &str) -> IResult<&str, Blueprint> {
    let (input, blueprint_id) = preceded(tag("Blueprint "), character::complete::u32)(line)?;
    let (input, ore_robot_ore_cost) =
        preceded(tag(": Each ore robot costs "), character::complete::u32)(input)?;
    let (input, clay_robot_ore_cost) = preceded(
        tag(" ore. Each clay robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, obsidian_robot_ore_cost) = preceded(
        tag(" ore. Each obsidian robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, obsidian_robot_clay_cost) =
        preceded(tag(" ore and "), character::complete::u32)(input)?;
    let (input, geode_robot_ore_cost) = preceded(
        tag(" clay. Each geode robot costs "),
        character::complete::u32,
    )(input)?;
    let (input, geode_robot_obsidian_cost) =
        preceded(tag(" ore and "), character::complete::u32)(input)?;
    let blueprint = Blueprint {
        id: blueprint_id as usize,
        ore_robot_cost: Resources {
            ore: ore_robot_ore_cost as usize,
            clay: 0,
            obsidian: 0,
        },
        clay_robot_cost: Resources {
            ore: clay_robot_ore_cost as usize,
            clay: 0,
            obsidian: 0,
        },
        obsidian_robot_cost: Resources {
            clay: obsidian_robot_clay_cost as usize,
            ore: obsidian_robot_ore_cost as usize,
            obsidian: 0,
        },
        geode_robot_cost: Resources {
            ore: geode_robot_ore_cost as usize,
            clay: 0,
            obsidian: geode_robot_obsidian_cost as usize,
        },
    };
    Ok((input, blueprint))
}

fn get_quality_level(blueprint: &Blueprint) -> usize {
    0
}

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(|line| parse_blueprint(line).unwrap().1)
        .map(|b| get_quality_level(&b))
        .sum()
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d19/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d19.txt");

    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "33");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "0");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
