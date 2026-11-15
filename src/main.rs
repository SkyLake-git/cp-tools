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
pub mod validate;

use crate::atcoder::session::{is_logged_in, make_cookie_from};
use crate::cli::dispatch::{
    dispatch_format, dispatch_purge, dispatch_start, dispatch_submit, dispatch_test, dispatch_watch,
};
use crate::config::credentials::{AtCoderCredentials, delete_atcoder, fetch_atcoder, save_atcoder};
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
use std::thread::sleep;
use std::time::Duration;

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

    /// Login to competition site
    Login {
        #[arg(short, long)]
        dotenv: bool,
    },

    /// Logout from competition site
    Logout,

    /// Start task
    Start { task_id: Option<String> },

    /// Format source code
    Format {},

    /// Purge cp-tools files
    Purge {},

    /// Run downloaded test cases
    Test,

    /// Submit source code
    Submit { no_test: Option<bool> },

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
        let unwrapped = workspace?;
        if unwrapped.settings.general.workspace_delay_millis > 0 {
            sleep(Duration::from_millis(
                (&unwrapped.settings.general.workspace_delay_millis).clone(),
            ));
        }
        f(unwrapped)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    dotenvy::dotenv().ok();
    assume_project_dir();

    let pwd = env::current_dir().expect("failed to get current dir");
    match cli.command {
        Commands::Init => {
            Workspace::init(pwd);
            status("Created".bright_green().bold(), "workspace at pwd");
        }
        Commands::Login { dotenv } => {
            if let Ok(_) = fetch_atcoder() {
                status(
                    "Failed".bright_red().bold(),
                    "You already logged in to contest, to login to another user: run `logout`",
                );
                return Ok(());
            }
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
                save_atcoder(AtCoderCredentials::new(session), dotenv)?;
                status(
                    "Succeeded".bright_green().bold(),
                    "saved your session token",
                );
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
            exec_workspace(pwd, |workspace: Workspace| {
                dispatch_format(&workspace, false)?;
                Ok(())
            })?;
        }
        Commands::Purge {} => dispatch_purge()?,
        Commands::Test => {
            exec_workspace(pwd, |workspace: Workspace| {
                dispatch_test(&workspace)?;
                Ok(())
            })?;
        }
        Commands::Submit { no_test } => {
            let no_test_val = no_test.unwrap_or(false);
            exec_workspace(pwd, |workspace: Workspace| {
                dispatch_submit(&workspace, no_test_val)?;
                Ok(())
            })?;
        }
        Commands::Watch => {
            exec_workspace(pwd, |workspace: Workspace| dispatch_watch(&workspace, true))?;
        }
    }

    Ok(())
}
