use crate::components::Component;
use crate::graphics::{Color, Drawable, Font, RenderTarget, Text};

pub struct Label<'a> {
    text: Text<'a>,
    position: (f32, f32),
    color: Color,
}

impl<'a> Label<'a> {
    pub fn new(font: &'a Font, text: &str, position: (f32, f32), color: Color) -> Self {
        let mut label = Self {
            text: Text::new(text, font, 16.0),
            position,
            color,
        };
        label.set_text(text);
        label
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.set_string(text);
        let bounds = self.text.local_bounds();
        self.text.set_origin((bounds.width / 2.0, bounds.height / 2.0));
        self.text.set_position(self.position);
        self.text.set_color(self.color);
    }
}

impl<'a> Component for Label<'a> {
    fn draw(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.text);
    }
}
