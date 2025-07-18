#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Soft17Rule {
    Hit,
    Stand,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rules {
    pub num_decks: u8,
    pub dealer_soft_17: Soft17Rule,
    pub double_after_split_allowed: bool,
    pub surrender_allowed: bool,
    pub max_splits: u8,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            num_decks: 6,
            dealer_soft_17: Soft17Rule::Stand,
            double_after_split_allowed: false,
            surrender_allowed: false,
            max_splits: 3,
        }
    }
}
