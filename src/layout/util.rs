use eframe::{
    egui::{pos2, Rect},
    epaint::Color32,
};

/// shorthand for
/// `Rect::from_min_max(pos2(x1, y1), pos2(x2, y2))`
pub fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
    Rect::from_min_max(pos2(x1, y1), pos2(x2, y2))
}

pub fn color(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}
