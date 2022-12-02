use advent_2022::day1::solve_part1;
use std::{env::args, fs::read_to_string};

fn main() {
    let path_to_input = args().skip(1).next().unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let max_sum = solve_part1(&file_content);

    println!("Max calories: {max_sum}")
}
