use std::{env::args, fs::read_to_string};

use advent_2022::day2::solve_part1;

fn main() {
    let path_to_input = args().skip(1).next().unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let s = solve_part1(&file_content);

    println!("Score: {s:?}");
}
