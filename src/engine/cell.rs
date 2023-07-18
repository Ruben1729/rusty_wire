use piston_window::types::ColorComponent;

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    ElectronHead,
    ElectronTail,
    Conductor,
}

impl Cell {
    pub fn get_color(&self) -> [ColorComponent; 4] {
        match *self {
            Cell::Empty =>        [0.0, 0.0, 0.0, 1.0],
            Cell::ElectronHead => [0.0, 0.0, 1.0, 1.0],
            Cell::ElectronTail => [1.0, 0.0, 0.0, 1.0],
            Cell::Conductor =>    [1.0, 1.0, 0.0, 1.0],
        }
    }
}
