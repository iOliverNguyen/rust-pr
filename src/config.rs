use anyhow::Result;
use crate::git::exec_git;

pub const GH_HOSTS_PATH: &str = "~/.config/gh/hosts.yml";

#[derive(Debug)]
pub struct Config {
    pub repo: String, // git
    // pub host: String, // git
    // pub user: String, // gh-cli
    // pub token: String, // gh-cli
    // pub email: String, // git config user.email
}


pub struct Auth {
    pub api_key: String,
}


impl Config {
    pub fn load() -> Result<Config> {

        // detect repository
        let repo = exec_git(vec!["show", "origin"]).unwrap();

        let config = Config { repo };
        Ok(config)
    }
}
