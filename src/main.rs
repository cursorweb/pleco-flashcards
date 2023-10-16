use eframe::{
    egui::{self, Color32, FontData, FontDefinitions, Ui},
    epaint::{FontFamily, Vec2},
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

use pleco_study::{color, config::*, scroll_area, text_label, Card, Cont, Reviewer, VLayout};

const RATIO: f32 = 0.75;

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
        let mut layout = VLayout::new();

        layout.ratio(RATIO, |rect| {
            let cont = Cont::new(rect, color(123, 123, 233));
            cont.add_text(ui, &self.card.simp, TITLE_SIZE);
        });

        layout.rest(|rect| {
            let button = egui::Button::new("Flip");
            if ui.put(rect, button).clicked() {
                self.front_side = false;
            }
        });
    }

    fn render_back(&mut self, ui: &mut Ui) {
        let mut layout = VLayout::new();

        layout.ratio(RATIO, |rect| {
            let def = Cont::new(rect, color(6, 209, 20));
            def.add_ui(ui, |ui, rect| {
                let mut layout = VLayout::from(rect);

                layout.ratio(0.1, |rect| {
                    let pinyin = Cont::new(rect, color(234, 123, 231));
                    pinyin.add_text(ui, &self.card.pinyin, TEXT_SIZE);
                });

                layout.ratio(0.3, |rect| {
                    let word = Cont::new(rect, color(255, 255, 255));
                    let label = text_label(
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
                        false,
                    );

                    let label_rect = label.layout_in_ui(ui).2.rect;

                    let label = text_label(
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

                layout.rest(|rect| {
                    let def = Cont::new(rect, color(130, 176, 255));
                    def.add_ui(ui, |ui, rect| {
                        let rect = pleco_study::rect(
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
                                ui.add(text_label(&self.card.def, TEXT_SIZE, true));
                            },
                            [false, true],
                        );
                    });
                });
            });
        });

        layout.rest(|rect| {
            let button = egui::Button::new("Next Card");
            if ui.put(rect, button).clicked() {
                let (next_strength, mut next_card) = self.reviewer.next_card().expect("All done!");

                std::mem::swap(&mut self.card, &mut next_card);
                let old_card = next_card;

                // todo: change self.strength
                self.next_reviewer.studied_card(old_card, self.strength);
                self.strength = next_strength;

                self.front_side = true;
            }
        });
    }

    /*
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
        self.create_scroll_card(
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
            pos2(MARGIN * 2.0, TITLE_SIZE + MARGIN + TEXT_SIZE + MARGIN * 5.0),
            pos2(WIDTH - MARGIN * 2.0, HEIGHT * RATIO - MARGIN * 2.0),
        );

        let frame = egui::Frame::none()
            .fill(Color32::from_rgb(130, 176, 255))
            .rounding(ROUNDING)
            .paint(rect);

        ui.painter().add(frame);

        ui.allocate_ui_at_rect(rect, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(Label::new(RichText::new(&self.card.def).size(TEXT_SIZE)));
            });
        });

        let button = egui::Button::new("Next Card");
        if ui.put(BUTTON_RECT, button).clicked() {
            let (next_strength, mut next_card) = self.reviewer.next_card().expect("All done!");

            std::mem::swap(&mut self.card, &mut next_card);
            let old_card = next_card;

            // todo: change self.strength
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

        ui.put(rect, Label::new(RichText::new(text).size(text_size)));
    }

    fn create_scroll_card(
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

        ui.push_id(32, |ui| {
            ui.allocate_ui_at_rect(rect, |ui| {
                egui::ScrollArea::horizontal()
                    .max_width(rect.width())
                    .show(ui, |ui| {
                        ui.scope(|ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.centered_and_justified(|ui| {
                                ui.add(Label::new(RichText::new(text).size(text_size)));
                            });
                        });
                    });
            });
        });
    }
    */
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
