use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use super::{parse, shortest::precalculate_shortest_paths, valve::Valve};

#[derive(Debug, Clone)]
enum Goal<'i> {
    Stay,
    Open(&'i str),
}

impl std::fmt::Display for Goal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goal::Stay => write!(f, "..."),
            Goal::Open(s) => write!(f, "open {s}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Node<'i> {
    remaining_minutes: usize,
    collected_pressure: usize,
    flow: usize,
    elephant_goal: Option<Goal<'i>>,
    my_goal: Option<Goal<'i>>,
    me: &'i str,
    open_by_me: Vec<&'i str>,
    open_by_elephant: Vec<&'i str>,
    elephant: &'i str,
    open: BTreeSet<&'i str>,
}

type SP<'i> = BTreeMap<(&'i str, &'i str), Vec<&'i str>>;
type VS<'i> = BTreeMap<&'i str, Valve<'i>>;
impl<'i> Node<'i> {
    #[inline]
    fn min_final_cost(&self) -> usize {
        self.flow * self.remaining_minutes + self.collected_pressure
    }

    #[inline]
    fn has_plan_and_time(&self) -> bool {
        self.my_goal.is_some() && self.elephant_goal.is_some() && self.have_time()
    }

    fn do_move<'a>(&'a mut self, valves: &'a VS<'i>, shortest_paths: &'a SP<'i>) {
        match (&self.my_goal, &self.elephant_goal) {
            (None, None) => unreachable!(),
            (None, Some(_)) => unreachable!(),
            (Some(_), None) => unreachable!(),
            (Some(Goal::Stay), Some(Goal::Stay)) => {
                self.burn();
                return;
            }
            _ => {}
        }
        self.collected_pressure += self.flow;
        self.move_myself(valves, shortest_paths);
        self.move_elephant(valves, shortest_paths);
        self.remaining_minutes -= 1;
    }
    fn open_valve<'a>(&mut self, valves: &'a VS<'i>, valve: &'i str) {
        self.open.insert(valve);
        self.flow += valves.get(valve).unwrap().rate as usize;
    }

    fn move_elephant<'a>(&'a mut self, valves: &'a VS<'i>, shortest_paths: &'a SP<'i>) {
        match self.elephant_goal {
            Some(Goal::Stay) => {}
            Some(Goal::Open(goal_valve)) => {
                if self.elephant == goal_valve {
                    self.open_valve(valves, goal_valve);
                    self.elephant_goal = None;
                } else {
                    match shortest_paths.get(&(self.elephant, goal_valve)) {
                        Some(p) => {
                            let first = p.first().unwrap();
                            self.elephant = first;
                        }
                        None => unreachable!(),
                    }
                }
            }
            None => unreachable!(),
        }
    }

    fn burn(&mut self) {
        self.collected_pressure += self.flow * self.remaining_minutes;
        self.remaining_minutes = 0;
    }

    fn move_myself<'a>(&'a mut self, valves: &'a VS<'i>, shortest_paths: &'a SP<'i>) {
        match self.my_goal {
            Some(Goal::Stay) => {}
            Some(Goal::Open(goal_valve)) => {
                if self.me == goal_valve {
                    self.open_valve(valves, goal_valve);
                    self.my_goal = None;
                } else {
                    match shortest_paths.get(&(self.me, goal_valve)) {
                        Some(p) => {
                            let first = p.first().unwrap();
                            self.me = first;
                        }
                        None => unreachable!(),
                    }
                }
            }
            None => unreachable!(),
        }
    }
    fn plan(&mut self, valves: &VS<'i>, shortest_paths: &SP<'i>) -> Vec<Self> {
        let mut res = Vec::new();
        if self.my_goal.is_none() {
            for valve in self
                .interesting_valves(valves, shortest_paths, self.me)
                .filter(|goal| match self.elephant_goal {
                    Some(Goal::Open(s)) => goal != &s,
                    _ => true,
                })
            {
                let mut new_node = self.clone();
                new_node.my_goal = Some(Goal::Open(valve));
                res.push(new_node);
            }
            if res.is_empty() {
                let mut new_node = self.clone();
                new_node.my_goal = Some(Goal::Stay);
                res.push(new_node);
            }
        } else if self.elephant_goal.is_none() {
            for valve in self
                .interesting_valves(valves, shortest_paths, self.elephant)
                .filter(|goal| match self.my_goal {
                    Some(Goal::Open(s)) => goal != &s,
                    _ => true,
                })
            {
                let mut new_node = self.clone();
                new_node.elephant_goal = Some(Goal::Open(valve));
                res.push(new_node);
            }
            if res.is_empty() {
                let mut new_node = self.clone();
                new_node.elephant_goal = Some(Goal::Stay);
                res.push(new_node);
            }
        }
        res
    }

    fn is_open(&self, key: &'i str) -> bool {
        self.open.contains(&key)
    }

    fn interesting_valves<'a>(
        &'a self,
        valves: &'a BTreeMap<&'i str, Valve<'i>>,
        shortest_paths: &'a BTreeMap<(&'i str, &'i str), Vec<&'i str>>,
        place: &'i str,
    ) -> impl Iterator<Item = &'i str> + '_ {
        valves
            .iter()
            .filter(|(_, v)| v.rate > 0)
            .map(|(k, _)| *k)
            .filter(|k| !self.is_open(k))
            .filter(|goal| match shortest_paths.get(&(place, goal)) {
                Some(p) => p.len() < self.remaining_minutes,
                None => false,
            })
    }

    #[inline]
    fn have_time(&self) -> bool {
        self.remaining_minutes > 0
    }
}

impl<'i> PartialEq for Node<'i> {
    fn eq(&self, other: &Self) -> bool {
        self.min_final_cost() == other.min_final_cost()
    }
}
impl<'i> Eq for Node<'i> {}

impl<'i> PartialOrd for Node<'i> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'i> Ord for Node<'i> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.has_plan_and_time(), other.has_plan_and_time()) {
            (true, true) => self.min_final_cost().cmp(&other.min_final_cost()),
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => self.min_final_cost().cmp(&other.min_final_cost()),
        }
    }
}

impl<'i> Node<'i> {
    fn new(remaining_minutes: usize, me: &'i str, elephant: &'i str) -> Self {
        Self {
            remaining_minutes,
            collected_pressure: 0,
            flow: 0,
            elephant_goal: None,
            my_goal: None,
            open_by_elephant: Vec::new(),
            open_by_me: Vec::new(),
            me,
            elephant,
            open: std::default::Default::default(),
        }
    }
}

pub(crate) fn solve_task2(file_content: &str, minutes: usize) -> usize {
    let valves = parse::parse(file_content)
        .into_iter()
        .map(|valve| (valve.name, valve))
        .collect::<BTreeMap<_, _>>();

    let shortest_paths = precalculate_shortest_paths(&valves);

    let mut max_pressure_collected = 0;

    let mut nodes = BinaryHeap::new();
    nodes.push(Node::new(minutes, "AA", "AA"));
    while let Some(mut node) = nodes.pop() {
        while node.has_plan_and_time() {
            node.do_move(&valves, &shortest_paths);
        }
        if node.have_time() {
            for next_node in node.plan(&valves, &shortest_paths) {
                nodes.push(next_node)
            }
        } else {
            if node.collected_pressure > max_pressure_collected {
                println!("{:#?}", node);
                max_pressure_collected = node.collected_pressure;
            }
        }
    }
    max_pressure_collected
}
