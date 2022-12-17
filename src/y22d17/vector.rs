#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Self {
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

impl From<(isize, isize)> for Vector {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}
