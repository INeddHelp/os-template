use crate::gui::{Context, Rect};

/// A widget is a GUI element that can be drawn on the screen.
pub trait Widget {
    /// Draws the widget at the specified position.
    fn draw(&self, context: &mut Context, rect: Rect);

    /// Returns the minimum size required to draw the widget.
    fn size_hint(&self, context: &mut Context) -> (f32, f32);

    /// Returns whether the widget contains the specified point.
    fn hit_test(&self, context: &mut Context, x: f32, y: f32) -> bool {
        let (w, h) = self.size_hint(context);
        x >= 0.0 && y >= 0.0 && x < w && y < h
    }
}
