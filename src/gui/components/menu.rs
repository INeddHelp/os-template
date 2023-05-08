use crate::components::button::Button;

pub struct Menu {
    buttons: Vec<Button>,
}

impl Menu {
    pub fn new(buttons: Vec<Button>) -> Self {
        Self { buttons }
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn draw(&self) {
        for button in &self.buttons {
            button.draw();
        }
    }

    pub fn handle_event(&self, event: &Event) {
        for button in &self.buttons {
            button.handle_event(event);
        }
    }
}
