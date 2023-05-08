use crate::components::Component;
use crate::events::Event;
use crate::graphics::Canvas;

struct Button {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    pub fn new(x: u32, y: u32, width: u32, height: u32, label: String) -> Button {
        Button {
            x,
            y,
            width,
            height,
            label,
        }
    }
}

impl Component for Button {
    fn render(&self, canvas: &mut Canvas) {
        canvas.draw_rect(self.x, self.y, self.width, self.height);
        canvas.draw_text(self.x + self.width / 2, self.y + self.height / 2, &self.label);
    }

    fn handle_event(&mut self, event: Event) {
        // TODO: Handle events such as clicks, mouse-over, etc.
    }
}
