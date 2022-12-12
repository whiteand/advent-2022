use std::collections::VecDeque;

pub fn solve_task1(file_content: &str) -> usize {
    let (grid, start, end) = parse_grid(file_content);
    solve(&grid, start, end)
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let (grid, _, end) = parse_grid(file_content);
    (0..grid.len())
        .flat_map(|row| (0..grid[row].len()).map(move |col| (row, col)))
        .filter(|(r, c)| grid[*r][*c] == START_VALUE)
        .map(|p| solve(&grid, p, end))
        .min()
        .unwrap_or_default()
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const END_VALUE: usize = ALPHABET.len() - 1;
const START_VALUE: usize = 0;

pub fn solve(grid: &[Vec<usize>], start: (usize, usize), end: (usize, usize)) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = grid
        .iter()
        .map(|v| vec![false; v.len()])
        .collect::<Vec<_>>();
    let mut minimal_distance = grid
        .iter()
        .map(|v| vec![usize::MAX; v.len()])
        .collect::<Vec<_>>();
    let mut tasks: Vec<(usize, usize)> = Vec::new();

    // invariants:
    //   visited[i] is true if the node was already visited and minimal distance was calculated to all neighbours
    //   minimal_distance[i] - contains minimal distance to the node if the node was already visited
    //   tasks - contains a list of visited nodes which neighbors were potentially not visited.

    minimal_distance[start.0][start.1] = 0;

    tasks.push(start);

    while tasks.len() > 0 {
        tasks.sort_by(|a, b| {
            let g1 = minimal_distance[a.0][a.1];
            let g2 = minimal_distance[b.0][b.1];
            g2.cmp(&g1)
        });
        let Some((row, col)) = tasks.pop() else {
            unreachable!();
        };
        if visited[row][col] {
            continue;
        }
        let current_height = grid[row][col];
        for (r, c) in get_neighbours(rows, cols, row, col)
            .into_iter()
            .filter(|(r, c)| !visited[*r][*c])
            .filter(|(r, c)| (0..=current_height + 1).contains(&grid[*r][*c]))
        {
            let min_distance =
                minimal_distance[r][c].min(minimal_distance[row][col].saturating_add(1));
            minimal_distance[r][c] = min_distance;
            if r == end.0 && c == end.1 {
                return min_distance;
            }
            tasks.push((r, c));
        }
        visited[row][col] = true;
    }
    usize::MAX
}

fn parse_grid(file_content: &str) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let mut res = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    for (row, line) in file_content.lines().enumerate() {
        let mut new_line = Vec::with_capacity(line.len());
        for (col, ch) in line.chars().enumerate() {
            let v = if ch == 'S' {
                start = Some((row, col));
                START_VALUE
            } else if ch == 'E' {
                end = Some((row, col));
                END_VALUE
            } else {
                ALPHABET
                    .chars()
                    .enumerate()
                    .find_map(|(i, c)| if c == ch { Some(i) } else { None })
                    .unwrap_or_default()
            };
            new_line.push(v);
        }
        res.push(new_line);
    }
    (res, start.unwrap(), end.unwrap())
}

pub fn get_neighbours(rows: usize, cols: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
    let min_row = row.saturating_sub(1);
    let max_row = (row + 1).min(rows.saturating_sub(1));
    let min_col = col.saturating_sub(1);
    let max_col = (col + 1).min(cols.saturating_sub(1));
    let mut res = Vec::new();
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if r == row && c == col {
                continue;
            }
            if r != row && c != col {
                continue;
            }
            res.push((r, c))
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const ACTUAL: &str = include_str!("../benches/y22d12.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "31");
    }
    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "484");
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "29");
    }
    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "478");
    }
}
