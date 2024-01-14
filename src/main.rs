use eframe::{
    egui::{self, Color32, FontData, FontDefinitions, Ui},
    epaint::{FontFamily, Vec2},
    App,
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

use pleco_study::{color, config::*, scroll_area, text_label, Card, Cont, Reviewer, VLayout};

const RATIO: f32 = 0.75;
const CARD_LOCATION: &str = "dbg_flash.txt";

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(WIDTH, HEIGHT));
    native_options.resizable = false;

    eframe::run_native(
        "Pleco Card Review",
        native_options,
        Box::new(|cc| Box::new(PlecoApp::new(cc))),
    )
    .unwrap();
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PlecoApp {
    reviewer: Reviewer,
    card: Option<Card>,
    strength: i32,

    /// show the total correct for this session
    #[serde(skip_serializing, skip_deserializing)]
    correct: i32,
    #[serde(skip_serializing, skip_deserializing)]
    total: i32,

    /// show the front side (simp) or back side (trad & pinyin)
    #[serde(skip_serializing, skip_deserializing)]
    card_state: CardState,

    /// next reviewer with the new strengths
    next_reviewer: Reviewer,
}

#[derive(Default)]
enum CardState {
    /// no definitions
    #[default]
    Front,
    /// definitions
    Back,
    /// results
    Results,
}

impl PlecoApp {
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

        if let Some(storage) = cc.storage {
            if let Some(save) = eframe::get_value(storage, eframe::APP_KEY) {
                return save;
            }
        }

        let mut reviewer = Reviewer::load_export(CARD_LOCATION);
        let (strength, card) = reviewer.next_card().expect("Should not be empty");

        Self {
            reviewer,
            card: Some(card),
            strength,
            card_state: CardState::Front,
            next_reviewer: Reviewer::new(),
            correct: 0,
            total: 0,
        }
    }

    fn render_front(&mut self, ui: &mut Ui) {
        let mut layout = VLayout::new();

        let card = self.card.as_ref().unwrap();

        layout.ratio(RATIO, |rect| {
            let cont = Cont::new(rect, color(123, 123, 233));
            cont.add_text(ui, &card.simp, TITLE_SIZE);
        });

        layout.rest(|rect| {
            let button = egui::Button::new("Flip");
            if ui.put(rect, button).clicked() {
                self.card_state = CardState::Back;
            }
        });
    }

    fn render_back(&mut self, ui: &mut Ui) {
        let mut layout = VLayout::new();
        let card = self.card.as_ref().unwrap();

        layout.ratio(RATIO, |rect| {
            let def = Cont::new(rect, color(6, 209, 20));
            def.add_ui(ui, |ui, rect| {
                let mut layout = VLayout::from(rect);

                // pinyin
                layout.ratio(0.15, |rect| {
                    let pinyin = Cont::new(rect, color(234, 123, 231));
                    pinyin.add_text(ui, &card.pinyin, TEXT_SIZE);
                });

                // hanzi
                layout.ratio(0.35, |rect| {
                    let word = Cont::new(rect, color(255, 255, 255));
                    let label = text_label(
                        format!(
                            "{}{}",
                            card.simp,
                            if card.simp != card.trad {
                                format!("[{}]", card.trad)
                            } else {
                                "".into()
                            }
                        ),
                        TITLE_SIZE,
                        false,
                    );

                    let label_rect = label.layout_in_ui(ui).2.rect;

                    // until we can find a way to avoid clone D:
                    let label = text_label(
                        format!(
                            "{}{}",
                            card.simp,
                            if card.simp != card.trad {
                                format!("[{}]", card.trad)
                            } else {
                                "".into()
                            }
                        ),
                        TITLE_SIZE,
                        false,
                    );

                    word.add_ui(ui, |ui, rect| {
                        if label_rect.width() >= rect.width() {
                            scroll_area(
                                ui,
                                "left-word",
                                rect,
                                |ui| {
                                    ui.add(label);
                                },
                                [true, false],
                            );
                        } else {
                            ui.put(rect, label);
                        }
                    });
                });

                // definition
                layout.rest(|rect| {
                    let def = Cont::new(rect, color(130, 176, 255));
                    def.add_ui(ui, |ui, rect| {
                        let rect = pleco_study::rect_c(
                            rect.left() + MARGIN,
                            rect.top() + MARGIN,
                            rect.right(),
                            rect.bottom(),
                        );

                        scroll_area(
                            ui,
                            "defn-scroll",
                            rect,
                            |ui| {
                                ui.add(text_label(&card.def, TEXT_SIZE, true));
                            },
                            [false, true],
                        );
                    });
                });
            });
        });

        layout.ratio_vsplit(0.5, |rect| {
            let button = egui::Button::new("Correct").fill(color(94, 227, 79));
            if ui.put(rect, button).clicked() {
                self.grade_card(1);
            }
        });

        layout.ratio_vsplit(1.0, |rect| {
            let button = egui::Button::new("Wrong").fill(color(232, 77, 77));
            if ui.put(rect, button).clicked() {
                self.grade_card(-1);
            }
        });
    }

    fn results(&mut self, ui: &mut Ui, storage: &mut dyn eframe::Storage) {
        let mut layout = VLayout::new();

        layout.ratio(RATIO, |rect| {
            let cont = Cont::new(rect, color(123, 123, 233));
            cont.add_text(
                ui,
                format!(
                    "{}/{} ({}%)",
                    self.correct,
                    self.total,
                    (self.correct as f32 / self.total as f32) * 100.0
                ),
                TITLE_SIZE,
            );
        });

        layout.rest(|rect| {
            let button = egui::Button::new("Continue");
            if ui.put(rect, button).clicked() {
                // all the cards are guaranteed to be here
                // self.next_reviewer.adjust_scores();
                // println!("{:?}", self.next_reviewer);
                self.save(storage);

                // self.reviewer = self.next_reviewer;
                std::mem::swap(&mut self.reviewer, &mut self.next_reviewer);

                self.next_reviewer = Reviewer::new();
                let (strength, card) = self.reviewer.next_card().expect("should not be empty");

                self.card = Some(card);
                self.strength = strength;

                self.card_state = CardState::Front;
            }
        });
    }

    /// Grades a card
    /// dstrength should be `1`, `-1`
    fn grade_card(&mut self, dstrength: i32) {
        let old_card = if let Some((next_strength, next_card)) = self.reviewer.next_card() {
            let old_card = self.card.take();
            self.card = Some(next_card);

            self.strength = next_strength;
            self.card_state = CardState::Front;

            old_card.unwrap()
        } else {
            self.card_state = CardState::Results;

            self.card.take().unwrap()
        };

        // adjust accuracy
        if dstrength > 0 {
            self.correct += 1;
        }

        self.total += 1;

        self.next_reviewer
            .studied_card(old_card, self.strength + dstrength);
    }
}

impl eframe::App for PlecoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.card_state {
            CardState::Front => self.render_front(ui),
            CardState::Back => self.render_back(ui),
            CardState::Results => self.results(ui, frame.storage_mut().unwrap()),
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
