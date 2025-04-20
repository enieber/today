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
    let file_path = generate_name_file_today();
    match args.command {
        Commands::New => {
            let response = gererate_day_weather().await;
            let today_str = date_string();
            let into = format!("# Today - {}", today_str);
            let new_file = new_markdown(file_path.clone(), into);
            info!("new_markdown {:?}", &new_file);
            let _update_file = update_markdown(file_path.clone(), "".to_string());
            let update_file = update_markdown(file_path.clone(), response);
            info!("update_markdown {:?}", &update_file);
            let _new_line = update_markdown(file_path.clone(), "".to_string());
            let _new_line_tasks = update_markdown(file_path.clone(), "## Tasks ".to_string());
            let _new_line2 = update_markdown(file_path.clone(), "".to_string());
            Ok(())
        },
        Commands::Add => {
            let _new_line_tasks = update_markdown(file_path.clone(), "- [ ] 1234 - bom dia ".to_string());
            Ok(())
        },
        Commands::Done {id: id }=> {
            info!("try done with id: {:?}", &id);
            Ok(())
        },
        Commands::UnDone {id: id } => {
            info!("try undone with id: {:?}", &id);
            Ok(())
        },
    }
}
