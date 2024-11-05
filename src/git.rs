use std::process::Command;
use anyhow::{anyhow, Context, Result};
use os_display::Quotable;
use crate::utils::format_title;

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
    let res = Command::new("git").args(&args).output()
        .with_context(|| {
            let args = args.iter()
                .map(|x| x.maybe_quote().to_string())
                .collect::<Vec<_>>().join(" ");
            format!("failed to exec: git {args}\n")
        })?;

    let stdout = String::from_utf8_lossy(&res.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&res.stderr).trim().to_string();
    if res.status.success() {
        return Ok(stdout);
    }
    let (start, end) = (format_title("GIT ERROR"), format_title(""));
    if stderr.is_empty() {
        Err(anyhow!("\n{start}\n{stdout}\n{end}\n"))
    } else if stdout.is_empty() {
        Err(anyhow!("\n{start}\n{stderr}\n{end}\n"))
    } else {
        Err(anyhow!("\n{start}\n{stdout}\n{stderr}\n{end}\n"))
    }
}

