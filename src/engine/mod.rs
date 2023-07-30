extern crate piston_window;
use piston_window::*;
use crate::editor::brush::{Brush, BRUSH_PATTERN_XOR, BrushState};

use crate::engine::cell::Cell;
use crate::engine::grid::Grid;

pub mod cell;
pub mod grid;

pub struct Engine<const WIDTH: usize, const HEIGHT: usize> {
    // Grid of cells
    grid: Grid<WIDTH, HEIGHT>,

    // Logic
    events: Events,
    sim_step_speed: usize,
    sim_step: usize,
    brush: Brush,

    // Display
    window: PistonWindow,
    pixel_size: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> Engine<WIDTH, HEIGHT> {
    pub fn new(pixel_size: usize) -> Self {
        let events = Events::new(EventSettings::new().ups(60).max_fps(60));
        let mut window: PistonWindow = WindowSettings::new("RustyWire",
                                                           [(WIDTH * pixel_size) as u32, (HEIGHT * pixel_size) as u32])
        .exit_on_esc(true)
        .build()
        .expect("Unable to create PistonWindow.");
        let grid: Grid<WIDTH, HEIGHT> = Grid::new();

        Engine {
            grid,
            events,
            sim_step_speed: 5,
            sim_step: 0,
            brush: Brush::new(pixel_size, &mut window),
            window,
            pixel_size
        }
    }

    pub fn start(&mut self) {
        // Base pattern
        self.brush.set_pattern(BRUSH_PATTERN_XOR.iter()
                                .map(|row| row.to_vec())
                                .collect());
        self.sim_step = 0;
        self.run();
    }

    fn run(&mut self) {
        while let Some(event) = self.events.next(&mut self.window) {
            self.handle_event(&event);
            self.render(event);
        }
    }

    pub fn render(&mut self, e: Event) {
        // Draw graphics
        self.window.draw_2d(&e, |c, g, device| {
            // UPDATE
            self.sim_step += 1;

            if self.sim_step >= 60 / self.sim_step_speed {
                self.grid.update();
                self.sim_step = 0;
            }

            if self.brush.state() == BrushState::Paint {
                for (dx, row) in self.brush.pattern().iter().enumerate() {
                    for (dy, &cell)  in row.iter().enumerate() {
                        let coords = self.brush.coordinates();
                        let x = coords.0 + dx;
                        let y = coords.1 + dy;

                        if x < WIDTH && y < HEIGHT {
                            self.grid.set_cell(y, x, cell);
                        }
                    }
                }
            }

            // Clear canvas
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.grid.render(|x, y, cell| {
                rectangle(
                    cell.get_color(),
                    [
                        (x * self.pixel_size) as f64,
                        (y * self.pixel_size) as f64,
                        self.pixel_size as f64,
                        self.pixel_size as f64
                    ],
                    c.transform,
                    g);
            });

            self.brush.render(c, g, device);
        });
    }

    fn handle_event(&mut self, event: &Event) {
        self.brush.handle_event(&event, &mut self.window);
    }
}

pub(crate) trait EventHandler {
    fn handle_event(&mut self, event: &Event, window: &mut PistonWindow);
}
