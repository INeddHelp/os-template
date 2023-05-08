use crate::components::Component;

/// Represents the type of an event.
pub enum EventType {
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
}

/// Represents an event that can be handled by a component.
pub struct Event {
    pub event_type: EventType,
    pub target: Box<dyn Component>,
}

/// Trait that allows a component to handle an event.
pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
}
