use eframe::egui::Rect;

use crate::rect;

use super::config::*;

/// Vertical layout
pub struct VLayout {
    // position of div
    top: f32,
    left: f32,
    right: f32,
    bottom: f32,

    // margin aka offset
    x_margin: f32,
    y_margin: f32,
}

impl VLayout {
    pub fn new() -> Self {
        Self {
            top: 0.0,
            left: 0.0,
            right: WIDTH,
            bottom: HEIGHT,

            x_margin: MARGIN,
            y_margin: MARGIN,
        }
    }

    /// The width will be automatically truncated
    /// using formula: `width - self.x`
    pub fn draw(&mut self, width: f32, height: f32, draw: impl FnOnce(Rect)) {
        let rect = rect(
            self.left + self.x_margin,
            self.top + self.y_margin,
            width - self.x_margin,
            self.top + self.y_margin + height,
        );

        draw(rect);

        self.y_margin += height + MARGIN;
    }

    /// Calculates height based on ratio of remaining space
    pub fn ratio(&mut self, ratio: f32, draw: impl FnOnce(Rect)) {
        let remaining = (self.bottom - self.y_margin) * ratio;
        self.draw(self.right, remaining, draw);
    }

    /// Gives rest of height
    pub fn rest(self, draw: impl FnOnce(Rect)) {
        let rect = rect(
            self.left + self.x_margin,
            self.top + self.y_margin,
            self.right - self.x_margin,
            self.bottom - MARGIN,
        );

        draw(rect);
    }
}

impl From<Rect> for VLayout {
    fn from(value: Rect) -> Self {
        Self {
            left: value.left(),
            top: value.top(),
            right: value.right(),
            bottom: value.bottom(),

            x_margin: MARGIN,
            y_margin: MARGIN,
        }
    }
}
