use crate::utils::{format_title, log_prefix};
use anyhow::{anyhow, Context, Result};
use os_display::Quotable;
use owo_colors::{Color, OwoColorize};
use std::process::Command;
use tracing::{debug, Level};
use tracing_subscriber::fmt::format;

#[derive(Debug)]
pub struct RepoInfo {
    pub remote: String,
}

impl RepoInfo {
    pub fn load() -> RepoInfo {
        let remote: String = {
            let out = exec_git(vec!["remote"]).unwrap();
            out.split("\n").next().unwrap_or("").to_string()
        };

        RepoInfo { remote }
    }
}

pub fn exec_git(args: Vec<&str>) -> Result<String> {
    let format_args = || {
        args.iter()
            .map(|x| x.maybe_quote().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let res = Command::new("git").args(&args).output().with_context(|| {
        // 👈 from anyhow
        format!("failed to exec: git {}\n", format_args())
    })?;

    let stdout = String::from_utf8_lossy(&res.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&res.stderr).trim().to_string();

    debug!(
        "git {} {}",
        format_args(),
        if res.status.success() {
            format!("{}", "✔".green())
        } else {
            format!("{} {}", "✗".red(), res.status)
        }
    );
    if tracing::enabled!(Level::DEBUG) {
        if !stdout.is_empty() {
            log_prefix(" GIT  ", &stdout);
        }
        if !stderr.is_empty() {
            log_prefix(" GIT! ", &stderr);
        }
    }

    if res.status.success() {
        return Ok(stdout);
    }

    if stderr.is_empty() {
        Err(anyhow!(stdout))
    } else if stdout.is_empty() {
        Err(anyhow!(stderr))
    } else {
        Err(anyhow!("{stdout}\n{stderr}"))
    }
}
