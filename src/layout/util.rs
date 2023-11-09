use eframe::{
    egui::{pos2, Label, Rect, RichText, ScrollArea, Ui},
    epaint::Color32,
};

/// Rect Corners, shorthand for
/// `Rect::from_min_max(pos2(x1, y1), pos2(x2, y2))`
pub fn rect_c(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
    Rect::from_min_max(pos2(x1, y1), pos2(x2, y2))
}

/// Rect Width-Height, shorthand for
/// `Rect::from_min_max(pos2(x, y), pos2(x + w, y + h))`
pub fn rect_wh(x: f32, y: f32, w: f32, h: f32) -> Rect {
    rect_c(x, y, x + w, y + h)
}

pub fn color(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

pub fn text_label(text: impl Into<String>, size: f32, wrap: bool) -> Label {
    Label::new(RichText::new(text).size(size)).wrap(wrap)
}

/// scroll opt: [horiz, vert]
pub fn scroll_area(
    ui: &mut Ui,
    id: &str,
    rect: Rect,
    show: impl FnOnce(&mut Ui),
    scroll_opt: [bool; 2],
) {
    ui.push_id(id, |ui| {
        ui.allocate_ui_at_rect(rect, |ui| {
            ScrollArea::new(scroll_opt).show(ui, show);
        });
    });
}
