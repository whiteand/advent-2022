pub enum Move {
    Empty,
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}

impl Default for Move {
    fn default() -> Self {
        Move::Empty
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "_"),
            Self::Up(d) => write!(f, "U {}", d),
            Self::Right(d) => write!(f, "R {}", d),
            Self::Down(d) => write!(f, "D {}", d),
            Self::Left(d) => write!(f, "L {}", d),
        }
    }
}
