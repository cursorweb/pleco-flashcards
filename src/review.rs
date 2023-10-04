use crate::Card;
use rand::prelude::*;
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Reviewer {
    /// current set of cards
    pub cards: HashMap<i32, Vec<Card>>,

    /// the lowest score
    pub lowest: i32,

    /// highest score (non-inclusive)
    pub highest: i32,
}

impl Reviewer {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            lowest: i32::MAX,
            highest: i32::MIN,
        }
    }

    pub fn load_cards(file: &str) -> Self {
        let text = fs::read_to_string(file).unwrap();
        let mut card_vec: Vec<Card> = text
            .lines()
            .map(|line| {
                // 猪[豬]\tzhu1\tnoun pig; hog; swine
                let mut cols = line.split("\t").map(String::from);
                let (simp, trad) = parse_trad(cols.next().unwrap());
                let pinyin = cols.next().unwrap();
                let def = cols.next().unwrap();

                Card {
                    simp,
                    trad,
                    pinyin,
                    def,
                }
            })
            .collect();

        card_vec.shuffle(&mut thread_rng());

        let cards = HashMap::from([(0, card_vec)]);

        Self {
            cards,
            lowest: 0,
            highest: 1,
        }
    }

    /// returns `None` if there are no more cards.
    pub fn next_card(&mut self) -> Option<(i32, Card)> {
        let mut cards = self.cards.get_mut(&self.lowest).unwrap();

        while cards.is_empty() {
            self.lowest += 1;
            if self.lowest == self.highest {
                return None;
            }

            cards = self.cards.get_mut(&self.lowest).unwrap();
        }

        Some((
            self.lowest,
            cards.pop().expect("card group should be nonempty"),
        ))
    }

    /// Make sure this applies to the next reviewer!
    pub fn studied_card(&mut self, card: Card, score: i32) {
        if score < self.lowest {
            self.lowest = score;
        }

        if score > self.highest {
            self.highest = score;
        }

        self.cards.entry(score).or_insert(Vec::new()).push(card);
    }
}

fn parse_trad(text: String) -> (String, String) {
    let (simp, text) = text.split_once('[').unwrap();
    let (trad, _) = text.split_once(']').unwrap();

    (simp.to_owned(), trad.to_owned())
}
