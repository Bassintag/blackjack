use clap::{Parser, Subcommand, command};

use crate::commands::table::{TableArgs, cmd_table};

mod args;
mod commands;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Table {
        #[command(flatten)]
        table_args: TableArgs,
    },
}

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Table { table_args } => {
            cmd_table(table_args);
        }
    }
}
