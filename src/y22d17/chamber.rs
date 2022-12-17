use super::{figure::Figure, placed_figure::PlacedFigure, vector::Vector};

pub struct Chamber<'i> {
    pub placed: Vec<PlacedFigure<'i>>,
    width: usize,
    height: usize,
}

impl<'i> Chamber<'i> {
    pub fn new(width: usize) -> Self {
        Self {
            placed: Vec::new(),
            width,
            height: 0,
        }
    }
    pub fn place(&mut self, figure: &'i Figure, left_bottom: Vector) {
        if figure.points.is_empty() {
            return;
        }
        let max_y = figure
            .points
            .iter()
            .map(|v| v.y + left_bottom.y)
            .max()
            .unwrap();
        if max_y >= self.height {
            self.height = max_y + 1;
        }
        self.placed.push(PlacedFigure {
            figure,
            left_bottom,
        })
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn taken(&self) -> impl Iterator<Item = Vector> + '_ {
        self.placed
            .iter()
            .flat_map(|pf| pf.figure.points.iter().map(|v| v.plus(&pf.left_bottom)))
    }
    pub fn print(&self) {
        let mut screen = vec![vec!['.'; self.width]; self.height];

        for Vector { x, y } in self.taken() {
            screen[y][x] = '#';
        }

        screen.reverse();

        let mut s: String = String::with_capacity(
            self.width * self.height + self.height * 2 + self.width + 2 + self.height + 1,
        );
        for line in screen {
            s.push('|');
            for ch in line {
                s.push(ch);
            }
            s.push('|');
            s.push('\n');
        }
        s.push('+');
        for _ in 0..self.width {
            s.push('-');
        }
        s.push('+');
        println!("{s}");
    }
}
