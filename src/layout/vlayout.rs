use eframe::egui::Rect;

use crate::rect_wh;

use super::config::*;

/// Vertical layout
pub struct VLayout {
    // position of div
    right: f32,
    bottom: f32,

    x_offset: f32,
    y_offset: f32,
}

impl VLayout {
    pub fn new() -> Self {
        Self {
            right: WIDTH,
            bottom: HEIGHT,

            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    /// Create a rect from the offsets
    /// Automatically downscaled to have a bit of margin around itself
    fn draw(&self, width: f32, height: f32, draw: impl FnOnce(Rect)) {
        let rect = rect_wh(
            self.x_offset + MARGIN / 2.0,
            self.y_offset + MARGIN / 2.0,
            width - MARGIN,
            height - MARGIN,
        );

        // println!(
        //     "width: {width} : drawing from x1={} to x2={}",
        //     self.left + self.x_offset,
        //     self.left + self.x_offset + width - MARGIN
        // );

        draw(rect);
    }

    pub fn dbg_cursor(&self, ui: &mut eframe::egui::Ui) {
        use eframe::egui::Frame;
        let frame = Frame::none().fill(crate::color(255, 0, 0)).paint(rect_wh(
            self.x_offset,
            self.y_offset,
            self.remw(),
            20.0,
        ));
        ui.painter().add(frame);
    }

    /// Calculates height based on ratio of remaining space
    pub fn ratio(&mut self, ratio: f32, draw: impl FnOnce(Rect)) {
        let height = self.remh() * ratio;
        self.draw(self.remw(), height, draw);
        self.y_offset += height;
    }

    /// vertical ratio (as opposed to horizontal stacks)
    /// uses rest of height ...
    pub fn ratio_vsplit(&mut self, ratio: f32, draw: impl FnOnce(Rect)) {
        // println!("before xo: {}/{WIDTH}", self.x_offset);
        let width = self.remw() * ratio;

        self.draw(width, self.remh(), draw);

        self.x_offset += width;
        // println!("after xo: {}/{WIDTH}", self.x_offset);
    }

    /// Gives rest of height
    pub fn rest(self, draw: impl FnOnce(Rect)) {
        self.draw(self.remw(), self.remh(), draw);
    }

    /// remaining width
    fn remw(&self) -> f32 {
        self.right - self.x_offset
    }

    /// remaining height
    fn remh(&self) -> f32 {
        self.bottom - self.y_offset
    }
}

impl From<Rect> for VLayout {
    fn from(value: Rect) -> Self {
        Self {
            right: value.right(),
            bottom: value.bottom(),

            x_offset: value.left(),
            y_offset: value.top(),
        }
    }
}
