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
            position: Vector::new(2, (chamber.height() + 2) as isize),
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
        match self.direction.next() {
            None => None,
            Some(dir) => match dir {
                Left => {
                    if self.position.x > 0 {
                        self.position = self.position.plus(&Vector { x: -1, y: 0 });
                    }
                    Some((self.figure, self.position))
                }
                Right => {
                    let new_possible_pos = self.position.plus(&Vector::new(1, 0));
                    let can_move =
                        self.figure.width() + (new_possible_pos.x as usize) < self.chamber.width();
                    if can_move {
                        self.position = new_possible_pos;
                    }
                    Some((self.figure, self.position))
                }
                Down => {
                    if self.position.y <= 0 {
                        self.finished = true;
                        return None;
                    }
                    let new_pos = self.position.plus(&Vector::new(0, -1));
                    let can_move = self
                        .figure
                        .points
                        .iter()
                        .map(|p| p.plus(&new_pos))
                        .all(|p| !self.chamber.is_taken(&p));

                    if can_move {
                        self.position = new_pos;
                        Some((self.figure, self.position))
                    } else {
                        self.finished = true;
                        None
                    }
                }
            },
        }
    }
}

pub fn solve_task1<const W: usize>(file_content: &str) -> usize {
    let figures = get_figures();
    let dirs = parse::parse(file_content).collect::<Vec<_>>();
    let mut figures_it = infinite(&figures);
    let mut dirs = infinite(&dirs);
    let mut all_dirs = dirs.cloned().flat_map(|dir| [dir, Down]);
    let mut chamber = Chamber::new(W);
    let mut figures_it = infinite::infinite(&figures);
    let mut i = 0;
    loop {
        let fig = figures_it.next().unwrap();
        let pos = {
            let (_, pos) = FallingFigure::new(&chamber, fig, &mut all_dirs)
                .last()
                .unwrap();
            pos
        };
        chamber.place(fig, pos);
        // chamber.print(None)
        i += 1;
        if i == 2022 {
            println!("{}", chamber.height());
            return 0;
        }
    }

    0
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d17/example.txt");
    const ACTUAL: &str = include_str!("../benches/y22d17.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1::<7>(INPUT)), "3068");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1::<7>(ACTUAL)), "0");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
