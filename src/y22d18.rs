use std::collections::BTreeMap;

use itertools::Itertools;

pub fn get_neighbours(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn solve_task1(file_content: &str) -> usize {
    let mut d: BTreeMap<(i32, i32, i32), u8> = Default::default();
    for (x, y, z) in file_content
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| -> i32 { x.parse().unwrap() })
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
    {
        let mut open_edges = 6;

        for neighbour in get_neighbours(x, y, z) {
            if !d.contains_key(&neighbour) {
                continue;
            }
            open_edges -= 1;
            *d.entry(neighbour).or_insert(6) -= 1;
        }

        d.insert((x, y, z), open_edges);
    }
    let mut sum: usize = 0;
    for v in d.values() {
        sum += *v as usize;
    }
    sum
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d18/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d18.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "64");
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
