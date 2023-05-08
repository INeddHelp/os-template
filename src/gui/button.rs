use crate::gui::{Color, Drawable};

pub struct Button {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    label: String,
    color: Color,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, label: &str, color: Color) -> Self {
        Button {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            color,
        }
    }
}

impl Drawable for Button {
    fn draw(&self) {
        // TODO: Code to draw button using GUI library
    }
}
