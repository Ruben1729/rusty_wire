use piston_window::{G2dTextureContext, Glyphs, PistonWindow, text, TextureSettings};
use piston_window::types::FontSize;

pub struct UiTextList {
    // Relative origin
    coordinates: [usize; 2],

    // layout
    is_vertical: bool,

    // Font
    font_size: FontSize,
    glyphs: Glyphs,

    // Text to display
    stack: Vec<String>
}

impl UiTextList {
    pub fn new(coordinates: [usize; 2],
               is_vertical: bool,
               font_size: FontSize,
               font: &str,
               texture_ctx: G2dTextureContext) -> Self {
        UiTextList {
            coordinates,
            is_vertical,
            font_size,
            glyphs: Glyphs::from_bytes(
                include_bytes!("../../jetbrains.ttf"),
                texture_ctx,
                TextureSettings::new(),
            ).unwrap(),
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, text: &str) {
        self.stack.push(text.parse().unwrap());
    }

    pub fn step(&mut self) {
        // let mut vertical_mod: usize = self.coordinates[0];
        // let mut horizontal_mod: usize = self.coordinates[1];

        // for (i, item) in self.stack.iter().enumerate() {
        //     text::Text::new_color([1.0, 1.0, 1.0, 1.0], (self.font_size * 4)).draw(
        //         item,
        //         &mut self.glyphs,
        //         &c.draw_state,
        //         c.transform.trans(horizontal_mod, vertical_mod).zoom(0.25),
        //         g
        //     ).unwrap();
        //
        //     if self.is_vertical {
        //         vertical_mod = vertical_mod + (self.font_size * i)
        //     } else {
        //         horizontal_mod = horizontal_mod + (self.font_size * i)
        //     }
        // }
    }
}
