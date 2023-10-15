use eframe::{
    egui::{Frame, Label, Rect, RichText, Ui},
    epaint::Color32,
};

use crate::rect;

use super::config::ROUNDING;

pub struct Cont {
    rect: Rect,
    color: Color32,
}

impl Cont {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color32) -> Self {
        Self {
            rect: rect(x, y, x + width, y + height),
            color,
        }
    }

    fn draw_frame(&self, ui: &mut Ui) {
        let frame = Frame::none()
            .fill(self.color)
            .rounding(ROUNDING)
            .paint(self.rect);

        ui.painter().add(frame);
    }

    pub fn add_text(&self, ui: &mut Ui, text: &str, text_size: f32) {
        self.draw_frame(ui);
        ui.put(self.rect, Label::new(RichText::new(text).size(text_size)));
    }

    pub fn add_ui(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui, Rect)) {
        self.draw_frame(ui);
        func(ui, self.rect);
    }
}
