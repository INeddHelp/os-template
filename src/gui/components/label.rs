use crate::gui::Drawable;

pub struct Label {
    text: String,
    x: i32,
    y: i32,
}

impl Label {
    pub fn new(text: String, x: i32, y: i32) -> Self {
        Self { text, x, y }
    }
}

impl Drawable for Label {
    fn draw(&self) {
        println!("Drawing label with text: '{}' at ({}, {})", self.text, self.x, self.y);
    }
}
