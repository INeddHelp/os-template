use crate::gui::event::{Event, Key};
use crate::gui::layout::{Constraint, Direction, Layout, Margin, Size};
use crate::gui::style::{Color, Style};
use crate::gui::{Context, Rect};

pub struct TextBox {
    id: String,
    value: String,
    placeholder: String,
    cursor_pos: usize,
    focused: bool,
    on_change: Option<Box<dyn Fn(&str)>>,
    style: Style,
}

impl TextBox {
    pub fn new(id: &str) -> Self {
        TextBox {
            id: id.to_string(),
            value: String::new(),
            placeholder: String::new(),
            cursor_pos: 0,
            focused: false,
            on_change: None,
            style: Style::default(),
        }
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn on_change<F: 'static + Fn(&str)>(mut self, f: F) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        match event {
            Event::Key(Key::Backspace) => {
                if self.cursor_pos > 0 {
                    self.value.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
                true
            }
            Event::Key(Key::Delete) => {
                if self.cursor_pos < self.value.len() {
                    self.value.remove(self.cursor_pos);
                }
                true
            }
            Event::Key(Key::Left) => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
                true
            }
            Event::Key(Key::Right) => {
                if self.cursor_pos < self.value.len() {
                    self.cursor_pos += 1;
                }
                true
            }
            Event::Key(Key::Char(c)) => {
                self.value.insert(self.cursor_pos, *c);
                self.cursor_pos += 1;
                true
            }
            _ => false,
        }
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size {
        let padding = self.style.padding.unwrap_or_default();
        let margin = self.style.margin.unwrap_or_default();
        let width = constraints.width.unwrap_or_else(|| {
            context
                .get_text_size(&self.placeholder, &self.style)
                .width
        });
        let height = constraints.height.unwrap_or_else(|| {
            context
                .get_text_size(&self.placeholder, &self.style)
                .height
        });

        let mut layout = Layout::new(width, height);
        layout
            .add_child(
                Margin::new(
                    padding.left,
                    padding.top,
                    padding.right,
                    padding.bottom,
                    Label::new(&self.value, &self.style),
                ),
                0.0,
                Direction::LeftToRight,
            )
            .add_child(
                Margin::new(0, 0, 0, 0, Label::new(&self.placeholder, &self.style)),
                0.0,
                Direction::LeftToRight,
            );

        let size = layout.layout(context, &constraints);
        self.style.size = Some(size);

        size.inflate(padding.left + padding.right, padding.top + padding.bottom)
            .inflate(margin.left + margin.right, margin.top + margin.bottom)
    }

    pub fn draw(&self, context: &mut Context, clip: Rect) {
        let padding = self.style.padding.unwrap_or_default();
        let margin = self.style.margin.unwrap_or_default();
        let size = self.style.size.unwrap_or_default();

        let mut rect = Rect::new(
            margin.left as f32,
            margin.top as f32,
            size.width as f32,
            size.height as f32,
        );
        rect = rect.clip(clip);

        let background_color = if self.focused {
            self.style.active_background_color.unwrap_or(self.style.background_color)
        } else {
            self.style.background_color
        };
        context.draw_rect(rect, background_color);

        let mut label_rect = Rect::new(
            (padding.left + self.cursor_pos * 8) as f32,
            padding.top as f32,
            0.0,
            0.0,
        );
        label_rect = label_rect.clip(rect);
        let label_clip = Rect::new(
            label_rect.x - padding.left as f32,
            label_rect.y - padding.top as f32,
            rect.width - padding.left as f32 - padding.right as f32,
            rect.height - padding.top as f32 - padding.bottom as f32,
        );
        Label::new(&self.value, &self.style).draw(context, label_clip);

        if self.value.is_empty() {
            let placeholder_clip = Rect::new(
                padding.left as f32,
                padding.top as f32,
                rect.width - padding.left as f32 - padding.right as f32,
                rect.height - padding.top as f32 - padding.bottom as f32,
            );
            Label::new(&self.placeholder, &self.style.placeholder_style()).draw(context, placeholder_clip);
        }

        if self.focused {
            let line_rect = Rect::new(
                (padding.left + self.cursor_pos * 8) as f32,
                rect.height - padding.bottom as f32 - 2.0,
                8.0,
                2.0,
            );
            context.draw_rect(line_rect, self.style.color.unwrap_or_default());
        }
    }
}