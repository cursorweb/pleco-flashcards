#[derive(Debug)]
pub struct Card {
    /// the word to be tested (simplified)
    pub simp: String,

    /// traditional chinese
    pub trad: String,
    pub pinyin: String,
    pub def: String,
}
