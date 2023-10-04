use pleco_study::*;

fn main() {
    let mut review = Reviewer::load_cards("dbg_flash.txt");
    while let Some((num, card)) = review.next_card() {
        println!("{num} @ {card:?}");
    }
}
