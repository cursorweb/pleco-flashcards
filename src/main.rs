use eframe::{
    egui::{self, FontData, FontDefinitions, RichText, Ui},
    epaint::{FontFamily, Vec2},
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use pleco_study::{Card, Reviewer};

const TITLE_SIZE: f32 = 32.0;
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(WIDTH, HEIGHT));

    eframe::run_native(
        "Pleco Card Review",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    reviewer: Reviewer,
    card: Card,
    strength: i32,

    /// show the front side (simp) or back side (trad & pinyin)
    front_side: bool,

    /// next reviewer with the new strengths
    next_reviewer: Reviewer,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let source = SystemSource::new();
        let font =
            source.select_best_match(&[FamilyName::Title("kaiti".into())], &Properties::default());

        let handle = font.unwrap().load().unwrap();
        let handle = handle.handle().unwrap();
        let Handle::Memory { bytes, .. } = handle else {
            panic!("uh oh we got a path")
        };

        let bytes = (*bytes).clone();

        let mut fonts = FontDefinitions::default();

        fonts
            .font_data
            .insert("chinese".into(), FontData::from_owned(bytes));

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .push("chinese".into());

        cc.egui_ctx.set_fonts(fonts);

        let mut reviewer = Reviewer::load_cards("flash.txt");
        let (strength, card) = reviewer.next_card().expect("Should not be empty");

        Self {
            reviewer,
            card,
            strength,
            front_side: true,
            next_reviewer: Reviewer::new(),
        }
    }

    fn render_front(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            if ui.button("Flip").clicked() {
                self.front_side = false;
            }

            ui.centered_and_justified(|ui| {
                ui.label(RichText::new(&self.card.simp).size(64.0));
            });
        });
    }

    fn render_back(&mut self, ui: &mut Ui) {
        ui.label(&self.card.pinyin);
        ui.label(
            RichText::new(format!(
                "{}{}",
                self.card.simp,
                if self.card.simp != self.card.trad {
                    format!("[{}]", self.card.trad)
                } else {
                    "".into()
                }
            ))
            .size(64.0),
        );
        ui.label(&self.card.def);

        if ui.button("Next").clicked() {
            let (next_strength, mut next_card) = self.reviewer.next_card().expect("All done!");

            std::mem::swap(&mut self.card, &mut next_card);
            let old_card = next_card;
            self.next_reviewer.studied_card(old_card, self.strength);
            self.strength = next_strength;

            self.front_side = true;
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.front_side {
                self.render_front(ui);
            } else {
                self.render_back(ui);
            }
        });
    }
}
