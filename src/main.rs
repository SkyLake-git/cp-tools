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

use crate::atcoder::scrape::{is_logged_in, make_cookie_from};
use crate::cli::dispatch::{dispatch_format, dispatch_purge, dispatch_start, dispatch_test};
use crate::config::credentials::{AtCoderCredentials, delete_atcoder, save_atcoder};
use crate::constants::assume_project_dir;
use crate::modern_log::{format_status, recv_input, status, write_status};
use crate::usecase::workspace::Workspace;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Password;
use owo_colors::OwoColorize;
use secrecy::SecretString;
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

    Login,
    Logout,

    /// Start task
    Start {
        task_id: Option<String>,
    },

    Format {},

    /// Purge cp-tools files
    Purge {},

    /// Run downloaded test cases
    Test,

    /// Submit source code
    Submit {
        source: String,
    },

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
    assume_project_dir();

    let pwd = env::current_dir().expect("failed to get current dir");
    match cli.command {
        Commands::Init => {
            Workspace::init(pwd);
            status("Created".bright_green().bold(), "workspace at pwd");
        }
        Commands::Login => {
            let session: SecretString = SecretString::from(
                Password::new()
                    .with_prompt(format_status(
                        "Paste".bright_cyan().bold(),
                        "AtCoder REVEL_SESSION here to login",
                    ))
                    .interact()?,
            );

            status("Checking".bright_green().bold(), "session");

            if let Ok(logged_in) = is_logged_in(make_cookie_from(&session))
                && logged_in
            {
                status(
                    "Succeeded".bright_green().bold(),
                    "saved your session token",
                );
                save_atcoder(AtCoderCredentials::new(session))?;
            } else {
                status(
                    "Failed".bright_red().bold(),
                    "invalid session, or check your network",
                )
            }
        }
        Commands::Logout => {
            if let Ok(_) = delete_atcoder() {
                status("Succeeded".bright_green().bold(), "logged out")
            } else {
                status(
                    "Failed".bright_red().bold(),
                    "to logout, or already logged out",
                )
            }
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
        Commands::Format {} => {
            exec_workspace(pwd, |workspace: Workspace| dispatch_format(workspace))?;
        }
        Commands::Purge {} => dispatch_purge()?,
        Commands::Test => {
            exec_workspace(pwd, |workspace: Workspace| dispatch_test(workspace))?;
        }
        Commands::Submit { source } => {}
        Commands::Watch => {}
    }

    Ok(())
}
