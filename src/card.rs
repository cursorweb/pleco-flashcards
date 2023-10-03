#[derive(Debug)]
pub struct Card {
    pub data: CardData,
    pub strength: i32,
}

impl Card {
    pub fn new(data: CardData) -> Self {
        Self { data, strength: 0 }
    }
}

#[derive(Debug)]
pub struct CardData {
    /// the word to be tested (simplified)
    pub simp: String,

    /// traditional chinese
    pub trad: String,
    pub pinyin: String,
    pub def: String,
}
