#![allow(unused_variables)]

mod cli;
mod list;
mod push;
mod utils;
mod config;
mod git;

use crate::list::list_prs;
use crate::push::push_prs;
use crate::utils::print_title;
use clap::{CommandFactory, Parser, Subcommand};
use cli::Cli;
use crate::git::RepoInfo;

fn main() {
    let args = Cli::parse();

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
        }
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
