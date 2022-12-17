mod chamber;
mod direction;
mod figure;
mod get_figures;
mod parse;
mod placed_figure;
mod vector;

use get_figures::get_figures;
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
    direction: Dirs,
    position: Vector,
}

impl<'i, Dirs: Iterator<Item = Direction>> FallingFigure<'i, Dirs> {
    fn new(chamber: &'i Chamber<'i>, figure: &'i Figure, dirs: Dirs) -> Self {
        Self {
            chamber,
            figure,
            direction: dirs,
            position: Vector::new(2, (chamber.height() + 2) as isize),
        }
    }
}

impl<'i, Dirs: Iterator<Item = Direction>> Iterator for FallingFigure<'i, Dirs> {
    type Item = (&'i Figure, Vector);

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction.next() {
            None => None,
            Some(dir) => match dir {
                Left => {
                    todo!("Implement Left")
                }
                Right => {
                    let new_possible_pos = self.position.plus(&Vector::new(1, 0));
                    let can_move = self
                        .figure
                        .points
                        .iter()
                        .map(|v| v.plus(&new_possible_pos))
                        .all(|v| v.x < self.chamber.width() as isize);
                    if can_move {
                        self.position = new_possible_pos;
                        Some((self.figure, self.position))
                    } else {
                        Some((self.figure, self.position))
                    }
                }
                Down => todo!(),
            },
        }
    }
}

pub fn solve_task1<const W: usize>(file_content: &str) -> usize {
    let figures = get_figures();
    let dirs = parse::parse(file_content).collect::<Vec<_>>();
    let all_dirs = dirs.iter().cloned().flat_map(|dir| [dir, Down]);
    let chamber = Chamber::new(W);
    let falling_figure = FallingFigure::new(&chamber, &figures[0], all_dirs);
    for x in falling_figure {
        chamber.print(Some(x));
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
