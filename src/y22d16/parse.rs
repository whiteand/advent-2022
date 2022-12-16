use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::alpha1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use super::valve::Valve;

pub fn parse(file_content: &str) -> Vec<Valve> {
    file_content
        .lines()
        .flat_map(|line| parse_valve(line))
        .map(|(_, b)| b)
        .collect()
}

fn parse_valve(line: &str) -> IResult<&str, Valve> {
    map(
        tuple((parse_name, parse_rate, parse_paths)),
        |(name, rate, paths)| Valve {
            rate: rate,
            paths: paths,
            name: name,
        },
    )(line)
}

fn parse_name(line: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), alpha1)(line)
}
fn parse_rate(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), character::complete::u32)(input)
}
fn parse_paths(input: &str) -> IResult<&str, Vec<&str>> {
    let parse_list = separated_list1(tag(", "), alpha1);
    preceded(
        alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        )),
        parse_list,
    )(input)
}
