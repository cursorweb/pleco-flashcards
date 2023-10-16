use eframe::{
    egui::{Frame, Label, Rect, RichText, Ui},
    epaint::Color32,
};

use crate::config::ROUNDING;

pub struct Cont {
    rect: Rect,
    color: Color32,
}

impl Cont {
    pub fn new(rect: Rect, color: Color32) -> Self {
        Self { rect, color }
    }

    fn draw_frame(&self, ui: &mut Ui) {
        let frame = Frame::none()
            .fill(self.color)
            .rounding(ROUNDING)
            .paint(self.rect);

        ui.painter().add(frame);
    }

    pub fn add_text(&self, ui: &mut Ui, text: impl Into<String>, text_size: f32) {
        self.draw_frame(ui);
        ui.put(self.rect, Label::new(RichText::new(text).size(text_size)));
    }

    pub fn add_ui(&self, ui: &mut Ui, func: impl FnOnce(&mut Ui, Rect)) {
        self.draw_frame(ui);
        func(ui, self.rect);
    }
}
