use crate::hand::Hand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlackjackPayout {
    Ratio3to2,
    Ratio6to5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Soft17Rule {
    Hit,
    Stand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurrenderType {
    None,
    Early,
    Late,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rules {
    pub blackjack_payout: BlackjackPayout,
    pub num_decks: u8,
    pub dealer_soft_17: Soft17Rule,
    pub double_after_split_allowed: bool,
    pub surrender: SurrenderType,
    pub max_splits: u8,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            blackjack_payout: BlackjackPayout::Ratio3to2,
            num_decks: 6,
            dealer_soft_17: Soft17Rule::Stand,
            double_after_split_allowed: false,
            surrender: SurrenderType::None,
            max_splits: 3,
        }
    }
}

impl Rules {
    pub fn dealer_must_stand(&self, hand: &Hand) -> bool {
        let value = hand.value();
        if value > 17 {
            true
        } else if value == 17 {
            if hand.is_soft() {
                match self.dealer_soft_17 {
                    Soft17Rule::Stand => true,
                    Soft17Rule::Hit => false,
                }
            } else {
                true
            }
        } else {
            false
        }
    }
}
