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
        tasks.sort_by(|(r1, c1), (r2, c2)| {
            let min_d1 = minimal_distance[*r1][*c1];
            let min_d2 = minimal_distance[*r2][*c2];
            min_d2.cmp(&min_d1)
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
        {
            let neighbour_height = grid[r][c];
            if !(0..=current_height + 1).contains(&neighbour_height) {
                continue;
            }
            let min_distance =
                minimal_distance[r][c].min(minimal_distance[row][col].saturating_add(1));
            minimal_distance[r][c] = min_distance;
            tasks.push((r, c));
        }
        visited[row][col] = true;
    }
    // print_dists(&minimal_distance);
    minimal_distance[end.0][end.1]
}
pub fn solve_task1(file_content: &str) -> usize {
    let (grid, start, end) = parse_grid(file_content);
    solve(&grid, start, end)
}

fn print_dists(minimal_distance: &[Vec<usize>]) {
    for l in minimal_distance.iter() {
        for x in l.iter() {
            let c = if *x == usize::MAX {
                "  ∞".to_owned()
            } else {
                format!("{:3}", x)
            };
            print!("{c}");
        }
        println!();
    }
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let (grid, _, end) = parse_grid(file_content);
    let mut min_step = usize::MAX;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let v = grid[i][j];
            if v == START_VALUE {
                let min_steps = solve(&grid, (i, j), end);
                min_step = min_step.min(min_steps);
            }
        }
    }
    min_step
}

fn dist(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
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

fn find_in_grid<T: Eq + PartialEq + Copy>(grid: &[Vec<T>], value: T) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(row, line)| {
        line.iter().enumerate().find_map(
            move |(col, x)| {
                if *x == value {
                    Some((row, col))
                } else {
                    None
                }
            },
        )
    })
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
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "31");
    }
    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "484");
    }
    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "29");
    }
    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
