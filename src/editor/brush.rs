use piston_window::{Button, Context, Event, G2d, GfxDevice, MouseButton, MouseCursorEvent, MouseScrollEvent, PistonWindow, PressEvent, rectangle, ReleaseEvent};
use crate::engine::cell::Cell;
use crate::engine::EventHandler;

#[derive(PartialEq, Clone, Copy)]
pub enum BrushState {
    Hover,
    Paint,
    Erase,
    Select,
}

pub struct Brush {
    coordinates: (usize, usize),
    pattern: Vec<Vec<Cell>>,
    state: BrushState,
    pixel_size: usize
}

impl Brush {
    pub fn new(pixel_size: usize) -> Self {
        Brush {
            coordinates: (0, 0),
            pattern: Vec::new(),
            state: BrushState::Hover,
            pixel_size
        }
    }

    pub fn state(&self) -> BrushState {
        self.state
    }

    pub fn pattern(&self) -> &Vec<Vec<Cell>> {
        &self.pattern
    }

    pub fn coordinates(&self) -> (usize, usize) {
        self.coordinates
    }

    pub fn set_pixel_size(&mut self, pixel_size: usize) {
        self.pixel_size = pixel_size;
    }

    pub fn set_pattern(&mut self, new_pattern: Vec<Vec<Cell>>) {
        self.pattern = new_pattern;
    }

    pub fn update_position(&mut self, cursor_pos: [f64; 2]) {
        self.coordinates.0 = (cursor_pos[0] / self.pixel_size as f64).floor() as usize;
        self.coordinates.1 = (cursor_pos[1] / self.pixel_size as f64).floor() as usize;
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, _device: &mut GfxDevice) {
        for (i, row) in self.pattern.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let mut hover_color = cell.get_color();
                hover_color[3] = 0.3;

                rectangle(
                    hover_color,
                    [
                        ((self.coordinates.0 + i) * self.pixel_size) as f64,
                        ((self.coordinates.1 + j) * self.pixel_size) as f64,
                        self.pixel_size as f64,
                        self.pixel_size as f64
                    ],
                    c.transform,
                    g);
            }
        }
    }
}

impl EventHandler for Brush {
    fn handle_event(&mut self, event: &Event, _: &mut PistonWindow) {
        event.mouse_cursor(|pos|    self.update_position(pos));

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                self.state = BrushState::Paint;
            } else if button == Button::Mouse(MouseButton::Right) {
                self.state = BrushState::Erase;
            }
        }

        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) ||
                button == Button::Mouse(MouseButton::Right) {
                self.state = BrushState::Hover;
            }
        }
    }
}

pub const BRUSH_PATTERN_CONDUCTOR: [[Cell; 1]; 1] = [[Cell::Conductor]];
pub const BRUSH_PATTERN_E_HEAD: [[Cell; 1]; 1] = [[Cell::ElectronHead]];
pub const BRUSH_PATTERN_E_TAIL: [[Cell; 1]; 1] = [[Cell::ElectronTail]];
pub const BRUSH_PATTERN_DIODE: [[Cell; 3]; 4] = [
    [Cell::Empty, Cell::Conductor, Cell::Empty],
    [Cell::Conductor, Cell::Conductor, Cell::Conductor],
    [Cell::Conductor, Cell::Empty, Cell::Conductor],
    [Cell::Empty, Cell::Conductor, Cell::Empty],
];

pub const BRUSH_PATTERN_XOR: [[Cell;7];5] = [
    [Cell::Conductor, Cell::Empty, Cell::Conductor, Cell::Conductor, Cell::Conductor, Cell::Empty, Cell::Conductor],
    [Cell::Empty, Cell::Conductor, Cell::Conductor, Cell::Empty, Cell::Conductor, Cell::Conductor, Cell::Empty],
    [Cell::Empty, Cell::Empty, Cell::Conductor, Cell::Empty, Cell::Conductor, Cell::Empty, Cell::Empty],
    [Cell::Empty, Cell::Empty, Cell::Conductor, Cell::Conductor, Cell::Conductor, Cell::Empty, Cell::Empty],
    [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Conductor, Cell::Empty, Cell::Empty, Cell::Empty],
];
