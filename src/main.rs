use anyhow::Result;

use cli::{Cli, Commands};
use tracing::info;
use crate::weather::gererate_day_weather;
use crate::markdown::{
    new_markdown,
    update_markdown,
};
use crate::task::{
    add_task,
    done_task,
    un_done_task,
};
use clap::Parser;

mod cli;
mod markdown;
mod weather;
mod task;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
          .with_max_level(tracing::Level::INFO)
          .init();
    let args = Cli::parse();

    match args.command {
        Commands::New => {
            let response = gererate_day_weather().await;
            let new_file = new_markdown(response);
            info!("new_file {:?}", &new_file);
            let new_line_tasks = update_markdown("## Tasks ".to_string());
            info!("new_line_tasks {:?}", &new_line_tasks);
            let _new_line2 = update_markdown("".to_string());
            Ok(())
        },
        Commands::Add { task } => {
            let _new_line_tasks = add_task(format!("{}", task));
            Ok(())
        },
        Commands::Done { id }=> {
            info!("try done with id: {:?}", &id);
            let _new_line_tasks = done_task(format!("{}", id));
            Ok(())
        },
        Commands::UnDone { id } => {
            info!("try undone with id: {:?}", &id);
            let _new_line_tasks = un_done_task(format!("{}", id));
            Ok(())
        },
    }
}
