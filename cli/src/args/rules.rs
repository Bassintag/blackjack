use std::fmt::Display;

use clap::{Args, ValueEnum};
use engine::rules::{BlackjackPayout, Rules, Soft17Rule, SurrenderType};

#[derive(ValueEnum, Clone)]
pub enum BlackjackPayoutArg {
    #[clap(name = "3to2")]
    Ratio3to2,
    #[clap(name = "6to5")]
    Ratio6to5,
}

impl Display for BlackjackPayoutArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BlackjackPayoutArg::Ratio3to2 => "3to2",
                BlackjackPayoutArg::Ratio6to5 => "6to5",
            }
        )
    }
}

impl From<&BlackjackPayoutArg> for BlackjackPayout {
    fn from(value: &BlackjackPayoutArg) -> Self {
        match value {
            BlackjackPayoutArg::Ratio3to2 => Self::Ratio3to2,
            BlackjackPayoutArg::Ratio6to5 => Self::Ratio6to5,
        }
    }
}

#[derive(ValueEnum, Clone)]
pub enum SurrenderArg {
    None,
    Early,
    Late,
}

impl Display for SurrenderArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SurrenderArg::None => "none",
                SurrenderArg::Early => "early",
                SurrenderArg::Late => "late",
            }
        )
    }
}

impl From<&SurrenderArg> for SurrenderType {
    fn from(value: &SurrenderArg) -> Self {
        match value {
            SurrenderArg::None => SurrenderType::None,
            SurrenderArg::Early => SurrenderType::Early,
            SurrenderArg::Late => SurrenderType::Late,
        }
    }
}

#[derive(ValueEnum, Clone)]
pub enum Soft17RuleArg {
    Hit,
    Stand,
}

impl Display for Soft17RuleArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Soft17RuleArg::Hit => "hit",
                Soft17RuleArg::Stand => "stand",
            }
        )
    }
}

impl From<&Soft17RuleArg> for Soft17Rule {
    fn from(value: &Soft17RuleArg) -> Self {
        match value {
            Soft17RuleArg::Hit => Soft17Rule::Hit,
            Soft17RuleArg::Stand => Soft17Rule::Stand,
        }
    }
}

#[derive(Args)]
pub struct RulesArgs {
    #[arg(long, default_value_t = BlackjackPayoutArg::Ratio3to2)]
    pub blackjack_payout: BlackjackPayoutArg,

    #[arg(long, default_value_t = 6)]
    pub num_decks: u8,

    #[arg(long, default_value_t = Soft17RuleArg::Stand)]
    pub dealer_soft_17: Soft17RuleArg,

    #[arg(long)]
    pub double_after_split_allowed: bool,

    #[arg(long, default_value_t = SurrenderArg::None)]
    pub surrender: SurrenderArg,

    #[arg(long, default_value_t = 6)]
    pub max_splits: u8,
}

impl From<&RulesArgs> for Rules {
    fn from(value: &RulesArgs) -> Self {
        Self {
            blackjack_payout: (&value.blackjack_payout).into(),
            num_decks: value.num_decks,
            surrender: (&value.surrender).into(),
            double_after_split_allowed: value.double_after_split_allowed,
            max_splits: value.max_splits,
            dealer_soft_17: (&value.dealer_soft_17).into(),
        }
    }
}
