extern crate piston_window;

use std::fmt;
use piston_window::*;

#[derive(PartialEq, Clone)]
pub enum BrushStyle {
    Cluster,
    Conductor,
    ElectronHead,
    ElectronTail,
    Empty,
}

#[derive(PartialEq, Clone)]
pub enum BrushState {
    Paint,
    Select,
    Erase,
    Released
}

#[derive(Clone)]
pub struct Brush {
    pub size: usize,
    pub style: BrushStyle,
    pub state: BrushState,
    pub pos: [usize; 2]
}

impl fmt::Display for BrushStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            BrushStyle::Cluster => "Cluster",
            BrushStyle::Conductor => "Conductor",
            BrushStyle::ElectronHead => "Electron Head",
            BrushStyle::ElectronTail => "Electron Tail",
            BrushStyle::Empty => "Empty",
            _ => todo!()
        };
        write!(f, "{}", printable)
    }
}

impl fmt::Display for BrushState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            BrushState::Paint => "Paint",
            BrushState::Select => "Select",
            BrushState::Erase => "Erase",
            BrushState::Released => "Released"
        };
        write!(f, "{}", printable)
    }
}

impl Brush {
    pub fn new(size: usize, style: BrushStyle, state: BrushState) -> Self {
        Brush {
            size,
            style,
            state,
            pos: [0, 0]
        }
    }

    pub fn increase_size(&mut self, modifier: usize) {
        self.size += modifier;
    }

    pub fn update_position(&mut self, cursor_pos: [f64; 2]) {
        self.pos[0] = (cursor_pos[0] / 10.0).floor() as usize;
        self.pos[1] = (cursor_pos[1] / 10.0).floor() as usize;
    }
}

