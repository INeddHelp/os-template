use crate::gui::widget::{Widget, Rectangle};

pub struct Grid {
    rows: usize,
    columns: usize,
    cell_width: i32,
    cell_height: i32,
    widgets: Vec<Box<dyn Widget>>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize, cell_width: i32, cell_height: i32) -> Self {
        Self {
            rows,
            columns,
            cell_width,
            cell_height,
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>, row: usize, column: usize) {
        self.widgets.push(widget);
        widget.set_position(Rectangle {
            x: (column as i32) * self.cell_width,
            y: (row as i32) * self.cell_height,
            width: self.cell_width,
            height: self.cell_height,
        });
    }
}

impl Widget for Grid {
    fn draw(&self) {
        for widget in &self.widgets {
            widget.draw();
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for widget in &mut self.widgets {
            widget.handle_event(event);
        }
    }

    fn set_position(&mut self, rectangle: Rectangle) {
        // This widget doesn't have a position since it's a container for other widgets
    }

    fn get_position(&self) -> Rectangle {
        // This widget doesn't have a position since it's a container for other widgets
        Rectangle::default()
    }

    fn get_min_width(&self) -> i32 {
        // Return the sum of the minimum widths of all widgets in the grid
        self.widgets.iter().map(|widget| widget.get_min_width()).sum()
    }

    fn get_min_height(&self) -> i32 {
        // Return the sum of the minimum heights of all widgets in the grid
        self.widgets.iter().map(|widget| widget.get_min_height()).sum()
    }
}
