use super::moves::Move;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub head: Position,
    pub tail: Position,
}
impl State {
    pub fn step(&self, m: Move) -> State {
        match m {
            Move::Empty => self.clone(),
            Move::Up(1) => {
                if self.head.y >= self.tail.y {
                    State {
                        head: Position {
                            y: self.head.y - 1,
                            ..self.head
                        },
                        tail: self.tail,
                    }
                } else {
                    State {
                        head: Position {
                            y: self.head.y - 1,
                            ..self.head
                        },
                        tail: Position {
                            y: self.tail.y - 1,
                            x: self.head.x,
                        },
                    }
                }
            }
            Move::Right(1) => {
                if self.head.x <= self.tail.x {
                    State {
                        head: Position {
                            x: self.head.x + 1,
                            ..self.head
                        },
                        tail: self.tail,
                    }
                } else {
                    State {
                        head: Position {
                            x: self.head.x + 1,
                            ..self.head
                        },
                        tail: Position {
                            x: self.tail.x + 1,
                            y: self.head.y,
                        },
                    }
                }
            }
            Move::Down(1) => {
                if self.head.y <= self.tail.y {
                    State {
                        head: Position {
                            y: self.head.y + 1,
                            ..self.head
                        },
                        tail: self.tail,
                    }
                } else {
                    State {
                        head: Position {
                            y: self.head.y + 1,
                            ..self.head
                        },
                        tail: Position {
                            y: self.head.y + 1,
                            x: self.head.x,
                        },
                    }
                }
            }
            Move::Left(1) => {
                if self.head.x >= self.tail.x {
                    State {
                        head: Position {
                            x: self.head.x - 1,
                            ..self.head
                        },
                        tail: self.tail,
                    }
                } else {
                    State {
                        head: Position {
                            x: self.head.x - 1,
                            ..self.head
                        },
                        tail: Position {
                            x: self.tail.x - 1,
                            y: self.head.y,
                        },
                    }
                }
            }
            _ => unreachable!(),
        }
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
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.head.x.max(self.tail.x).max(0);
        let min_x = self.head.x.min(self.tail.x).min(0);
        let max_y = self.head.y.max(self.tail.y).max(0);
        let min_y = self.head.y.min(self.tail.y).min(0);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == self.head.x && y == self.head.y {
                    write!(f, "H")?;
                } else if x == self.tail.x && y == self.tail.y {
                    write!(f, "T")?;
                } else if x == 0 && y == 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
                Some(self.current)
            }
            Move::Right(d) => {
                self.current = self.current.step(Move::Right(1));
                self.current_move = if d > 1 {
                    Move::Right(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current)
            }
            Move::Down(d) => {
                self.current = self.current.step(Move::Down(1));
                self.current_move = if d > 1 {
                    Move::Down(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current)
            }
            Move::Left(d) => {
                self.current = self.current.step(Move::Left(1));
                self.current_move = if d > 1 {
                    Move::Left(d - 1)
                } else {
                    Move::Empty
                };
                Some(self.current)
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_move(state: [i32; 4], m: Move, expected: [i32; 4]) {
        let state = State {
            head: Position {
                x: state[0],
                y: state[1],
            },
            tail: Position {
                x: state[2],
                y: state[3],
            },
        };
        let new_state = state.step(m);
        assert_eq!(new_state.head.x, expected[0]);
        assert_eq!(new_state.head.y, expected[1]);
        assert_eq!(new_state.tail.x, expected[2]);
        assert_eq!(new_state.tail.y, expected[3]);
    }

    #[test]
    fn test_up_5() {
        let state = State {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail.x, 0);
        assert_eq!(new_state.tail.y, 0);
    }
    #[test]
    fn test_up_1() {
        let state = State {
            head: Position { x: -1, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, -1);
        assert_eq!(new_state.head.y, -2);
        assert_eq!(new_state.tail.x, -1);
        assert_eq!(new_state.tail.y, -1);
    }
    #[test]
    fn test_up_2() {
        let state = State {
            head: Position { x: 0, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, -2);
        assert_eq!(new_state.tail.x, 0);
        assert_eq!(new_state.tail.y, -1);
    }
    #[test]
    fn test_up_3() {
        let state = State {
            head: Position { x: 1, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, -2);
        assert_eq!(new_state.tail.x, 1);
        assert_eq!(new_state.tail.y, -1);
    }
    #[test]
    fn test_up_4() {
        let state = State {
            head: Position { x: -1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, -1);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_up_6() {
        let state = State {
            head: Position { x: 1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_up_7() {
        let state = State {
            head: Position { x: -1, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, -1);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_up_8() {
        let state = State {
            head: Position { x: 0, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_up_9() {
        let state = State {
            head: Position { x: 1, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Up(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_right_5() {
        let state = State {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail.x, 0);
        assert_eq!(new_state.tail.y, 0);
    }
    #[test]
    fn test_right_1() {
        let state = State {
            head: Position { x: -1, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail.x, 0);
        assert_eq!(new_state.tail.y, 0);
    }
    #[test]
    fn test_right_2() {
        let state = State {
            head: Position { x: 0, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail.x, 0);
        assert_eq!(new_state.tail.y, 0);
    }
    #[test]
    fn test_right_3() {
        let state = State {
            head: Position { x: 1, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 2);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail.x, 1);
        assert_eq!(new_state.tail.y, -1);
    }
    #[test]
    fn test_right_4() {
        let state = State {
            head: Position { x: -1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_right_6() {
        let state = State {
            head: Position { x: 1, y: 0 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 2);
        assert_eq!(new_state.head.y, 0);
        assert_eq!(new_state.tail.x, 1);
        assert_eq!(new_state.tail.y, 0);
    }
    #[test]
    fn test_right_7() {
        let state = State {
            head: Position { x: -1, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 0);
        assert_eq!(new_state.head.y, 1);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_right_8() {
        let state = State {
            head: Position { x: 0, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 1);
        assert_eq!(new_state.head.y, 1);
        assert_eq!(new_state.tail, state.tail);
    }
    #[test]
    fn test_right_9() {
        let state = State {
            head: Position { x: 1, y: 1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Right(1));
        assert_eq!(new_state.head.x, 2);
        assert_eq!(new_state.head.y, 1);
        assert_eq!(new_state.tail.x, 1);
        assert_eq!(new_state.tail.y, 1);
    }

    #[test]
    fn test_left_1() {
        let state = State {
            head: Position { x: -1, y: -1 },
            tail: Position { x: 0, y: 0 },
        };
        let new_state = state.step(Move::Left(1));
        assert_eq!(new_state.head.x, -2);
        assert_eq!(new_state.head.y, -1);
        assert_eq!(new_state.tail.x, -1);
        assert_eq!(new_state.tail.y, -1);
    }

    #[test]
    fn test_left_2() {
        assert_move([0, -1, 0, 0], Move::Left(1), [-1, -1, 0, 0]);
    }
    #[test]
    fn test_left_3() {
        assert_move([1, -1, 0, 0], Move::Left(1), [0, -1, 0, 0]);
    }
    #[test]
    fn test_left_4() {
        assert_move([-1, 0, 0, 0], Move::Left(1), [-2, 0, -1, 0]);
    }
    #[test]
    fn test_left_5() {
        assert_move([0, 0, 0, 0], Move::Left(1), [-1, 0, 0, 0]);
    }
    #[test]
    fn test_left_6() {
        assert_move([1, 0, 0, 0], Move::Left(1), [0, 0, 0, 0]);
    }
    #[test]
    fn test_left_7() {
        assert_move([-1, 1, 0, 0], Move::Left(1), [-2, 1, -1, 1]);
    }
    #[test]
    fn test_left_8() {
        assert_move([0, 1, 0, 0], Move::Left(1), [-1, 1, 0, 0]);
    }
    #[test]
    fn test_left_9() {
        assert_move([1, 1, 0, 0], Move::Left(1), [0, 1, 0, 0]);
    }

    #[test]
    fn test_down_1() {
        assert_move([-1, -1, 0, 0], Move::Down(1), [-1, 0, 0, 0]);
    }
    #[test]
    fn test_down_2() {
        assert_move([0, -1, 0, 0], Move::Down(1), [0, 0, 0, 0]);
    }
    #[test]
    fn test_down_3() {
        assert_move([1, -1, 0, 0], Move::Down(1), [1, 0, 0, 0]);
    }
    #[test]
    fn test_down_4() {
        assert_move([-1, 0, 0, 0], Move::Down(1), [-1, 1, 0, 0]);
    }
    #[test]
    fn test_down_5() {
        assert_move([0, 0, 0, 0], Move::Down(1), [0, 1, 0, 0]);
    }
    #[test]
    fn test_down_6() {
        assert_move([1, 0, 0, 0], Move::Down(1), [1, 1, 0, 0]);
    }
    #[test]
    fn test_down_7() {
        assert_move([-1, 1, 0, 0], Move::Down(1), [-1, 2, -1, 1]);
    }
    #[test]
    fn test_down_8() {
        assert_move([0, 1, 0, 0], Move::Down(1), [0, 2, 0, 1]);
    }
    #[test]
    fn test_down_9() {
        assert_move([1, 0, 0, 0], Move::Down(1), [1, 2, 1, 1]);
    }
}
