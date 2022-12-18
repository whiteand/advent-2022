mod chamber;
mod direction;
mod figure;
mod get_figures;
mod infinite;
mod parse;
mod placed_figure;
mod vector;

use get_figures::get_figures;
use infinite::infinite;
use itertools::Itertools;

use self::{
    chamber::Chamber,
    direction::Direction::{self, *},
    figure::Figure,
    vector::Vector,
};

struct FallingFigure<'i, Dirs>
where
    Dirs: Iterator<Item = Direction>,
{
    chamber: &'i Chamber<'i>,
    figure: &'i Figure,
    direction: &'i mut Dirs,
    position: Vector,
    finished: bool,
}

impl<'i, Dirs: Iterator<Item = Direction>> FallingFigure<'i, Dirs> {
    fn new(chamber: &'i Chamber<'i>, figure: &'i Figure, dirs: &'i mut Dirs) -> Self {
        Self {
            chamber,
            figure,
            direction: dirs,
            position: Vector::new(2, (chamber.height() + 3) as isize),
            finished: false,
        }
    }
}

impl<'i, Dirs: Iterator<Item = Direction>> Iterator for FallingFigure<'i, Dirs> {
    type Item = (&'i Figure, Vector);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // self.chamber.print(Some((self.figure, self.position)));
        match self.direction.next() {
            None => None,
            Some(dir) => {
                let step = match dir {
                    Left => Vector::new(-1, 0),
                    Right => Vector::new(1, 0),
                    Down => Vector::new(0, -1),
                };
                let new_pos = self.position.plus(&step);

                let can_move = new_pos.y >= 0
                    && new_pos.x >= 0
                    && new_pos.x + self.figure.width() as isize <= self.chamber.width() as isize
                    && self
                        .figure
                        .points
                        .iter()
                        .all(|p| !self.chamber.is_taken(&p.plus(&new_pos)));

                if can_move {
                    self.position = new_pos;
                } else if dir.is_down() {
                    self.finished = true;
                    return None;
                }
                Some((self.figure, self.position))
            }
        }
    }
}

pub fn solve_task1<const W: usize, const M: usize>(file_content: &str) -> usize {
    let figures = get_figures();
    let dirs = parse::parse(file_content).collect::<Vec<_>>();
    let dirs = infinite(&dirs);
    let mut all_dirs = dirs.cloned().flat_map(|dir| [dir, Down]);
    let mut chamber = Chamber::new(W);
    let mut figures_it = infinite::infinite(&figures);
    let mut heights = std::iter::repeat_with(move || {
        let fig = figures_it.next().unwrap();
        let pos = FallingFigure::new(&chamber, fig, &mut all_dirs)
            .map(|p| p.1)
            .last()
            .unwrap();
        chamber.place(fig, pos);
        chamber.height()
    });
    heights.nth(M - 1).unwrap()
}
pub fn game<const W: usize, const M: usize>(start: usize, file_content: &str) -> usize {
    let mut common = start;
    let figures = get_figures();
    println!("Figures number: {}", figures.len());
    let dirs = parse::parse(file_content).collect::<Vec<_>>();
    println!("Directions len: {}", dirs.len());
    'c: loop {
        println!("Common: {common}");
        let dirs = infinite(&dirs);
        let mut all_dirs = dirs.cloned().flat_map(|dir| [dir, Down]);
        let mut chamber = Chamber::new(W);
        let mut figures_it = infinite::infinite(&figures);
        let mut heights = std::iter::repeat_with(move || {
            let fig = figures_it.next().unwrap();
            let pos = FallingFigure::new(&chamber, fig, &mut all_dirs)
                .map(|p| p.1)
                .last()
                .unwrap();
            chamber.place(fig, pos);
            chamber.height()
        });
        let mut should_equal = None;
        let mut last_x = 0;
        for (i, x) in heights.enumerate().filter(|p| p.0 % common == 0).take(150) {
            let diff = x - last_x;
            println!("{i:4}. {x:4} {:2}", x - last_x);
            if should_equal.is_some() && should_equal.unwrap() != diff {
                common += 1;
                continue 'c;
            }
            if i > common * 3 {
                should_equal = Some(diff)
            }
            last_x = x;
        }
        return common;
    }
    0
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    solve_task1::<7, 1000000000000>(file_content)
}
#[cfg(test)]
mod tests {
    use super::*;
    // COMMON: 35, addition: 53
    const INPUT: &str = include_str!("./y22d17/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d17.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1::<7, 2022>(INPUT)), "3068");
    }
    #[test]
    fn test_game() {
        assert_eq!(format!("{}", game::<7, 2022>(10091, ACTUAL)), "3068");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1::<7, 2022>(ACTUAL)), "3109");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "1514285714288");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
