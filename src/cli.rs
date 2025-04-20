use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "today")]
#[command(about = "Todo list with markdown format about task of today", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// New day genereate file
    New,
    /// Add task today
    Add,
    /// Done task passing id
    Done {
        /// id of task
        id: String,
    },
    UnDone {
        /// id of task
        id: String,
    }
}
