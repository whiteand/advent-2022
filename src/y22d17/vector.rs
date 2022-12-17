#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

impl Vector {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn plus(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Vector {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}
