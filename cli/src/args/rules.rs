use clap::Args;
use engine::rules::{Rules, Soft17Rule};

#[derive(Args)]
pub struct RulesArgs {
    #[arg(long, default_value_t = 6)]
    pub num_decks: u8,

    #[arg(long)]
    pub dealer_stands_on_soft_17: bool,

    #[arg(long)]
    pub double_after_split_allowed: bool,

    #[arg(long)]
    pub surrender_allowed: bool,

    #[arg(long, default_value_t = 6)]
    pub max_splits: u8,
}

impl RulesArgs {
    pub fn to_rules(&self) -> Rules {
        Rules {
            num_decks: self.num_decks,
            surrender_allowed: self.surrender_allowed,
            double_after_split_allowed: self.double_after_split_allowed,
            max_splits: self.max_splits,
            dealer_soft_17: if self.dealer_stands_on_soft_17 {
                Soft17Rule::Stand
            } else {
                Soft17Rule::Hit
            },
        }
    }
}
