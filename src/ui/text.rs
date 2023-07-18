use piston_window::types::FontSize;
use piston_window::types::ColorComponent;
use piston_window::{Context, G2d, GfxDevice, Glyphs, TextureSettings};

pub struct UiText {
    coordinates: (f64, f64),
    font_size: FontSize,
    glyphs: Glyphs,
    color: ColorComponent,
    value: String
}

impl UiText {
    pub fn new(value: String,
               coordinates: (f64, f64),
               font_size: FontSize,
               color: ColorComponent,
               font: String) -> Self {
        UiText {
            coordinates,
            font_size,
            glyphs: Glyphs::from_bytes(
                font.as_ref(),
                texture_ctx,
                TextureSettings::new(),
            ).expect("Unable to load glyphs."),
            color,
            value
        }
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        UiText::new_color(self.color, self.font_size).draw(
            &self.value,
            &self.glyphs,
            &c.draw_state,
            c.transform.trans(horizontal_mod, vertical_mod).zoom(0.25),
            g
        ).expect("Unable to render text.");

        self.glyphs.factory.encoder.flush(device);
    }
}

pub struct UiTextList {
    items: Vec<UiText>,

    coordinates: (f64, f64),
    is_vertical: bool,

    font_size: FontSize,
    font: String,

    color: ColorComponent,
}

impl UiTextList {
    pub fn new(
        coordinates: (f64, f64),
        is_vertical: bool,
        font_size: FontSize,
        font: String,
        color: ColorComponent,
    ) -> Self {
        UiTextList {
            items: Vec::new(),

            coordinates,
            is_vertical,

            font_size,
            font,

            color
        }
    }

    pub fn push(&mut self, text: String) {
        let mut text_coords = self.coordinates;

        if self.is_vertical {
            text_coords.1 += self.items.len() * self.font_size;
        } else {
            text_coords.0 += self.items.len() * self.font_size;
        }

        self.items.push(UiText::new(
            text,
            text_coords,
            self.font_size,
            self.color,
            self.font.copy()
        ));
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        for mut item in self.items {
            item.render(c, g, device);
        }
    }
}
