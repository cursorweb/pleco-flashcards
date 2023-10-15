use eframe::egui::Rect;

use crate::rect;

use super::config::MARGIN;

pub struct Layout {
    // where to draw the *next* item
    x: f32,
    y: f32,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            y: MARGIN,
            x: MARGIN,
        }
    }

    /// add a space
    fn add_y(&mut self) {
        self.y += MARGIN;
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
        let rect = rect(self.y, self.x, width - self.x, height);

        draw(rect);

        self.add_y();
    }
}
