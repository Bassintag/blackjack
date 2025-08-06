use std::fmt::Display;

use clap::{Args, ValueEnum};
use engine::rules::{Rules, Soft17Rule, SurrenderType};

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
            num_decks: value.num_decks,
            surrender: (&value.surrender).into(),
            double_after_split_allowed: value.double_after_split_allowed,
            max_splits: value.max_splits,
            dealer_soft_17: (&value.dealer_soft_17).into(),
        }
    }
}
