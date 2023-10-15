use eframe::egui::Rect;

use crate::rect;

use super::config::*;

/// vertical layout
pub struct VLayout {
    // where to draw the *next* item
    x: f32,
    y: f32,
    /// max width
    width: f32,
    /// max height
    height: f32,
}

impl VLayout {
    pub fn new() -> Self {
        Self {
            x: MARGIN,
            y: MARGIN,
            width: WIDTH,
            height: HEIGHT - MARGIN,
        }
    }

    /// increase x margin
    pub fn add_x(&mut self) {
        self.x += MARGIN;
    }

    /// decrease x margin
    pub fn sub_x(&mut self) {
        self.x -= MARGIN;
    }

    /// the width will be automatically truncated
    /// using formula: `width - self.x`
    pub fn draw(&mut self, width: f32, height: f32, draw: impl FnOnce(Rect)) {
        let rect = rect(self.x, self.y, width - self.x, self.y + height);

        draw(rect);

        self.y += height + MARGIN;
    }

    /// calculates height based on ratio of remaining space
    pub fn ratio(&mut self, ratio: f32, draw: impl FnOnce(Rect)) {
        let remaining = (self.height - self.y) * ratio;
        self.draw(self.width, remaining, draw);
    }

    /// gives rest of height
    pub fn rest(self, draw: impl FnOnce(Rect)) {
        let rect = rect(self.x, self.y, self.width - self.x, self.height);
        draw(rect);
    }
}

impl From<Rect> for VLayout {
    fn from(value: Rect) -> Self {
        Self {
            x: value.left() + MARGIN,
            y: value.right() + MARGIN,
            width: value.width(),
            height: value.height() - MARGIN,
        }
    }
}
