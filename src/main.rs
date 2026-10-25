pub mod atcoder;
pub mod cli;
pub mod config;
pub mod constants;
pub mod cookie;
pub mod def;
pub mod format;
pub mod modern_log;
pub mod tester;
pub mod usecase;
pub mod utils;

use crate::cli::dispatch::{dispatch_start, dispatch_test};
use crate::modern_log::{recv_input, status, write_status};
use crate::usecase::workspace::Workspace;
use anyhow::Result;
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cp-tools", version, about = "Competitive-Programming Tools")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize cp-tools environment
    Init,

    /// Start task
    Start { task_id: Option<String> },

    /// Run downloaded test cases
    Test,

    /// Submit source code
    Submit { source: String },

    /// Watch submission status
    Watch,
}

fn exec_workspace(
    pwd: PathBuf,
    mut f: impl FnMut(Workspace) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let workspace = Workspace::load(pwd);
    if workspace.is_err() {
        status(
            "Failed".bright_red().bold(),
            "to load workspace, try run `init`",
        );
        Ok(())
    } else {
        f(workspace?)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let pwd = env::current_dir().expect("failed to get current dir");
    match cli.command {
        Commands::Init => {
            Workspace::init(pwd);
            status("Created".bright_green().bold(), "workspace at pwd");
        }
        Commands::Start { task_id } => {
            let task_id_content = match task_id {
                Some(s) => s,
                None => {
                    write_status("Task ID".cyan().bold(), ": ");
                    recv_input()?
                }
            };
            exec_workspace(pwd, |workspace: Workspace| {
                dispatch_start(workspace, task_id_content.as_str())
            })?;
        }
        Commands::Test => {
            exec_workspace(pwd, |workspace: Workspace| dispatch_test(workspace))?;
        }
        Commands::Submit { source } => {}
        Commands::Watch => {}
    }

    Ok(())
}
