use eframe::{
    egui::{self, pos2, Color32, FontData, FontDefinitions, Label, Rect, RichText, Ui},
    epaint::{FontFamily, Vec2},
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use pleco_study::{Card, Reviewer};

const TITLE_SIZE: f32 = 64.0;
const TEXT_SIZE: f32 = 16.0;

const ROUNDING: f32 = 5.0;

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = WIDTH;

const MARGIN: f32 = 10.0;

const RATIO: f32 = 0.8;

const CARD_RECT: Rect = Rect::from_min_max(
    pos2(MARGIN, MARGIN),
    pos2(WIDTH - MARGIN, HEIGHT * RATIO - MARGIN),
);

const BUTTON_RECT: Rect = Rect::from_min_max(
    pos2(MARGIN, HEIGHT * RATIO),
    pos2(WIDTH - MARGIN, HEIGHT - MARGIN),
);

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

        let mut reviewer = Reviewer::load_cards("one_flash.txt");
        let (strength, card) = reviewer.next_card().expect("Should not be empty");

        Self {
            reviewer,
            card,
            strength,
            front_side: false,
            next_reviewer: Reviewer::new(),
        }
    }

    fn render_front(&mut self, ui: &mut Ui) {
        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(123, 123, 233))
            .rounding(ROUNDING)
            .paint(CARD_RECT);

        ui.painter().add(frame);
        ui.put(
            CARD_RECT,
            Label::new(RichText::new(&self.card.simp).size(TITLE_SIZE)),
        );

        let button = egui::Button::new("Flip");
        if ui.put(BUTTON_RECT, button).clicked() {
            self.front_side = false;
        }
    }

    fn render_back(&mut self, ui: &mut Ui) {
        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(6, 209, 20))
            .rounding(ROUNDING)
            .paint(CARD_RECT);

        ui.painter().add(frame);

        // pinyin
        self.create_cont_card(
            ui,
            Rect::from_min_max(
                pos2(MARGIN * 2.0, MARGIN * 2.0),
                pos2(WIDTH - MARGIN * 2.0, TEXT_SIZE + MARGIN * 3.0),
            ),
            Color32::from_rgb(234, 123, 231),
            &self.card.pinyin,
            TEXT_SIZE,
        );

        // simp[trad]
        self.create_cont_card(
            ui,
            Rect::from_min_max(
                pos2(MARGIN * 2.0, TEXT_SIZE + MARGIN * 4.0),
                pos2(
                    WIDTH - MARGIN * 2.0,
                    TITLE_SIZE + MARGIN + TEXT_SIZE + MARGIN * 4.0,
                ),
            ),
            Color32::from_rgb(255, 255, 255),
            format!(
                "{}{}",
                self.card.simp,
                if self.card.simp != self.card.trad {
                    format!("[{}]", self.card.trad)
                } else {
                    "".into()
                }
            ),
            TITLE_SIZE,
        );

        let rect = Rect::from_min_max(
            pos2(MARGIN * 2.0, TEXT_SIZE + MARGIN * 8.0),
            pos2(
                WIDTH - MARGIN * 2.0,
                TITLE_SIZE + MARGIN + TEXT_SIZE + MARGIN * 12.0,
            ),
        );
        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(49, 39, 12))
            .rounding(ROUNDING)
            .paint(rect);

        ui.painter().add(frame);

        let rect = Rect::from_min_max(
            pos2(MARGIN * 2.0, TEXT_SIZE + MARGIN * 8.0),
            pos2(
                WIDTH - MARGIN * 2.0,
                TITLE_SIZE + MARGIN + TEXT_SIZE + MARGIN * 12.0,
            ),
        );
        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(49, 39, 12))
            .rounding(ROUNDING)
            .paint(rect);

        ui.painter().add(frame);

        egui::ScrollArea::vertical()
            .max_height(100f32)
            .show(ui, |ui| {
                ui.label("something something".repeat(99));
            });

        let button = egui::Button::new("Next Card");
        if ui.put(BUTTON_RECT, button).clicked() {
            let (next_strength, mut next_card) = self.reviewer.next_card().expect("All done!");

            std::mem::swap(&mut self.card, &mut next_card);
            let old_card = next_card;
            self.next_reviewer.studied_card(old_card, self.strength);
            self.strength = next_strength;

            self.front_side = true;
        }
    }

    /// create a container with text
    fn create_cont_card(
        &self,
        ui: &mut Ui,
        rect: Rect,
        color: Color32,
        text: impl Into<String>,
        text_size: f32,
    ) {
        let frame = egui::Frame::none()
            .fill(color)
            .rounding(ROUNDING)
            .paint(rect);

        ui.painter().add(frame);

        ui.put(
            rect,
            Label::new(
                RichText::new(text).size(text_size),
                // .background_color(Color32::RED),
            ),
        );
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
