use crate::gui::widget::Widget;

pub struct Stack<T: Widget> {
    widgets: Vec<T>,
}

impl<T: Widget> Stack<T> {
    pub fn new() -> Self {
        Stack {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: T) {
        self.widgets.push(widget);
    }

    pub fn remove_widget(&mut self, widget: &T) {
        if let Some(index) = self.widgets.iter().position(|w| w == widget) {
            self.widgets.remove(index);
        }
    }
}

impl<T: Widget> Widget for Stack<T> {
    fn render(&self) -> String {
        let mut output = String::new();
        for widget in &self.widgets {
            output.push_str(&widget.render());
        }
        output
    }
}
