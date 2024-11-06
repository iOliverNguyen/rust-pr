#![allow(unused_variables)]

mod cli;
mod config;
mod git;
mod list;
mod push;
mod utils;

use crate::git::RepoInfo;
use crate::list::list_prs;
use crate::push::push_prs;
use crate::utils::print_title;
use clap::{CommandFactory, Parser};
use cli::Cli;
use tracing::Level;

fn main() {
    let args = Cli::parse();
    {
        use tracing_subscriber::{filter::Directive, EnvFilter};

        let env_filter = EnvFilter::builder()
            .with_default_directive(Directive::from(Level::INFO))
            .with_env_var("LOG_LEVEL")
            .from_env_lossy();
        let subscriber = tracing_subscriber::fmt()
            .with_writer(std::io::stderr) // 👈 use stderr
            .without_time()
            .with_file(true)
            .with_line_number(true);
        if args.debug {
            subscriber.with_max_level(Level::DEBUG).init();
        } else {
            subscriber.with_env_filter(env_filter).init();
        }
    }

    match &args.command {
        None => {
            if args.info {
                show_info(args)
            } else {
                show_help(args)
            }
        }
        Some(cmd) => match cmd {
            cli::Command::Ls => list_prs(args),
            cli::Command::Push => push_prs(args),
        },
    }
}

fn show_help(args: Cli) {
    print_title("HELP");
    Cli::command().print_help().unwrap()
}

fn show_info(args: Cli) {
    let info = RepoInfo::load();

    print_title("INFO");
    println!("{info:?}")
}
