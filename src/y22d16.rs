mod parse;
mod valve;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

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

        res.make_move_mutably(valves, m);

        res
    }
    fn make_move_mutably(&mut self, valves: &BTreeMap<&str, valve::Valve>, m: Move<'i>) {
        self.collected_pressure += self.flow;
        match &m {
            Move::GoTo(new_valve) => {
                self.valve = new_valve;
            }
            Move::Open => {
                self.open_valves.insert(self.valve);
                let current_valve = valves.get(self.valve).unwrap();

                self.flow += current_valve.rate as usize;
            }
            Move::Stay => {}
        }
        self.remaining_minutes -= 1;
        self.moves.push(m);
    }

    fn burn(&mut self) {
        self.collected_pressure += self.remaining_minutes * self.flow;
        self.remaining_minutes = 0;
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
    let shortest_paths = valves_map
        .keys()
        .cartesian_product(valves_map.keys())
        .filter(|(k1, k2)| k1 != k2)
        .flat_map(|(&from, &to)| get_shortest_path(&valves_map, from, to).map(|p| ((from, to), p)))
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
    while let Some(mut task) = tasks.pop() {
        if tasks.len() % 1000 == 0 {
            println!("{}", tasks.len())
        }
        let mut has_plans = false;
        for possible_plan in get_possible_plans(&valves_map, &shortest_paths, &task) {
            has_plans = true;
            let mut new_state = task.clone();
            for m in possible_plan {
                new_state.make_move_mutably(&valves_map, m);
            }
            tasks.push(new_state);
        }
        if !has_plans {
            task.burn();
            if task.collected_pressure > max_pressure_collected {
                println!(
                    "rem:\t{}, col: {}, val: {}, flow: {}",
                    task.remaining_minutes, task.collected_pressure, task.valve, task.flow
                );
                println!("{}", task.moves.iter().map(|m| format!("{}", m)).join(" "));
                max_pressure_collected = task.collected_pressure;
            }
        } else {
            tasks.sort_by_key(|x| x.approximate_quality());
        }
    }
    max_pressure_collected
}

fn get_possible_plans<'i>(
    valves_map: &BTreeMap<&'i str, valve::Valve<'i>>,
    shortest_paths: &BTreeMap<(&'i str, &'i str), Vec<&'i str>>,
    state: &FullState<'i>,
) -> Vec<Vec<Move<'i>>> {
    if state.remaining_minutes <= 0 {
        return Vec::new();
    }

    valves_map
        .iter()
        .filter(|(&n, v)| !state.open_valves.contains(n) && v.rate > 0)
        .map(|(k, _)| k)
        .flat_map(|&goal| {
            let dir = (state.valve, goal);
            let shortest_path = shortest_paths.get(&dir);
            shortest_path
        })
        .filter(|path| path.len() < state.remaining_minutes)
        .map(|p| {
            p.into_iter()
                .map(|valve| Move::GoTo(valve))
                .chain(std::iter::once(Move::Open))
                .collect()
        })
        .collect()
}

fn get_shortest_path<'i>(
    valves_map: &BTreeMap<&str, valve::Valve<'i>>,
    from: &'i str,
    to: &'i str,
) -> Option<Vec<&'i str>> {
    let mut tasks: VecDeque<Vec<&str>> = VecDeque::new();
    tasks.push_back(Vec::new());
    while let Some(task) = tasks.pop_front() {
        let current = task.iter().last().unwrap_or(&from);
        if current.eq(&to) {
            return Some(task);
        }
        for neighbour in &valves_map.get(current).unwrap().paths {
            if current.contains(neighbour) {
                continue;
            }
            let mut new_task = task.clone();
            new_task.push(neighbour);
            tasks.push_back(new_task);
        }
    }
    None
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
