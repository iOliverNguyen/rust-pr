use crate::cli::Cli;
use crate::config::Config;

pub fn list_prs(args: Cli) {
    let cfg = Config::load();
    println!("{:?}", &cfg);
}
