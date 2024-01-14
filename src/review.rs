use crate::Card;
use rand::prelude::*;
use std::{collections::HashMap, fs};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Reviewer {
    /// current set of cards
    cards: HashMap<i32, Vec<Card>>,

    /// the lowest score
    lowest: i32,

    /// highest score (non-inclusive)
    highest: i32,
}

impl Reviewer {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            lowest: i32::MAX,
            highest: i32::MIN,
        }
    }

    /// Create a new reviewer from an exported file
    pub fn load_export(file: &str) -> Self {
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

        let mut t = thread_rng();
        card_vec.shuffle(&mut t);

        let cards = HashMap::from([(0, card_vec)]);

        Self {
            cards,
            lowest: 0,
            highest: 1,
        }
    }

    /// returns `None` if there are no more cards.
    pub fn next_card(&mut self) -> Option<(i32, Card)> {
        for i in self.lowest..self.highest {
            let cards = self.cards.get_mut(&i);

            if let Some(cards) = cards {
                if let Some(card) = cards.pop() {
                    self.lowest = i;
                    return Some((self.lowest, card));
                }
            }
        }

        None
    }

    /// Make sure this is used on the next reviewer!
    /// Arguments: `(&mut new, ...)`
    pub fn studied_card(&mut self, card: Card, score: i32) {
        if score < self.lowest {
            self.lowest = score;
        }

        if score >= self.highest {
            self.highest = score + 1;
        }

        self.cards.entry(score).or_insert(Vec::new()).push(card);
    }

    /// adjust the scores
    /// for example, if the lowest score is `-1`
    /// every score will be subtracted by `-1`
    pub fn adjust_scores(&mut self) {
        self.highest -= self.lowest;
        let mut new_cards: HashMap<i32, _> = HashMap::new();

        // let cards = self.cards;
        let mut cards = HashMap::new();
        std::mem::swap(&mut self.cards, &mut cards);

        for (key, val) in cards {
            new_cards.insert(key - self.lowest, val);
        }

        self.cards = new_cards;
    }
}

fn parse_trad(text: String) -> (String, String) {
    let (simp, text) = text.split_once('[').unwrap();
    let (trad, _) = text.split_once(']').unwrap();

    (simp.to_owned(), trad.to_owned())
}
