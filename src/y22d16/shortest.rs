use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use super::valve;

pub fn precalculate_shortest_paths<'a>(
    valves_map: &BTreeMap<&'a str, valve::Valve<'a>>,
) -> BTreeMap<(&'a str, &'a str), Vec<&'a str>> {
    let reachable = get_reachable_valves(valves_map, "AA");

    let mut memory: BTreeMap<(&str, &str), Option<Vec<&str>>> = BTreeMap::new();

    for from in valves_map.keys().filter(|k| reachable.contains(*k)) {
        let mut tasks: BinaryHeap<ShortestPath> = BinaryHeap::new();
        tasks.push(Vec::new().into());
        while let Some(ShortestPath(path)) = tasks.pop() {
            let current = path.iter().last().unwrap_or(&from);
            if !memory.contains_key(&(from, current)) && from != current {
                memory.insert((from, current), Some(path.clone()));
            }
            for neighbour in &valves_map.get(current).unwrap().paths {
                if current.contains(neighbour) {
                    continue;
                }
                if memory.contains_key(&(from, neighbour)) {
                    continue;
                }
                let mut new_task = path.clone();
                new_task.push(neighbour);
                tasks.push(new_task.into());
            }
        }
    }

    memory
        .into_iter()
        .flat_map(|(k, mp)| mp.map(|p| (k, p)))
        .collect()
}

#[derive(Debug)]
struct ShortestPath<'i>(Vec<&'i str>);

impl<'i> PartialEq for ShortestPath<'i> {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}
impl<'i> Eq for ShortestPath<'i> {}

impl<'i> PartialOrd for ShortestPath<'i> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl<'i> Ord for ShortestPath<'i> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let n = self.0.len();
        let m = other.0.len();
        if n > m {
            std::cmp::Ordering::Less
        } else if n < m {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
impl<'i> From<Vec<&'i str>> for ShortestPath<'i> {
    fn from(v: Vec<&'i str>) -> Self {
        ShortestPath(v)
    }
}

fn get_reachable_valves<'i>(
    valves_map: &BTreeMap<&'i str, valve::Valve<'i>>,
    from: &'i str,
) -> BTreeSet<&'i str> {
    let mut visited = BTreeSet::new();
    let mut tasks = vec![from];
    while let Some(valve) = tasks.pop() {
        visited.insert(valve);
        for neighbour in &valves_map.get(valve).unwrap().paths {
            if !visited.contains(neighbour) {
                tasks.push(neighbour)
            }
        }
    }
    visited
}
