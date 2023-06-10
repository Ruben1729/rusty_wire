use crate::lib::cell::Cell;

pub struct Cluster<const WIDTH: usize, const HEIGHT: usize> {
    canvas: [[Cell; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> Cluster<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Cluster {
            canvas: [[Cell::Empty; WIDTH]; HEIGHT]
        }
    }
}
