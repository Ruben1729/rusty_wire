use piston_window::{Context, G2d, GfxDevice, rectangle};
use crate::engine::cell::Cell;

pub enum BrushState {
    Paint,
    Select,
    Erase
}

pub struct Brush {
    coordinates: (usize, usize),
    pattern: Vec<Vec<Cell>>,
    state: BrushState,
    scale: usize
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            coordinates: (0, 0),
            pattern: Vec::new(),
            state: BrushState::Paint,
            scale: 1
        }
    }

    pub fn set_scale(&mut self, scale: usize) {
        self.scale = scale;
    }

    pub fn update_position(&mut self, cursor_pos: [f64; 2]) {
        self.coordinates.0 = (cursor_pos[0] / 10.0).floor() as usize;
        self.coordinates.1 = (cursor_pos[1] / 10.0).floor() as usize;
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        for x in self.coordinates.0..(self.coordinates.0 + self.pattern.len()) {
            for y in self.coordinates.1..(self.coordinates.1 + self.pattern[x].len()) {
                let mut hover_color = self.pattern[x][y].get_color();
                hover_color[3] = 0.3;

                rectangle(
                    hover_color,
                    [
                        (self.coordinates.0 * self.scale) as f64,
                        (self.coordinates.1 * self.scale) as f64,
                        self.scale as f64,
                        self.scale as f64
                    ],
                    c.transform,
                    g);
            }
        }
    }
}


