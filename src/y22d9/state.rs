use super::moves::Move;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn go(&self, m: Move) -> Position {
        match m {
            Move::Empty => Position {
                x: self.x,
                y: self.y,
            },
            Move::Left(d) => Position {
                x: self.x - d as i32,
                y: self.y,
            },
            Move::Up(d) => Position {
                x: self.x,
                y: self.y - d as i32,
            },
            Move::Right(d) => Position {
                x: self.x + d as i32,
                y: self.y,
            },
            Move::Down(d) => Position {
                x: self.x,
                y: self.y + d as i32,
            },
        }
    }
}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialOrd<Position> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            x => x,
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn touching(head: &Position, tail: &Position) -> bool {
    (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1
}

fn follow(head: &Position, tail: &Position) -> Position {
    if touching(head, tail) {
        return Position {
            x: tail.x,
            y: tail.y,
        };
    }
    let dx = (head.x - tail.x).signum();
    let dy = (head.y - tail.y).signum();
    Position {
        x: tail.x + dx,
        y: tail.y + dy,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State(Vec<Position>);

impl FromIterator<Position> for State {
    fn from_iter<T: IntoIterator<Item = Position>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl State {
    pub fn new(size: usize) -> Self {
        std::iter::repeat_with(|| Position::default())
            .take(size)
            .collect::<State>()
    }
    pub fn step(&self, m: Move) -> State {
        let mut res = Vec::with_capacity(self.0.len());
        res.push(self.0[0].go(m));
        for i in 1..self.0.len() {
            res.push(follow(&res[i - 1], &self.0[i]));
        }
        Self(res)
    }
    pub fn get_path<T>(&self, moves: T) -> StateStream<T>
    where
        T: Iterator<Item = Move>,
    {
        return StateStream {
            current: self.clone(),
            moves,
            current_move: Move::Empty,
        };
    }
    pub fn tail(&self) -> Position {
        self.0.last().map(|x| x.clone()).unwrap_or_default()
    }
}

pub struct StateStream<T>
where
    T: Iterator<Item = Move>,
{
    current: State,
    moves: T,
    current_move: Move,
}

impl<T> Iterator for StateStream<T>
where
    T: Iterator<Item = Move>,
{
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_move {
            Move::Empty => match self.moves.next() {
                Some(m) => {
                    self.current_move = m;
                    self.next()
                }
                None => None,
            },
            Move::Up(d) => {
                self.current = self.current.step(Move::Up(1));
                self.current_move = if d > 1 { Move::Up(d - 1) } else { Move::Empty };
                Some(self.current.clone())
            }
            Move::Right(d) => {
                self.current = self.current.step(Move::Right(1));
                self.current_move = if d > 1 {
                    Move::Right(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current.clone())
            }
            Move::Down(d) => {
                self.current = self.current.step(Move::Down(1));
                self.current_move = if d > 1 {
                    Move::Down(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current.clone())
            }
            Move::Left(d) => {
                self.current = self.current.step(Move::Left(1));
                self.current_move = if d > 1 {
                    Move::Left(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_move(state: [i32; 4], m: Move, expected: [i32; 4]) {
        let state = vec![
            Position {
                x: state[0],
                y: state[1],
            },
            Position {
                x: state[2],
                y: state[3],
            },
        ]
        .into_iter()
        .collect::<State>();
        let new_state = state.step(m);
        assert_eq!(
            new_state.0,
            vec![
                Position {
                    x: expected[0],
                    y: expected[1],
                },
                Position {
                    x: expected[2],
                    y: expected[3],
                },
            ]
        )
    }

    #[test]
    #[ignore]
    fn test_up_5() {
        assert_move([0, 0, 0, 0], Move::Up(1), [0, -1, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_up_1() {
        assert_move([-1, -1, 0, 0], Move::Up(1), [-1, -2, -1, -1]);
    }
    #[test]
    #[ignore]
    fn test_up_2() {
        assert_move([-1, -1, 0, 0], Move::Up(1), [-1, -2, -1, -1]);
    }
    #[test]
    #[ignore]
    fn test_up_3() {
        assert_move([1, -1, 0, 0], Move::Up(1), [1, -2, 1, -1]);
    }
    #[test]
    #[ignore]
    fn test_up_4() {
        assert_move([-1, 0, 0, 0], Move::Up(1), [-1, -1, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_up_6() {
        assert_move([1, 0, 0, 0], Move::Up(1), [1, -1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_up_7() {
        assert_move([-1, 1, 0, 0], Move::Up(1), [-1, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_up_8() {
        assert_move([0, 1, 0, 0], Move::Up(1), [0, 0, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_up_9() {
        assert_move([1, 1, 0, 0], Move::Up(1), [1, 0, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_right_5() {
        assert_move([0, 0, 0, 0], Move::Right(1), [1, 0, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_right_1() {
        assert_move([-1, -1, 0, 0], Move::Right(1), [0, -1, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_right_2() {
        assert_move([0, -1, 0, 0], Move::Right(1), [1, -1, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_right_3() {
        assert_move([1, -1, 0, 0], Move::Right(1), [2, -1, 1, -1])
    }
    #[test]
    #[ignore]
    fn test_right_4() {
        assert_move([-1, 0, 0, 0], Move::Right(1), [0, 0, 0, 0])
    }
    #[test]
    #[ignore]
    fn test_right_6() {
        assert_move([1, 0, 0, 0], Move::Right(1), [2, 0, 1, 0]);
    }
    #[test]
    #[ignore]
    fn test_right_7() {
        assert_move([-1, 1, 0, 0], Move::Right(1), [0, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_right_8() {
        assert_move([0, 1, 0, 0], Move::Right(1), [1, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_right_9() {
        assert_move([1, 1, 0, 0], Move::Right(1), [2, 1, 1, 1]);
    }

    #[test]
    #[ignore]
    fn test_left_1() {
        assert_move([-1, -1, 0, 0], Move::Left(1), [-2, -1, -1, -1]);
    }

    #[test]
    #[ignore]
    fn test_left_2() {
        assert_move([0, -1, 0, 0], Move::Left(1), [-1, -1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_3() {
        assert_move([1, -1, 0, 0], Move::Left(1), [0, -1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_4() {
        assert_move([-1, 0, 0, 0], Move::Left(1), [-2, 0, -1, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_5() {
        assert_move([0, 0, 0, 0], Move::Left(1), [-1, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_6() {
        assert_move([1, 0, 0, 0], Move::Left(1), [0, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_7() {
        assert_move([-1, 1, 0, 0], Move::Left(1), [-2, 1, -1, 1]);
    }
    #[test]
    #[ignore]
    fn test_left_8() {
        assert_move([0, 1, 0, 0], Move::Left(1), [-1, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_left_9() {
        assert_move([1, 1, 0, 0], Move::Left(1), [0, 1, 0, 0]);
    }

    #[test]
    #[ignore]
    fn test_down_1() {
        assert_move([-1, -1, 0, 0], Move::Down(1), [-1, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_2() {
        assert_move([0, -1, 0, 0], Move::Down(1), [0, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_3() {
        assert_move([1, -1, 0, 0], Move::Down(1), [1, 0, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_4() {
        assert_move([-1, 0, 0, 0], Move::Down(1), [-1, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_5() {
        assert_move([0, 0, 0, 0], Move::Down(1), [0, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_6() {
        assert_move([1, 0, 0, 0], Move::Down(1), [1, 1, 0, 0]);
    }
    #[test]
    #[ignore]
    fn test_down_7() {
        assert_move([-1, 1, 0, 0], Move::Down(1), [-1, 2, -1, 1]);
    }
    #[test]
    #[ignore]
    fn test_down_8() {
        assert_move([0, 1, 0, 0], Move::Down(1), [0, 2, 0, 1]);
    }
    #[test]
    #[ignore]
    fn test_down_9() {
        assert_move([1, 1, 0, 0], Move::Down(1), [1, 2, 1, 1]);
    }
}
