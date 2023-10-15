use eframe::egui::Rect;

use crate::rect;

use super::config::*;

/// vertical layout
pub struct VLayout {
    // where to draw the *next* item
    x: f32,
    y: f32,
    /// max width
    right: f32,
    /// max height
    bottom: f32,
}

impl VLayout {
    pub fn new() -> Self {
        Self {
            x: MARGIN,
            y: MARGIN,
            right: WIDTH,
            bottom: HEIGHT - MARGIN,
        }
    }

    /// The width will be automatically truncated
    /// using formula: `width - self.x`
    pub fn draw(&mut self, width: f32, height: f32, draw: impl FnOnce(Rect)) {
        let rect = rect(self.x, self.y, width - self.x, self.y + height);

        draw(rect);

        self.y += height + MARGIN;
    }

    /// Calculates height based on ratio of remaining space
    pub fn ratio(&mut self, ratio: f32, draw: impl FnOnce(Rect)) {
        let remaining = (self.bottom - self.y) * ratio;
        self.draw(WIDTH, remaining, draw);
    }

    /// Gives rest of height
    pub fn rest(self, draw: impl FnOnce(Rect)) {
        let rect = rect(self.x, self.y, WIDTH - self.x, self.bottom);
        draw(rect);
    }
}

impl From<Rect> for VLayout {
    fn from(value: Rect) -> Self {
        Self {
            x: value.left() + MARGIN,
            y: value.top() + MARGIN,
            right: value.right(),
            bottom: value.bottom() - MARGIN,
        }
    }
}
