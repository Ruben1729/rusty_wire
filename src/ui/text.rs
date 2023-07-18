use piston_window::types::FontSize;
use piston_window::types::ColorComponent;
use piston_window::{Context, G2d, G2dTextureContext, GfxDevice, Glyphs, Text, TextureSettings, Transformed};

pub struct UiText {
    coordinates: (f64, f64),
    font_size: FontSize,
    glyphs: Glyphs,
    color: [ColorComponent; 4],
    value: String
}

impl UiText {
    pub fn new(value: String,
               coordinates: (f64, f64),
               font_size: FontSize,
               color: [ColorComponent; 4],
               font: String,
               window_ctx: G2dTextureContext) -> Self {
        UiText {
            coordinates,
            font_size,
            glyphs: Glyphs::from_bytes(
                font.as_ref(),
                window_ctx,
                TextureSettings::new(),
            ).expect("Unable to load glyphs."),
            color,
            value
        }
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        Text::new_color(self.color, self.font_size).draw(
            &self.value,
            &mut self.glyphs,
            &c.draw_state,
            c.transform.trans(self.coordinates.0, self.coordinates.1).zoom(0.25),
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
    texture_ctx: G2dTextureContext,

    color: [ColorComponent; 4],
}

impl UiTextList {
    pub fn new(
        coordinates: (f64, f64),
        is_vertical: bool,
        font_size: FontSize,
        font: String,
        texture_ctx: G2dTextureContext,
        color: [ColorComponent; 4],
    ) -> Self {
        UiTextList {
            items: Vec::new(),

            coordinates,
            is_vertical,

            font_size,
            font,
            texture_ctx,

            color
        }
    }

    pub fn push(mut self, text: String) {
        let mut text_coords = self.coordinates;

        if self.is_vertical {
            text_coords.1 += self.items.len() as f64 * self.font_size as f64;
        } else {
            text_coords.0 += self.items.len() as f64 * self.font_size as f64;
        }

        self.items.push(UiText::new(
            text,
            text_coords,
            self.font_size,
            self.color,
            self.font,
            self.texture_ctx,
        ));
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        for mut item in self.items {
            item.render(c, g, device);
        }
    }
}
