use crate::gui::style::{Color, Style};

pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary_color: Color::from_rgb(0, 0, 255),
            secondary_color: Color::from_rgb(255, 255, 255),
        }
    }
}

impl Theme {
    pub fn get_style(&self) -> Style {
        Style {
            background_color: Some(self.secondary_color),
            border_color: Some(self.primary_color),
            border_width: Some(1),
            padding: Some(Default::default()),
            margin: Some(Default::default()),
            font_size: Some(16),
            text_color: Some(Color::from_rgb(0, 0, 0)),
        }
    }
}
