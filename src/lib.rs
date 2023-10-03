use std::fs::read_to_string;

pub fn read_flash() -> String {
    let x = read_to_string("flash.txt").unwrap();
    x
}

#[derive(Debug)]
pub struct Card {
    /// the word to be tested (simplified)
    pub simp: String,
    /// traditional chinese
    pub trad: String,
    pub pinyin: String,
    pub defn: String,
}

pub fn parse_flash(text: String) -> Vec<Card> {
    text.lines().map(|line| {
        // 猪[豬]\tzhu1\tnoun pig; hog; swine
        let mut cols = line.split("\t").map(String::from);
        // println!("{line}");
        let (simp, trad) = parse_trad(cols.next().unwrap());
        let pinyin = cols.next().unwrap();
        let defn = cols.next().unwrap();
        Card { simp, trad, pinyin, defn }
    }).collect()
}

pub fn parse_trad(text: String) -> (String, String) {
    let mut chars = text.chars();
    let simp = chars.by_ref().take_while(|&c| c != '[').collect();
    let trad = chars.take_while(|&c| c != ']').collect();

    (simp, trad)
}