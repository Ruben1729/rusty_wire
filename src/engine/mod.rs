extern crate piston_window;
use piston_window::*;
use crate::editor::brush::Brush;

use crate::engine::camera::Camera;
use crate::engine::cell::Cell;
use crate::engine::grid::Grid;

pub mod camera;
pub mod cell;
pub mod grid;

pub struct Engine<const WIDTH: usize, const HEIGHT: usize> {
    // Grid of cells
    grid: Grid<WIDTH, HEIGHT>,

    // Logic
    events: Events,
    brush: Brush,

    // Display
    window: PistonWindow,
    camera: Camera
}

impl<const WIDTH: usize, const HEIGHT: usize> Engine<WIDTH, HEIGHT> {
    pub fn new(display_width: usize, display_height: usize) -> Self {
        let events = Events::new(EventSettings::new().ups(60).max_fps(60));
        let window: PistonWindow = WindowSettings::new("RustyWire",
                                                       [display_width as u32, display_height as u32])
        .exit_on_esc(true)
        .build()
        .expect("Unable to create PistonWindow.");
        let grid: Grid<WIDTH, HEIGHT> = Grid::new();

        Engine {
            grid,
            events,
            brush: Brush::new(),
            window,
            camera: Camera::new((0, 0), (display_width, display_height), 1, 5.0)
        }
    }

    pub fn start(&mut self) {
        self.run();
    }

    fn run(&mut self) {
        while let Some(event) = self.events.next(&mut self.window) {
            // Handle Event
            event.mouse_cursor(|pos|    self.brush.update_position(pos));

            // Update then Render
            self.update();
            self.render(event);
        }
    }

    pub fn render(&mut self, e: Event) {
        // Draw graphics
        self.window.draw_2d(&e, |c, g, device| {
            // Clear canvas
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.camera.render(|x, y, scale| {
                rectangle(
                    self.grid.get_cell(x, y).get_color(),
                    [
                        (x * scale) as f64,
                        (y * scale) as f64,
                        scale as f64,
                        scale as f64
                    ],
                    c.transform,
                    g);
            });
            self.brush.render(c, g, device);
        });
    }

    pub fn update(&mut self) {
        self.grid.update();
        self.camera.update();
    }
}
