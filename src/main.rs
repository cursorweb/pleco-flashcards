use eframe::{
    egui::{self, pos2, Align, Color32, FontData, FontDefinitions, Layout, Rect, RichText, Ui},
    epaint::{FontFamily, Vec2},
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use pleco_study::{Card, Reviewer};

const TITLE_SIZE: f32 = 64.0;
const ROUNDING: f32 = 5.0;
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(WIDTH, HEIGHT));
    native_options.resizable = false;

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
    fn load_fonts(cc: &eframe::CreationContext<'_>) {
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
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::load_fonts(cc);

        cc.egui_ctx.style_mut(|style| {
            style.visuals.override_text_color = Some(Color32::BLACK);
        });

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
        if ui.button("Flip").clicked() {
            //
        }

        let card_rect = Rect::from_min_max(pos2(10.0, 10.0), pos2(WIDTH - 10.0, HEIGHT - 10.0));

        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(123, 123, 233))
            .rounding(ROUNDING)
            .paint(card_rect);

        ui.painter().add(frame);
        ui.put(
            card_rect,
            egui::Label::new(RichText::new(&self.card.simp).size(TITLE_SIZE)),
        );

        // ui.put(frame);
        // ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        //     if ui.button("Flip").clicked() {
        //         self.front_side = false;
        //     }

        //     egui::Frame::none()
        //         .fill(Color32::from_rgb(123, 123, 233))
        //         .rounding(ROUNDING)
        //         .show(ui, |ui| {
        //             ui.centered_and_justified(|ui| {
        //                 ui.label(RichText::new(&self.card.simp).size(TITLE_SIZE));
        //             });
        //         });
        // });
    }

    fn render_back(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            if ui.button("Next").clicked() {
                let (next_strength, mut next_card) = self.reviewer.next_card().expect("All done!");

                std::mem::swap(&mut self.card, &mut next_card);
                let old_card = next_card;
                self.next_reviewer.studied_card(old_card, self.strength);
                self.strength = next_strength;

                self.front_side = true;
            }

            egui::Frame::none()
                .fill(Color32::from_rgb(6, 209, 20))
                .rounding(ROUNDING)
                .show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
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
                            .size(TITLE_SIZE),
                        );
                        ui.label(&self.card.def);
                    });
                });
            // });
        });
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
