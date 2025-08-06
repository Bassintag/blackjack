use clap::Args;
use colored::{ColoredString, Colorize};
use engine::{
    card::{Card, Rank},
    hand::Hand,
    shoe::CountShoe,
    strategy::StrategyGenerator,
};
use regex::Regex;

use crate::{args::rules::RulesArgs, utils::format::action_to_long_colored_string};
use anyhow::{Result, anyhow};

#[derive(Args)]
pub struct HandArgs {
    player: String,

    dealer: String,

    #[command(flatten)]
    rules: RulesArgs,
}

pub fn cmd_hand(args: &HandArgs) -> Result<()> {
    let shoe = CountShoe::new(args.rules.num_decks);
    let mut strategy = StrategyGenerator::new((&args.rules).into(), shoe);

    let dealer_value = parse_value(&args.dealer)?;
    let dealer_upcard = Card::from_rank(Rank::from_value(dealer_value));

    let parts = args.player.split(":");
    let mut player_hand = Hand::new();
    let mut cards = Vec::<Card>::new();
    for part in parts {
        let player_value = parse_value(&part.to_string())?;
        let card = Card::from_rank(Rank::from_value(player_value));
        player_hand.add_card(&card);
        cards.push(card);
    }

    println!("Dealer value: {}", dealer_value);

    let is_pair = cards.len() == 2 && cards[0].rank == cards[1].rank;
    let evs = strategy.eval_round(player_hand, dealer_upcard, is_pair);

    println!("Expected values:");
    println!("  Hit: {}", colorize_ev(evs.hit));
    println!("  Stand: {}", colorize_ev(evs.stand));
    println!("  Double: {}", colorize_ev(evs.double));
    if let Some(split_ev) = evs.split {
        println!("  Split: {}", colorize_ev(split_ev));
    }
    if let Some(surrender_ev) = evs.surrender {
        println!("  Surrender: {}", colorize_ev(surrender_ev));
    }

    let (best_action, ev) = evs.best();

    println!(
        "\nBest action is: {}, (ev = {:.2})",
        action_to_long_colored_string(&best_action),
        ev
    );

    Ok(())
}

fn colorize_ev(value: f64) -> ColoredString {
    let string_value = format!("{:.2}", value);
    if value < 0.0 {
        string_value.red()
    } else {
        string_value.green()
    }
}

fn parse_value(value_string: &String) -> Result<u8> {
    let re = Regex::new("[2-9JQKA]|10").unwrap();
    if !re.is_match(value_string) {
        Err(anyhow!("Invalid card value {}", value_string))
    } else {
        Ok(match value_string.as_str() {
            "A" => 11,
            "J" | "Q" | "K" | "10" => 10,
            _ => value_string.parse()?,
        })
    }
}
