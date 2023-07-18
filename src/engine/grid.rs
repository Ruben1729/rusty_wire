use crate::engine::cell::Cell;

pub struct Grid<const WIDTH: usize, const HEIGHT: usize> {
    value: [[Cell;WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Grid {
            value: [[Cell::Empty; WIDTH]; HEIGHT]
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        if x < WIDTH && y < HEIGHT {
            self.value[y][x]
        } else {
            Cell::Empty
        }
    }

    pub fn update(&mut self) {
        let mut new_world = self.value.clone();
        for (i, row) in self.value.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                new_world[i][j] = match cell {
                    Cell::ElectronHead => Cell::ElectronTail,
                    Cell::ElectronTail => Cell::Conductor,
                    Cell::Conductor => {
                        let heads = self.count_heads(i, j);
                        if heads == 1 || heads == 2 {
                            Cell::ElectronHead
                        } else {
                            Cell::Conductor
                        }
                    }
                    Cell::Empty => Cell::Empty
                };
            }
        }
        self.value = new_world;
    }

    fn count_heads(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                if i == x && j == y {
                    continue;
                }

                if self.value[i][j] == Cell::ElectronHead {
                    count += 1;
                }
            }
        }

        count
    }
}