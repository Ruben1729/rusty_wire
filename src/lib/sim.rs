// Graphics Library
extern crate piston_window;
use piston_window::*;

// Cells
use crate::lib::cell::Cell;
use crate::lib::editor::brush::{BrushState, BrushStyle};

// Event Handler
use crate::lib::editor::input_handler::InputHandler;

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
            [10, 10],
            false,
            20,
            "jetbrains.ttf",
            window.create_texture_context()
        );

        text_key.push("keymap");
        text_key.push("c: conductor");
        text_key.push("e: electron head");
        text_key.push("d: diode");

        let mut sim_step = 0;

        while let Some(e) = events.next(&mut window) {
            // Register input events
            e.mouse_cursor(|pos| {
                self.input_handler.brush.update_position(pos);
            });

            // no need to track if its released, press is only useful for painting
            if let Some(button) = e.press_args() {
                self.input_handler.key_press(button);
            }

            if let Some(button) = e.release_args() {
                self.input_handler.key_release(button);
            }

            if self.input_handler.brush.state == BrushState::Paint {
                if self.input_handler.brush.style == BrushStyle::ElectronHead {
                    self.canvas[self.input_handler.brush.pos[0]][self.input_handler.brush.pos[1]] = Cell::ElectronHead
                } else if self.input_handler.brush.style == BrushStyle::Conductor {
                    self.canvas[self.input_handler.brush.pos[0]][self.input_handler.brush.pos[1]] = Cell::Conductor
                }
            } else if self.input_handler.brush.state == BrushState::Erase {
                self.canvas[self.input_handler.brush.pos[0]][self.input_handler.brush.pos[1]] = Cell::Empty
            }

            // Draw graphics
            window.draw_2d(&e, |c, g, device| {
                // Update Map
                if !self.is_paused {
                    sim_step += 1;
                }

                if sim_step >= 60 / self.sim_step_speed {
                    self.next();
                    sim_step = 0;
                }

                // Clear canvas
                clear([0.0, 0.0, 0.0, 1.0], g);

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

