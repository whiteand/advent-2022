fn parse_grid(file_content: &str) -> Vec<Vec<u8>> {
    file_content
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let mut res = 0;
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            if is_visible(&grid, row, col) {
                res += 1;
            }
        }
    }
    res
}

fn is_visible(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let v = grid[row][col];
    let cols = grid[0].len();
    (0..row).all(|i| grid[i][col] < v)
        || ((row + 1)..grid.len()).all(|i| grid[i][col] < v)
        || (0..col).all(|j| grid[row][j] < v)
        || ((col + 1)..cols).all(|j| grid[row][j] < v)
}
struct TakeWhileInclusiveIter<T, P>
where
    T: Iterator,
    P: Fn(&T::Item) -> bool,
{
    iter: T,
    finished: bool,
    predicate: P,
}

trait TakeWhileInclusive: Iterator + Sized {
    fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusiveIter<Self, P>
    where
        P: Fn(&Self::Item) -> bool,
    {
        TakeWhileInclusiveIter {
            iter: self,
            finished: false,
            predicate,
        }
    }
}

impl<T> TakeWhileInclusive for T where T: Iterator {}

impl<T, P> Iterator for TakeWhileInclusiveIter<T, P>
where
    T: Iterator,
    P: Fn(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.iter.next() {
            Some(v) => {
                if (self.predicate)(&v) {
                    return Some(v);
                }
                self.finished = true;
                return Some(v);
            }
            None => None,
        }
    }
}

fn get_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let cols = grid[0].len();
    let top = (0..row)
        .rev()
        .map(|ind| grid[ind][col])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let right = (col + 1..cols)
        .map(|ind| grid[row][ind])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let bottom = ((row + 1)..grid.len())
        .map(|ind| grid[ind][col])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let left = (0..col)
        .rev()
        .map(|ind| grid[row][ind])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    top * right * bottom * left
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let grid = parse_grid(file_content);
    let mut res = 0;
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            let score = get_score(&grid, row, col);
            res = res.max(score);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "21");
    }

    #[test]
    fn test_score_1() {
        let grid = parse_grid(INPUT);
        let score = get_score(&grid, 1, 2);
        assert_eq!(score, 4)
    }
    #[test]
    fn test_score_2() {
        let grid = parse_grid(INPUT);
        let score = get_score(&grid, 3, 2);
        assert_eq!(score, 8)
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "8");
    }
}
