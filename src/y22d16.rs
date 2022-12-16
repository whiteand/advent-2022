mod parse;
mod valve;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

use parse::parse;

#[derive(Debug, Clone)]
enum Move<'i> {
    GoTo(&'i str),
    Open,
    Stay,
}

impl std::fmt::Display for Move<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::GoTo(s) => write!(f, "->{}", s),
            Move::Open => write!(f, "OPEN"),
            Move::Stay => write!(f, "..."),
        }
    }
}

#[derive(Debug, Clone)]
struct FullState<'i> {
    pub flow: usize,
    pub valve: &'i str,
    pub open_valves: BTreeSet<&'i str>,
    pub collected_pressure: usize,
    pub remaining_minutes: usize,
    pub moves: Vec<Move<'i>>,
}

impl<'i> FullState<'i> {
    fn make_move(&self, valves: &BTreeMap<&str, valve::Valve>, m: Move<'i>) -> FullState<'i> {
        let mut res = self.clone();

        res.collected_pressure += self.flow;
        match &m {
            Move::GoTo(new_valve) => {
                res.valve = new_valve;
            }
            Move::Open => {
                res.open_valves.insert(res.valve);
                let current_valve = valves.get(res.valve).unwrap();

                res.flow += current_valve.rate as usize;
            }
            Move::Stay => {}
        }
        res.remaining_minutes -= 1;
        res.moves.push(m);
        res
    }

    fn approximate_quality(&self) -> usize {
        self.collected_pressure + self.flow * self.remaining_minutes
    }
}

pub fn solve_task1(file_content: &str, minutes: usize) -> usize {
    let valves = parse(file_content);
    let valves_map = valves
        .into_iter()
        .map(|valve| (valve.name, valve))
        .collect::<BTreeMap<_, _>>();
    let mut tasks = vec![FullState {
        flow: 0,
        valve: "AA",
        remaining_minutes: minutes,
        open_valves: Default::default(),
        collected_pressure: 0,
        moves: Vec::new(),
    }];

    let mut max_pressure_collected = 0;
    while let Some(task) = tasks.pop() {
        let mut has_moves = false;
        for possible_move in get_possible_moves(&valves_map, &task, minutes as usize) {
            has_moves = true;
            let new_state = task.make_move(&valves_map, possible_move);
            tasks.push(new_state);
        }
        if has_moves {
            tasks.sort_by_key(|x| x.approximate_quality());
        }
        if task.remaining_minutes <= 0 && task.collected_pressure > max_pressure_collected {
            println!(
                "rem:\t{}, col: {}, val: {}, flow: {}",
                task.remaining_minutes, task.collected_pressure, task.valve, task.flow
            );
            println!("{}", task.moves.iter().map(|m| format!("{}", m)).join(" "));
            max_pressure_collected = task.collected_pressure;
        }
    }
    max_pressure_collected
}

fn get_possible_moves<'i>(
    valves_map: &BTreeMap<&str, valve::Valve<'i>>,
    state: &FullState<'i>,
    minutes: usize,
) -> Vec<Move<'i>> {
    if state.remaining_minutes <= 0 {
        return Vec::new();
    }

    let mut res = Vec::new();

    for neighbour in valves_map.get(&state.valve).unwrap().paths.iter() {
        let been_there = state
            .moves
            .iter()
            .rev()
            .take_while(|m| match m {
                Move::GoTo(_) => true,
                Move::Open => false,
                Move::Stay => true,
            })
            .find(|m| match m {
                Move::GoTo(s) => s == neighbour,
                Move::Open => false,
                Move::Stay => false,
            })
            .is_some();
        if been_there {
            continue;
        }
        res.push(Move::GoTo(neighbour));
    }
    if res.len() <= 0 {
        res.push(Move::Stay);
    }
    if !state.open_valves.contains(&state.valve) && valves_map.get(&state.valve).unwrap().rate != 0
    {
        res.push(Move::Open);
    }

    return res;
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d16/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d16.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT, 30)), "1651");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL, 30)), "0");
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
