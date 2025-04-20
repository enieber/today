use anyhow::Result;

use cli::{Cli, Commands};
use tracing::info;
use crate::weather::gererate_day_weather;
use crate::markdown::{new_markdown, update_markdown};
use today::{generate_name_file_today, date_string};
use clap::Parser;

mod cli;
mod markdown;
mod weather;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
          .with_max_level(tracing::Level::INFO)
          .init();
    let args = Cli::parse();
    match args.command {
        Commands::New => {
            let response = gererate_day_weather().await;
            let file_name = generate_name_file_today();
            let today_str = date_string();
            let into = format!("# Today - {}", today_str);
            let file_path = format!("{file_name}");
            let new_file = new_markdown(file_path.clone(), into);
            info!("new_markdown {:?}", &new_file);
            let _update_file = update_markdown(file_path.clone(), "".to_string());
            let update_file = update_markdown(file_path.clone(), response);
            info!("update_markdown {:?}", &update_file);
            let _new_line = update_markdown(file_path.clone(), "".to_string());
            Ok(())
        },
        Commands::Add => {
            Ok(())
        },
        // Commands::Done => {},
        // Commands::NotDone => {},
    }
}
