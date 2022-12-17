use itertools::Itertools;

pub struct Figure {
    pub points: Vec<(usize, usize)>,
}

impl Figure {
    pub fn new() -> Self {
        Figure::default()
    }
    /// Returns the height of this [`Figure`].
    ///
    /// # Panics
    ///
    /// Panics if figure is empty
    pub fn height(&self) -> usize {
        let last = self.points.last().unwrap();
        last.0 + 1
    }

    /// Returns the width of this [`Figure`].
    ///
    /// # Panics
    ///
    /// Panics if figure is empty
    pub fn width(&self) -> usize {
        self.points.last().iter().map(|(y, x)| *x).max().unwrap() + 1
    }

    pub fn contains(&self, target_x: usize, target_y: usize) -> bool {
        self.points
            .iter()
            .take_while(|(y, _)| *y <= target_y)
            .contains(&(target_y, target_x))
    }
}

impl std::fmt::Debug for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.contains(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<(usize, usize)>> for Figure {
    fn from(points: Vec<(usize, usize)>) -> Self {
        Figure { points }
    }
}
impl Default for Figure {
    fn default() -> Self {
        Self {
            points: Default::default(),
        }
    }
}
