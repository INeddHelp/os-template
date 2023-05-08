use crate::gui::color::Color;

pub struct DarkTheme;

impl DarkTheme {
    pub fn background_color() -> Color {
        Color::new(0x1E, 0x1E, 0x1E)
    }

    pub fn foreground_color() -> Color {
        Color::new(0xFF, 0xFF, 0xFF)
    }

    pub fn primary_color() -> Color {
        Color::new(0x03, 0xA9, 0xF4)
    }

    pub fn accent_color() -> Color {
        Color::new(0xFF, 0x57, 0x22)
    }

    pub fn error_color() -> Color {
        Color::new(0xFF, 0x18, 0x00)
    }

    pub fn text_size() -> u32 {
        14
    }
}
