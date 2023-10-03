use pleco_study::*;

fn main() {
    let x = Reviewer::load_cards("dbg_flash.txt");
    println!("{x:#?}");
}
