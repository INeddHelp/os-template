use crate::gui::layout::{Layout, Size, Margin, Constraints};
use crate::gui::style::Style;
use crate::gui::Context;

pub struct Window {
    id: String,
    children: Vec<Box<dyn Widget>>,
    layout: Layout,
    title: String,
    position: (i32, i32),
    size: (u32, u32),
    style: Style,
}

impl Window {
    pub fn new(id: &str) -> Self {
        Window {
            id: id.to_string(),
            children: Vec::new(),
            layout: Layout::default(),
            title: String::new(),
            position: (0, 0),
            size: (800, 600),
            style: Style::default(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = (x, y);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn add_child<T: Widget + 'static>(mut self, child: T) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size {
        // Set the window's padding to the padding specified in its style, or to zero if none was specified.
        let padding = self.style.padding.unwrap_or_default();

        // Calculate the maximum available space for the window's contents.
        let available_space = Constraints {
            width: Some(constraints.width.unwrap_or(self.size.0) - padding.left - padding.right),
            height: Some(constraints.height.unwrap_or(self.size.1) - padding.top - padding.bottom),
            ..constraints
        };

        // Lay out the window's children.
        let mut layout = Layout::new(available_space.width.unwrap_or(0), available_space.height.unwrap_or(0));
        let mut next_pos_y = 0.0;
        for child in self.children.iter_mut() {
            let child_size = child.layout(context, available_space);
            layout.add_child(Margin::new(0, next_pos_y as i32, 0, 0, child), 1.0, Direction::LeftToRight);
            next_pos_y += child_size.height as f64;
        }

        // Add padding to the window's layout.
        layout.pad(padding.top, padding.right, padding.bottom, padding.left);

        // Set the window's layout.
        self.layout = layout;

        // Return the size of the window, including padding.
        Size {
            width: self.layout.width() + padding.left + padding.right,
            height: self.layout.height() + padding.top + padding.bottom,
        }
    }

    pub fn draw(&self, context: &mut Context, rect: Rect) {
        // Set the window's background color to the background color specified in its style, or to white if none was specified.
        let background_color = self.style.background_color.unwrap_or(Color::white());

        // Draw the window's background.
        context.fill_rect(rect, background_color);

        // Draw the window's title bar.
        let title_bar_height = 20;
        // Set the title bar's background color to the title bar color specified in the window's style, or to a dark gray if none was specified.
        let title_bar_color = self.style.title_bar_color.unwrap_or(Color::dark_gray());
        let title_bar_rect = Rect::new(rect.x, rect.y, rect.width, title_bar_height as u32);
        context.fill_rect(title_bar_rect, title_bar_color);
        // Draw the window's title text.
        let title_text_color = self.style.title_text_color.unwrap_or(Color::white());
        let font_size = self.style.title_font_size.unwrap_or(16);
        let font = context.get_font(font_size);
        let title_text_rect = Rect::new(rect.x + 5, rect.y + 2, rect.width - 10, title_bar_height as u32 - 4);
        context.draw_text(&self.title, font, title_text_rect, title_text_color, Align::Left);

        // Draw the window's border.
        let border_color = self.style.border_color.unwrap_or(Color::black());
        let border_thickness = self.style.border_thickness.unwrap_or(1);
        let border_rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
        context.stroke_rect(border_rect, border_thickness, border_color);

        // Draw the window's children.
        let content_rect = Rect::new(rect.x + 1, rect.y + title_bar_height as i32 + 1, rect.width - 2, rect.height - title_bar_height as u32 - 2);
        self.layout.draw(context, content_rect);
    }
}

pub trait Widget {
    fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size;
    fn draw(&self, context: &mut Context, rect: Rect);
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LeftToRight,
    TopToBottom,
}

pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Padding {
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Padding {
            top,
            right,
            bottom,
            left,
        }
    }
}

pub struct Margin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
    pub child: Box<dyn Widget>,
}

impl Margin {
    pub fn new(top: i32, right: i32, bottom: i32, left: i32, child: impl Widget + 'static) -> Self {
        Margin {
            top,
            right,
            bottom,
            left,
            child: Box::new(child),
        }
    }
}

impl Widget for Margin {
    fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size {
        let child_size = self.child.layout(context, constraints);
        Size {
            width: child_size.width + self.left as u32 + self.right as u32,
            height: child_size.height + self.top + height as u32 + self.bottom as u32,
        }
    }

    fn draw(&self, context: &mut Context, rect: Rect) {
// Calculate the rectangle for the child, taking the margin into account.
        let child_rect = Rect {
            x: rect.x + self.left,
            y: rect.y + self.top,
            width: rect.width - (self.left + self.right) as u32,
            height: rect.height - (self.top + self.bottom) as u32,
        };

// Draw the child.
        self.child.draw(context, child_rect);
    }
}

pub struct Spacer {
    width: Option<u32>,
    height: Option<u32>,
}

impl Spacer {
    pub fn new() -> Self {
        Spacer {
            width: None,
            height: None,
        }
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
}

impl Widget for Spacer {
    fn layout(&mut self, _context: &mut Context, constraints: Constraints) -> Size {
        Size {
            width: self.width.unwrap_or(constraints.width.unwrap_or(0)),
            height: self.height.unwrap_or(constraints.height.unwrap_or(0)),
        }
    }

    fn draw(&self, _context: &mut Context, _rect: Rect) {
        // A Spacer doesn't draw anything.
    }
}





