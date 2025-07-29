use clap::{Parser, Subcommand, command};

use crate::commands::{
    hand::{HandArgs, cmd_hand},
    table::{TableArgs, cmd_table},
};

mod args;
mod commands;
mod utils;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hand(HandArgs),
    Table(TableArgs),
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hand(args) => {
            if let Err(error) = cmd_hand(args) {
                eprintln!("Error: {error}")
            }
        }
        Commands::Table(args) => {
            cmd_table(args);
        }
    }
}
