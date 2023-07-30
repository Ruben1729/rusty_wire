use piston_window::{Context, G2d, G2dTextureContext, GfxDevice};
use piston_window::types::{ColorComponent, FontSize};
use crate::ui::text::UiText;

pub struct UiTextScroll {
    items: Vec<UiText>,
    active_idx: isize,

    coordinates: (f64, f64),

    font_size: FontSize,
    texture_ctx: G2dTextureContext,

    color: [ColorComponent; 4],
}

impl UiTextScroll {
    pub fn new(
        coordinates: (f64, f64),
        font_size: FontSize,
        texture_ctx: G2dTextureContext,
        color: [ColorComponent; 4],
    ) -> Self {
        UiTextScroll {
            items: Vec::new(),
            active_idx: 0,

            coordinates,

            font_size,
            texture_ctx,

            color
        }
    }

    pub fn push(&mut self, text: String, texture_ctx: G2dTextureContext) {
        self.items.push(UiText::new(
            text,
            self.coordinates,
            self.font_size,
            self.color,
            texture_ctx,
        ));
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, device: &mut GfxDevice) {
        let prev_idx = (self.active_idx + 1) % self.items.len() as isize;
        let next_idx = (self.active_idx - 1 + self.items.len() as isize) % self.items.len() as isize;

        if let Some(item) = self.items.get_mut(prev_idx as usize) {
            item.set_coordinates((self.coordinates.0 - 5.0, self.coordinates.1 - 15.0));
            item.set_opacity(0.3);
            item.render(c, g, device);
        };

        if let Some(item) = self.items.get_mut(next_idx as usize) {
            item.set_coordinates((self.coordinates.0 - 5.0, self.coordinates.1 + 15.0));
            item.set_opacity(0.3);
            item.render(c, g, device);
        };

        if let Some(item) = self.items.get_mut(self.active_idx as usize){
            item.set_coordinates(self.coordinates);
            item.set_opacity(1.0);
            item.render(c, g, device)
        };
    }

    pub fn increase(&mut self) {
        self.active_idx = (self.active_idx + 1) % self.items.len() as isize;
    }

    pub fn decrease(&mut self) {
        self.active_idx = (self.active_idx - 1 + self.items.len() as isize) % self.items.len() as isize;
    }

    pub fn active_idx(&self) -> isize {
        self.active_idx
    }
}