use std::rc::Rc;
use std::cell::RefCell;

use crate::event::{Event, EventHandler};

pub struct MenuItem {
    label: String,
    on_select: Box<dyn Fn()>,
}

impl MenuItem {
    pub fn new(label: String, on_select: Box<dyn Fn()>) -> Self {
        MenuItem { label, on_select }
    }
}

pub struct Menu {
    items: Vec<Rc<RefCell<MenuItem>>>,
    on_select: Option<Box<dyn Fn()>>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { items: Vec::new(), on_select: None }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(Rc::new(RefCell::new(item)));
    }

    pub fn set_on_select(&mut self, on_select: Box<dyn Fn()>) {
        self.on_select = Some(on_select);
    }
}

impl EventHandler for Menu {
    fn handle_event(&mut self, event: &Event) {
        if let Event::MouseClick(x, y) = event {
            // TODO: handle click event
        }
    }
}
