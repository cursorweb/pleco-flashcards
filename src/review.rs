use crate::{Card, CardData};
use std::fs;

#[derive(Debug)]
pub struct Reviewer {
    /// current set of cards
    pub cards: Vec<Card>,

    /// the next set with new strengths
    pub next: Vec<Card>,
}

impl Reviewer {
    pub fn load_cards(file: &str) -> Self {
        let text = fs::read_to_string(file).unwrap();
        let cards = text
            .lines()
            .map(|line| {
                // 猪[豬]\tzhu1\tnoun pig; hog; swine
                let mut cols = line.split("\t").map(String::from);
                // println!("{line}");
                let (simp, trad) = parse_trad(cols.next().unwrap());
                let pinyin = cols.next().unwrap();
                let def = cols.next().unwrap();
                Card::new(CardData {
                    simp,
                    trad,
                    pinyin,
                    def,
                })
            })
            .collect();
        Self {
            cards,
            next: Vec::new(),
        }
    }
}

fn parse_trad(text: String) -> (String, String) {
    let (simp, text) = text.split_once('[').unwrap();
    let (trad, _) = text.split_once(']').unwrap();

    (simp.to_owned(), trad.to_owned())
}
