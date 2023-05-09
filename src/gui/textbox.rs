use crate::event::{Event, Key};
use crate::gui::{Drawable, Rect};

pub struct Textbox {
    pub rect: Rect,
    pub text: String,
    pub cursor_pos: usize,
}

impl Textbox {
    pub fn new(x: i32, y: i32, width: u32) -> Self {
        Textbox {
            rect: Rect::new(x, y, width, 20),
            text: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            match key {
                Key::Backspace => {
                    if self.cursor_pos > 0 {
                        self.text.remove(self.cursor_pos - 1);
                        self.cursor_pos -= 1;
                    }
                }
                Key::Delete => {
                    if self.cursor_pos < self.text.len() {
                        self.text.remove(self.cursor_pos);
                    }
                }
                Key::Left => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                    }
                }
                Key::Right => {
                    if self.cursor_pos < self.text.len() {
                        self.cursor_pos += 1;
                    }
                }
                Key::Char(c) => {
                    self.text.insert(self.cursor_pos, *c);
                    self.cursor_pos += 1;
                }
                _ => {}
            }
        }
    }
}

impl Drawable for Textbox {
    fn draw(&self) {
        // TODO: Draw the border of the textbox
        // ...

        // TODO: Draw the text inside the textbox
        // ...

        // TODO: Draw the cursor
        // ...
    }
}
