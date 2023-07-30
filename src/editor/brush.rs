use piston_window::{Button, Context, Event, G2d, G2dTextureContext, GfxDevice, Motion, MouseButton, MouseCursorEvent, MouseScrollEvent, PistonWindow, PressEvent, rectangle, ReleaseEvent, Window};
use piston_window::Input;
use crate::engine::cell::Cell;
use crate::engine::EventHandler;
use crate::ui::text_scroll::UiTextScroll;

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
    brush_id: Brushes,
    text_scroll: UiTextScroll,
    state: BrushState,
    pixel_size: usize
}

impl Brush {
    pub fn new(pixel_size: usize, mut window: &mut PistonWindow) -> Self {
        let mut text_scroll = UiTextScroll::new(
            (10.0, 30.0),
            50,
            window.create_texture_context(),
            [1.0, 1.0, 1.0, 1.0]
        );

        text_scroll.push(String::from("Conductor"), window.create_texture_context());
        text_scroll.push(String::from("Head"), window.create_texture_context());
        text_scroll.push(String::from("Tail"), window.create_texture_context());
        text_scroll.push(String::from("Diode"), window.create_texture_context());
        text_scroll.push(String::from("Xor"), window.create_texture_context());

        Brush {
            coordinates: (0, 0),
            pattern: Vec::new(),
            brush_id: Brushes::Diode,
            text_scroll,
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

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
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

        self.text_scroll.render(c, g, device);
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

        event.mouse_scroll(|args| {
            if args[1] > 0.0 {
                self.text_scroll.increase();
            } else if args[1] < 0.0 {
                self.text_scroll.decrease();
            }

            // TODO: Got a little lazy, make this so that it gets mapped to the enum somehow
            match self.text_scroll.active_idx() {
                1 => {
                    self.set_pattern(BRUSH_PATTERN_E_HEAD.iter()
                        .map(|row| row.to_vec())
                        .collect())
                }
                2 => {
                    self.set_pattern(BRUSH_PATTERN_E_TAIL.iter()
                        .map(|row| row.to_vec())
                        .collect())
                }
                3 => {
                    self.set_pattern(BRUSH_PATTERN_DIODE.iter()
                        .map(|row| row.to_vec())
                        .collect())
                }
                4 => {
                    self.set_pattern(BRUSH_PATTERN_XOR.iter()
                        .map(|row| row.to_vec())
                        .collect())
                }
                _ => {
                    self.set_pattern(BRUSH_PATTERN_CONDUCTOR.iter()
                        .map(|row| row.to_vec())
                        .collect())
                }
            }
        });
    }
}

#[repr(isize)]
pub enum Brushes {
    Conductor,
    Head,
    Tail,
    Diode,
    Xor
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
