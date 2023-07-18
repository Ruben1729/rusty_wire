extern crate piston_window;

use std::collections::HashMap;
use piston_window::*;

const FONT_SIZE: u32 = 25;

pub struct Cluster {

}

pub struct UiSearchBar {
    // Search
    query: String,
    results: Vec<String>,
    database: Vec<String>,

    // Font
    glyphs: Glyphs,

    // Relative origin
    coordinates: [f64; 2],
}

impl UiSearchBar {
    pub fn new(coordinates: [f64; 2], texture_ctx: G2dTextureContext) -> Self {
        UiSearchBar {
            query: String::new(),
            database: Vec::new(),
            results: Vec::new(),
            glyphs: Glyphs::from_bytes(
                include_bytes!("../assets/fonts/jetbrains.ttf"),
                texture_ctx,
                TextureSettings::new(),
            ).unwrap(),
            coordinates
        }
    }

    pub fn push(&mut self, text: &str) {
        self.database.push(text.parse().unwrap());
    }

    pub fn pop(&mut self) {
        self.query.pop();
    }

    pub fn append(&mut self, text: char) {
        self.query.push(text);
    }

    pub fn step(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        Text::new_color([1.0, 1.0, 1.0, 1.0], FONT_SIZE * 4).draw(
            "Search:",
            &mut self.glyphs,
            &c.draw_state,
            c.transform.trans(120.0, 30.0).zoom(0.25),
            g,
        ).unwrap();

        Text::new_color([1.0, 1.0, 1.0, 1.0], FONT_SIZE * 4).draw(
            &self.query,
            &mut self.glyphs,
            &c.draw_state,
            c.transform.trans(250.0, 30.0).zoom(0.25),
            g,
        ).unwrap();

        let mut vertical_mod: f64 = 60.0;

        for (i, item) in self.results.iter().enumerate() {
            Text::new_color([1.0, 1.0, 1.0, 1.0], (FONT_SIZE * 4)).draw(
                item,
                &mut self.glyphs,
                &c.draw_state,
                c.transform.trans(250.0, vertical_mod).zoom(0.25),
                g
            ).unwrap();

            vertical_mod += FONT_SIZE as f64 + 2.0;
        }

        self.glyphs.factory.encoder.flush(device);
    }

    pub fn filter_strings(&mut self) {
        self.results.clear();

        for string in &self.database {
            if string.contains(&self.query) {
                self.results.push(string.clone());
            }
        }
    }
}
