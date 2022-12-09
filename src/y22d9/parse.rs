use nom::{self, IResult};

use super::moves::Move;

fn try_parse_move(line: &str) -> IResult<&str, Move> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::bytes::complete::is_a("LRUD"),
            nom::character::complete::space1,
            nom::character::complete::u32,
        ),
        |(a, d): (&str, u32)| match a {
            "L" => Move::Left(d),
            "U" => Move::Up(d),
            "R" => Move::Right(d),
            "D" => Move::Down(d),
            _ => unreachable!(),
        },
    )(line)
}

pub fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input
        .lines()
        .map(|line| try_parse_move(line).map(|x| x.1).unwrap())
}
