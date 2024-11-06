use crate::git::exec_git;
use anyhow::Result;

pub const GH_HOSTS_PATH: &str = "~/.config/gh/hosts.yml";

#[derive(Debug)]
pub struct Config {
    pub repo: String, // [git] "username/repo"
    pub host: String, // [git] "github.com"
    pub repo_url: String, // [git] "git@github.com:username/repo.git"
                      // pub user: String, // [gh-cli]
                      // pub token: String, // [gh-cli]
                      // pub email: String, // git config user.email
}

pub struct Auth {
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        exec_git(vec!["remote", "-v"])?;

        // detect repository
        let repo = exec_git(vec!["show", "origin"])?;
        let host = "".to_string();
        let repo_url = "".to_string();

        let config = Config {
            repo,
            host,
            repo_url,
        };
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_load_config() {}
}
