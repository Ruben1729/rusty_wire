extern crate piston_window;
use piston_window::*;

use crate::engine::camera::Camera;
use crate::engine::cell::Cell;

pub mod camera;
pub mod cell;

pub struct Engine<const WIDTH: usize, const HEIGHT: usize> {
    // Grid of cells
    grid: [[Cell; WIDTH]; HEIGHT],

    // Logic
    events: Events,

    // Display
    window: PistonWindow,
    camera: Camera
}

impl<const WIDTH: usize, const HEIGHT: usize> Engine<WIDTH, HEIGHT> {
    pub fn new(display_width: u32, display_height: u32) -> Self {
        let events = Events::new(EventSettings::new().ups(60).max_fps(60));
        let window: PistonWindow = WindowSettings::new("RustyWire",
                                                       [display_width, display_height])
            .exit_on_esc(true)
            .build()
            .expect("Unable to create PistonWindow.");

        Engine {
            grid: [[Cell::Empty; WIDTH];HEIGHT],
            events,
            window,
            camera: Camera::new((0.0, 0.0), 1)
        }
    }

    pub fn start(&mut self) {
        self.run();
    }

    fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            self.update();
            self.render();
        }
    }

    pub fn render(&mut self) {

    }

    pub fn update(&mut self) {
        let mut new_world = self.clone();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                new_world.grid[i][j] = match cell {
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
