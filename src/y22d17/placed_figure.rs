use super::figure::Figure;
use super::vector::Vector;

pub struct PlacedFigure<'i> {
    pub figure: &'i Figure,
    pub left_bottom: Vector,
}

impl<'i> PlacedFigure<'i> {
    fn new(figure: &'i Figure, left_bottom: Vector) -> Self {
        Self {
            figure,
            left_bottom,
        }
    }
}
