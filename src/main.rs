use anyhow::Result;

use cli::{Cli, Commands};
use crate::weather::gererate_day_weather;
use crate::markdown::{new_markdown};
use today::generate_name_file_today;
use clap::Parser;

mod cli;
mod markdown;
mod weather;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::New => {
            let response = gererate_day_weather().await;
            let file_name = generate_name_file_today();
            let file_path = format!("{file_name}");
            let _new_file = new_markdown(file_path, response);
            Ok(())
        },
        Commands::Add => {
            Ok(())
        },
        // Commands::Done => {},
        // Commands::NotDone => {},
    }
}
