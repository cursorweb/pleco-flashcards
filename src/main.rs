use eframe::egui::{self, FontDefinitions};

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Pleco Card Review",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct MyEguiApp {
    counter: i32,
    my_string: String,
    my_f32: f32,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let font_defs = FontDefinitions::default();
        cc.egui_ctx.set_fonts(font_defs);
        cc.egui_ctx.set_pixels_per_point(2f32);
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.hyperlink("https://github.com/emilk/egui");

                ui.label(format!("Count: {}", self.counter));

                if ui.button("+1").clicked() {
                    self.counter += 1;
                }

                if ui.button("-1").clicked() {
                    self.counter -= 1;
                }
            });
        });
    }
}
