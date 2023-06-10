use piston_window::{Button, Key, MouseButton};
use crate::lib::editor::brush::*;

#[derive(Clone)]
pub struct InputHandler {
    pub brush: Brush,
    pub keys_pressed: Vec<Button>
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            brush: Brush::new(1, BrushStyle::Conductor, BrushState::Released),
            keys_pressed: Vec::new()
        }
    }

    pub fn key_press(&mut self, button: Button) {
        if button == Button::Mouse(MouseButton::Left) {
            self.brush.state = BrushState::Paint;
        } else if button == Button::Mouse(MouseButton::Right) {
            self.brush.state = BrushState::Erase;
        } else if button == Button::Keyboard(Key::E) {
            self.brush.style = BrushStyle::ElectronHead;
        } else if button == Button::Keyboard(Key::T) {
            self.brush.style = BrushStyle::ElectronTail;
        } else if button == Button::Keyboard(Key::C) {
            self.brush.style = BrushStyle::Conductor;
        }
    }

    pub fn key_release(&mut self, button: Button) {
        if button == Button::Mouse(MouseButton::Left) || button == Button::Mouse(MouseButton::Right) {
            self.brush.state = BrushState::Released;
            return;
        }

        self.keys_pressed.retain(|&x| x != button);
    }
}
