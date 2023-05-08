use crate::gui::layout::{Layout, Size, Margin, Constraints};
use crate::gui::style::Style;
use crate::gui::Context;

pub enum FlexDirection {
    Row,
    Column,
}

pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
}

pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
}

pub struct FlexContainer {
    children: Vec<Box<dyn Widget>>,
    layout: Layout,
    direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
}

impl FlexContainer {
    pub fn new() -> Self {
        FlexContainer {
            children: Vec::new(),
            layout: Layout::default(),
            direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
        }
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn add_child<T: Widget + 'static>(mut self, child: T) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size {
        // Lay out the container's children.
        let mut layout = Layout::new(constraints.width.unwrap_or(0), constraints.height.unwrap_or(0));
        let mut next_pos_x = 0.0;
        let mut next_pos_y = 0.0;
        let mut max_child_width = 0;
        let mut max_child_height = 0;
        let mut total_child_width = 0;
        let mut total_child_height = 0;
        let mut child_count = 0;
        for child in self.children.iter_mut() {
            let child_size = child.layout(context, Constraints {
                width: match self.direction {
                    FlexDirection::Row => None,
                    FlexDirection::Column => Some(constraints.width),
                },
                height: match self.direction {
                    FlexDirection::Row => Some(constraints.height),
                    FlexDirection::Column => None,
                },
            });
            let child_margin = child.style().margin.unwrap_or_default();
            if self.direction == FlexDirection::Row {
                if next_pos_x + child_size.width as f64 + child_margin.left as f64 + child_margin.right as f64 <= constraints.width.unwrap_or(0) as f64 {
                    layout.add_child(Margin::new(child_margin.top, child_margin.right, child_margin.bottom, child_margin.left, child), 1.0, Direction::LeftToRight);
                    if child_size.height as u32 > max_child_height {
                        max_child_height = child_size.height as u32;
                    }
                    next_pos_x += child_size.width as f64 + child_margin.left as f64 + child_margin.right as f64;
                    total_child_width += child_size.width as u32 + child_margin.left + child_margin.right;
                    child_count += 1;
                }
            } else {
                if next_pos_y + child_size.height as f64 + child_margin.top as f64 + child_margin.bottom as f64 <= constraints.height.unwrap_or(0) as f64 {
                    layout.add_child(Margin::new
                                         (child_margin.top, child_margin.right, child_margin.bottom, child_margin.left, child), 1.0, Direction::TopToBottom);
                    if child_size.width as u32 > max_child_width {
                        max_child_width = child_size.width as u32;
                    }
                    next_pos_y += child_size.height as f64 + child_margin.top as f64 + child_margin.bottom as f64;
                    total_child_height += child_size.height as u32 + child_margin.top + child_margin.bottom;
                    child_count += 1;
                }
            }
        }

        // Determine the size of the container based on its children and layout properties.
        let mut container_width = constraints.width.unwrap_or(0);
        let mut container_height = constraints.height.unwrap_or(0);
        match self.direction {
            FlexDirection::Row => {
                if self.layout.width == 0 {
                    container_width = total_child_width;
                }
                if self.layout.height == 0 {
                    container_height = max_child_height;
                }
            },
            FlexDirection::Column => {
                if self.layout.width == 0 {
                    container_width = max_child_width;
                }
                if self.layout.height == 0 {
                    container_height = total_child_height;
                }
            },
        }

        // Handle align items.
        let total_child_size = match self.direction {
            FlexDirection::Row => total_child_height,
            FlexDirection::Column => total_child_width,
        };
        let total_space = match self.direction {
            FlexDirection::Row => container_height,
            FlexDirection::Column => container_width,
        };
        let total_margin = match self.direction {
            FlexDirection::Row => {
                let margin = self.children.iter().map(|child| {
                    let margin = child.style().margin.unwrap_or_default();
                    margin.top + margin.bottom
                }).max().unwrap_or(0);
                (child_count - 1) * margin
            },
            FlexDirection::Column => {
                let margin = self.children.iter().map(|child| {
                    let margin = child.style().margin.unwrap_or_default();
                    margin.left + margin.right
                }).max().unwrap_or(0);
                (child_count - 1) * margin
            },
        };
        let total_free_space = (total_space - total_margin) as f64 - total_child_size as f64;
        let mut offset = match self.align_items {
            AlignItems::FlexStart => 0.0,
            AlignItems::FlexEnd => total_free_space,
            AlignItems::Center => total_free_space / 2.0,
        };

        // Position the children.
        for child in &mut self.children {
            let child_size = child.layout_size();
            let child_margin = child.style().margin.unwrap_or_default();

            match self.direction {
                FlexDirection::Row => {
                    let remaining_space = container_width as f64 - total_child_width as f64 - margin.left as f64 - margin.right as f64;
                    let extra_space_per_child = match self.justify_content {
                        JustifyContent::FlexStart => 0.0,
                        JustifyContent::FlexEnd => remaining_space / (child_count - 1) as f64,
                        JustifyContent::Center => remaining_space / (2 * child_count) as f64,
                        JustifyContent::SpaceBetween => remaining_space / (child_count - 1) as f64,
                        JustifyContent::SpaceAround => remaining_space / child_count as f64,
                    };
                    x += extra_space_per_child;

                    let align_offset = match self.align_items {
                        AlignItems::FlexStart => 0.0,
                        AlignItems::FlexEnd => container_height as f64 - child_size.height as f64 - child_margin.top as f64 - child_margin.bottom as f64,
                        AlignItems::Center => (container_height as f64 - child_size.height as f64 - child_margin.top as f64 - child_margin.bottom as f64) / 2.0,
                    };

                    child.set_position(Point::new(x + child_margin.left as f64, y + align_offset + child_margin.top as f64));
                    x += child_size.width as f64 + extra_space_per_child;
                },
                FlexDirection::Column => {
                    let remaining_space = container_height as f64 - total_child_height as f64 - margin.top as f64 - margin.bottom as f64;
                    let extra_space_per_child = match self.justify_content {
                        JustifyContent::FlexStart => 0.0,
                        JustifyContent::FlexEnd => remaining_space / (child_count - 1) as f64,
                        JustifyContent::Center => remaining_space / (2 * child_count) as f64,
                        JustifyContent::SpaceBetween => remaining_space / (child_count - 1) as f64,
                        JustifyContent::SpaceAround => remaining_space / child_count as f64,
                    };
                    y += extra_space_per_child;

                    let align_offset = match self.align_items {
                        AlignItems::FlexStart => 0.0,
                        AlignItems::FlexEnd => container_width as f64 - child_size.width as f64 - child_margin.left as f64 - child_margin.right as f64,
                        AlignItems::Center => (container_width as f64 - child_size.width as f64 - child_margin.left as f64 - child_margin.right as f64) / 2.0,
                    };

                    child.set_position(Point::new(x + align_offset + child_margin.left as f64, y + child_margin.top as f64));
                    y += child_size.height as f64 + extra_space_per_child;
                },
            }
        }

        Size::new(container_width, container_height)
    }

    pub fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
}

impl Widget for FlexContainer {
    fn layout(&mut self, context: &mut Context, constraints: Constraints) -> Size {
        self.layout(context, constraints)
    }
}

fn render(&self, context: &mut Context, bounds: Rect) {
    for child in &self.children {
        child.render(context, bounds);
    }
}
