// Graphics Library
extern crate piston_window;
use piston_window::*;

// Cells
use crate::lib::cell::cell::Cell;
use crate::lib::editor::brush::{BrushState, BrushStyle};

// Event Handler
use crate::lib::editor::input_handler::InputHandler;
use crate::lib::editor::ui_searchbar::UiSearchBar;

// UI
use crate::lib::editor::ui_text::*;

#[derive(Clone)]
pub struct Simulation<const WIDTH: usize, const HEIGHT: usize> {
    // Graphics related variables
    canvas:[[Cell; WIDTH]; HEIGHT],
    width: usize,
    height: usize,
    pixel_size: usize,

    // Simulation related variables
    is_paused: bool,
    sim_step_speed: usize,

    // Events
    input_handler: InputHandler,
}

impl<const WIDTH: usize, const HEIGHT: usize> Simulation<WIDTH, HEIGHT> {
    pub fn new(pixel_size: usize) -> Self {
        Simulation {
            canvas: [[Cell::Empty; WIDTH]; HEIGHT],
            width: WIDTH,
            height: HEIGHT,
            pixel_size,

            is_paused: false,
            sim_step_speed: 5,

            input_handler: InputHandler::new()
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
        let mut window: PistonWindow = WindowSettings::new("RustyWire", [(self.width * self.pixel_size) as u32, (self.height * self.pixel_size) as u32])
        .exit_on_esc(true)
            .build()
            .unwrap();

        let mut text_key: UiTextList = UiTextList::new(
            [10.0, 20.0],
            true,
            20,
            window.create_texture_context()
        );

        text_key.push("keymap");
        text_key.push("q:       conductor");
        text_key.push("w:       electron head");
        text_key.push("e:       electron tail");
        text_key.push("space:   play/pause");
        text_key.push("esc:     quit");

        let mut sim_step = 0;

        while let Some(e) = events.next(&mut window) {
            // Register input events
            e.mouse_cursor(|pos| {
                self.input_handler.brush.update_position(pos);
            });

            // no need to track if its released, press is only useful for painting
            if let Some(button) = e.press_args() {
                if button == Button::Keyboard(Key::Space) {
                    self.is_paused = !self.is_paused;
                }
                self.input_handler.key_press(button);
            }

            if let Some(button) = e.release_args() {
                self.input_handler.key_release(button);
            }

            if self.input_handler.brush.state == BrushState::Paint {
                self.canvas[self.input_handler.brush.pos[0]][self.input_handler.brush.pos[1]] = match self.input_handler.brush.style {
                    BrushStyle::Conductor => Cell::Conductor,
                    BrushStyle::ElectronHead => Cell::ElectronHead,
                    BrushStyle::ElectronTail => Cell::ElectronTail,
                    _ => Cell::Empty
                };
            } else if self.input_handler.brush.state == BrushState::Erase {
                self.canvas[self.input_handler.brush.pos[0]][self.input_handler.brush.pos[1]] = Cell::Empty
            }

            // Draw graphics
            window.draw_2d(&e, |c, g, device| {
                // Clear canvas
                clear([0.0, 0.0, 0.0, 1.0], g);

                // Update Map
                if !self.is_paused {
                    sim_step += 1;
                }

                if sim_step >= 60 / self.sim_step_speed {
                    self.next();
                    sim_step = 0;
                }

                // Update Colors
                for (i, row) in self.canvas.iter().enumerate() {
                    for (j, &cell) in row.iter().enumerate() {
                        let color = cell.get_color();
                        rectangle(
                            color,
                            [
                                (i * self.pixel_size) as f64,
                                (j * self.pixel_size) as f64,
                                self.pixel_size as f64,
                                self.pixel_size as f64
                            ],
                            c.transform,
                            g);
                    }
                }

                // Draw brush hover
                let mut hover_color = match self.input_handler.brush.style {
                    BrushStyle::Cluster =>      Cell::Empty.get_color(),
                    BrushStyle::Conductor =>    Cell::Conductor.get_color(),
                    BrushStyle::ElectronHead => Cell::ElectronHead.get_color(),
                    BrushStyle::ElectronTail => Cell::ElectronTail.get_color(),
                    BrushStyle::Empty =>        Cell::Empty.get_color()
                };

                hover_color[3] = 0.3;

                rectangle(
                    hover_color,
                    [
                        (self.input_handler.brush.pos[0] * self.pixel_size) as f64,
                        (self.input_handler.brush.pos[1] * self.pixel_size) as f64,
                        self.pixel_size as f64,
                        self.pixel_size as f64
                    ],
                    c.transform,
                    g);

                // Update Text
                text_key.step(c, g, device);

                // Update Icons
                let mut sim_state_icon = match self.is_paused {
                    true => PLAY_ICON,
                    false => PAUSE_ICON
                };

                for (i, row) in sim_state_icon.iter().enumerate() {
                    for (j, &item) in row.iter().enumerate() {
                        let color = match item {
                            0 => [0.0, 0.0, 0.0, 1.0],
                            1 => [1.0, 1.0, 1.0, 1.0],
                            _ => [0.0, 0.0, 0.0, 1.0]
                        };

                        rectangle(
                            color,
                            [
                                ((j + 190) * 4) as f64,
                                ((i + 3) * 4) as f64,
                                4.0,
                                4.0
                            ],
                            c.transform,
                            g);
                    }
                }
            });
        }
    }

    fn next(&mut self) {
        let mut new_world = self.clone();
        for (i, row) in self.canvas.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                new_world.canvas[i][j] = match cell {
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
        *self = new_world
    }

    fn count_heads(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                if i == x && j == y {
                    continue;
                }

                if self.canvas[i][j] == Cell::ElectronHead {
                    count += 1;
                }
            }
        }

        count
    }
}

const PAUSE_ICON: [[usize;5];7] = [
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
];

const PLAY_ICON: [[usize;5];7] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 1, 1, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];

