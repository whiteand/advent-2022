use advent::y15d04::solve_task2;
use std::{env::args, fs::read_to_string};

fn main() {
    let path_to_input = args().skip(1).next().unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let answer = solve_task2(&file_content);

    println!("Answer: {answer}")
}
