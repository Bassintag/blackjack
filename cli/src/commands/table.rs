use std::fmt;

use clap::{Args, ValueEnum};
use engine::{
    shoe::CountShoe,
    strategy::{StrategyGenerator, StrategyTable},
};
use serde::Serialize;

use crate::{
    args::rules::RulesArgs,
    utils::format::{action_to_colored_string, action_to_string},
};

#[derive(ValueEnum, Clone)]
enum PrintFormat {
    Markdown,
    CSV,
    Json,
}

impl fmt::Display for PrintFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrintFormat::Markdown => "markdown",
                PrintFormat::CSV => "csv",
                PrintFormat::Json => "json",
            }
        )
    }
}

#[derive(Args)]
pub struct TableArgs {
    #[arg(long, short, default_value_t = PrintFormat::Markdown)]
    format: PrintFormat,

    #[arg(long)]
    hard: bool,

    #[arg(long)]
    soft: bool,

    #[arg(long)]
    pair: bool,

    #[command(flatten)]
    rules: RulesArgs,
}

pub fn cmd_table(args: &TableArgs) {
    let all = !args.hard && !args.soft && !args.pair;
    let shoe = CountShoe::new(args.rules.num_decks);
    let mut generator = StrategyGenerator::new(args.rules.to_rules(), shoe);
    let printer: Box<dyn TablePrinter> = match args.format {
        PrintFormat::Markdown => Box::new(MarkdownTablePrinter {}),
        PrintFormat::CSV => Box::new(CSVTablePrinter {}),
        PrintFormat::Json => Box::new(JsonTablePrinter {}),
    };

    if args.hard || all {
        println!("\n\n# HARD \n");
        let table = generator.hard_table();
        printer.print(&table, TableType::Hard);
    }

    if args.soft || all {
        println!("\n\n# SOFT\n");
        let table = generator.soft_table();
        printer.print(&table, TableType::Soft);
    }

    if args.pair || all {
        println!("\n\n# PAIR\n");
        let table = generator.pair_table();
        printer.print(&table, TableType::Pair);
    }
}

trait TablePrinter {
    fn print(&self, table: &StrategyTable, table_type: TableType) -> ();
}

enum TableType {
    Hard,
    Soft,
    Pair,
}

impl TableType {
    fn get_label(&self, value: u8) -> String {
        match self {
            TableType::Hard => value.to_string(),
            TableType::Soft => {
                let hard_part: u8 = value - 11;
                format!("A{hard_part}")
            }
            TableType::Pair => {
                let value = match value {
                    11 => "A".into(),
                    _ => value.to_string(),
                };
                format!("{value},{value}")
            }
        }
    }
}

struct MarkdownTablePrinter;

impl TablePrinter for MarkdownTablePrinter {
    fn print(&self, table: &StrategyTable, table_type: TableType) {
        print!("| Hand \\ Dealer |");
        for dealer_value in table.dealer_value_range() {
            let header = match dealer_value {
                11 => "A".to_string(),
                _ => dealer_value.to_string(),
            };
            print!("  {header:<2} |");
        }
        println!();

        print!("|---------------|");
        for _ in 2..=11 {
            print!(":---:|");
        }
        println!();

        for player_value in table.player_value_range() {
            let label = table_type.get_label(player_value);
            print!("| {label:<13} |");
            for dealer_value in table.dealer_value_range() {
                let value = table.get(player_value, dealer_value);
                let symbol = action_to_colored_string(&value.action);
                print!(" {symbol:>2}  |");
            }
            println!();
        }
    }
}

struct CSVTablePrinter;

impl TablePrinter for CSVTablePrinter {
    fn print(&self, table: &StrategyTable, table_type: TableType) -> () {
        for dealer_value in table.dealer_value_range() {
            let header = match dealer_value {
                11 => "A".to_string(),
                _ => dealer_value.to_string(),
            };
            print!(",{}", header);
        }
        println!();

        for player_value in table.player_value_range() {
            let label = table_type.get_label(player_value);
            print!("{label}");
            for dealer_value in table.dealer_value_range() {
                let value = table.get(player_value, dealer_value);
                let symbol = action_to_string(&value.action);
                print!(",{}", symbol);
            }
            println!();
        }
    }
}

#[derive(Serialize)]
struct PlayerRow {
    player: String,
    entries: Vec<DealerEntry>,
}

#[derive(Serialize)]
struct DealerEntry {
    dealer: String,
    action: String,
    ev: f64,
}

struct JsonTablePrinter;

impl TablePrinter for JsonTablePrinter {
    fn print(&self, table: &StrategyTable, table_type: TableType) -> () {
        let mut rows = Vec::new();

        for player_value in table.player_value_range() {
            let mut row = PlayerRow {
                player: table_type.get_label(player_value),
                entries: Vec::new(),
            };

            for dealer_value in table.dealer_value_range() {
                let entry = table.get(player_value, dealer_value);
                let dealer_label = match dealer_value {
                    11 => "A".to_string(),
                    _ => dealer_value.to_string(),
                };
                row.entries.push(DealerEntry {
                    dealer: dealer_label,
                    action: action_to_string(&entry.action),
                    ev: entry.ev,
                });
            }

            rows.push(row);
        }

        let json = serde_json::to_string_pretty(&rows).unwrap();
        println!("{}", json);
    }
}
