use pleco_study::*;

fn main() {
    let x = parse_flash(read_flash());
    println!("{x:#?}");
}
