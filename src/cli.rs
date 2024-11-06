use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    // Turn debugging information on
    #[arg(long)]
    pub debug: bool,

    #[arg(short = 'v', long, action = ArgAction::Count)]
    pub verbose: u8,

    #[arg(long = "info")]
    pub info: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "list your PRs and status")]
    Ls,

    #[command(about = "push your commits as PRs")]
    Push,
}
