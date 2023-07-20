extern crate piston_window;
use piston_window::*;
use piston_window::types::FontSize;

pub struct UiTextList {
    // Relative origin
    coordinates: [f64; 2],

    // layout
    is_vertical: bool,

    // Font
    font_size: FontSize,
    glyphs: Glyphs,

    // Text to display
    stack: Vec<String>
}

impl UiTextList {
    pub fn new(coordinates: [f64; 2],
               is_vertical: bool,
               font_size: FontSize,
               texture_ctx: G2dTextureContext) -> Self {
        UiTextList {
            coordinates,
            is_vertical,
            font_size,
            glyphs: Glyphs::from_bytes(
                include_bytes!("../assets/fonts/jetbrains.ttf"),
                texture_ctx,
                TextureSettings::new(),
            ).unwrap(),
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, text: &str) {
        self.stack.push(text.parse().unwrap());
    }

    pub fn step(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        let mut horizontal_mod: f64 = self.coordinates[0];
        let mut vertical_mod: f64 = self.coordinates[1];

        for (i, item) in self.stack.iter().enumerate() {
            Text::new_color([1.0, 1.0, 1.0, 1.0], (self.font_size * 4)).draw(
                item,
                &mut self.glyphs,
                &c.draw_state,
                c.transform.trans(horizontal_mod, vertical_mod).zoom(0.25),
                g
            ).unwrap();

            if self.is_vertical {
                vertical_mod += self.font_size as f64 + 2.0;
            } else {
                horizontal_mod += self.font_size as f64 + 2.0;
            }
        }

        self.glyphs.factory.encoder.flush(device);
    }
}
