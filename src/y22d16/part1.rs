use std::collections::{BTreeMap, BTreeSet};

use super::{parse, shortest::precalculate_shortest_paths, step::Step, valve};

#[derive(Debug, Clone)]
struct FullState<'i> {
    pub flow: usize,
    pub valve: &'i str,
    pub open_valves: BTreeSet<&'i str>,
    pub collected_pressure: usize,
    pub remaining_minutes: usize,
    pub moves: Vec<Step<'i>>,
}

impl<'i> FullState<'i> {
    fn make_move_mutably(&mut self, valves: &BTreeMap<&str, valve::Valve>, m: Step<'i>) {
        self.collected_pressure += self.flow;
        match &m {
            Step::GoTo(new_valve) => {
                self.valve = new_valve;
            }
            Step::Open => {
                self.open_valves.insert(self.valve);
                let current_valve = valves.get(self.valve).unwrap();

                self.flow += current_valve.rate as usize;
            }
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
    let valves = parse::parse(file_content);
    let valves_map = valves
        .into_iter()
        .map(|valve| (valve.name, valve))
        .collect::<BTreeMap<_, _>>();
    let shortest_paths = precalculate_shortest_paths(&valves_map);

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
) -> Vec<Vec<Step<'i>>> {
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
                .map(|valve| Step::GoTo(valve))
                .chain(std::iter::once(Step::Open))
                .collect()
        })
        .collect()
}