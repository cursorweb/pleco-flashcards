use druid::widget::{Align, Button, Either, Flex, Label};
use druid::{AppLauncher, Data, Env, Widget, WidgetExt, WindowDesc};
use pleco_study::{Card, Reviewer};

#[derive(Clone, Data)]
struct CardsState {
    #[data(ignore)]
    reviewer: Reviewer,
    card_data: (i32, Card),
    reveal_card: bool,
}

fn main() {
    let main_window = WindowDesc::new(build_card())
        .title("Pleco Card Reviewer")
        .window_size((400.0, 400.0));

    let initial_state = {
        let mut reviewer = Reviewer::load_cards("flash.txt");
        let next_card = reviewer
            .next_card()
            .expect("Cards should not start out empty");

        CardsState {
            reviewer,
            card_data: next_card,
            reveal_card: false,
        }
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_card() -> impl Widget<CardsState> {
    Either::new(
        |a: &CardsState, _| a.reveal_card,
        build_card_back(),
        build_card_front(),
    )
}

fn build_card_front() -> impl Widget<CardsState> {
    let simp_label =
        Label::new(|data: &CardsState, _env: &Env| format!("{}", data.card_data.1.simp))
            .with_text_size(64.0);

    let show_btn = Button::new("Flip").on_click(|_, data: &mut CardsState, _| {
        data.reveal_card = true;
    });

    let layout = Flex::column().with_child(simp_label).with_child(show_btn);

    Align::centered(layout)
}

fn build_card_back() -> impl Widget<CardsState> {
    let text_label = Label::new(|data: &CardsState, _env: &Env| {
        format!("{}[{}]", data.card_data.1.simp, data.card_data.1.trad)
    })
    .with_text_size(64.0)
    .fix_width(200.0);

    let def_label = Label::new(|data: &CardsState, _env: &Env| format!("{}", data.card_data.1.def))
        .with_text_size(32.0)
        .fix_width(200.0);

    let layout = Flex::column()
        .with_child(text_label)
        .with_child(def_label)
        .center();

    Align::centered(layout)
}
