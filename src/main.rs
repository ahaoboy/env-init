use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

mod exec;

mod install;
use install::{install, install_ei};
mod ssh;
use ssh::ssh;
mod init;
use init::init;
mod base;
mod common;
use common::{is_dry_run, set_dry_run};
mod shell;
use shell::shell;
mod node;
mod reset;
use node::node;
use reset::reset;

mod rust;
use rust::rust;

mod root;
use root::root;

use crate::install::config_git;

#[cfg(windows)]
use crate::install::windows;

/// A cross-platform CLI tool to bootstrap your development environment.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Run in dry-run mode (show what would be done without making changes)
    #[arg(long, global = true)]
    dry_run: bool,

    /// Enable verbose output
    #[arg(long, short, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Install platform-specific development tools
    Install,
    /// Install CLI tools via easy-install
    InstallEi,
    /// Configure SSH server
    Ssh,
    /// Initialize embedded configuration files
    Init,
    /// Reset modified system configuration files from backup
    Reset,
    /// Install Rust toolchain
    Rust,
    /// Configure root auto-login (Linux)
    Root,
    /// Configure fish shell and starship prompt
    Shell,
    /// Install Node.js and pnpm
    Node,
    /// Configure git settings and aliases
    Git,
    /// Configure Windows/MSYS2 settings
    #[cfg(windows)]
    Windows,

    /// Run easy-install with the given arguments
    #[command(trailing_var_arg = true, allow_hyphen_values = true)]
    Ei {
        args: Vec<String>,
    },
}

async fn handle_ei(args: Vec<String>) -> Result<()> {
    let mut v = vec!["ei".to_string()];
    v.extend(args);
    easy_install::run_main(easy_install::Args::parse_from(v)).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter)).init();

    set_dry_run(cli.dry_run);
    if is_dry_run() {
        info!("Running in dry-run mode — no changes will be made");
    }

    match cli.command {
        Cmd::Install => install().await?,
        Cmd::InstallEi => install_ei().await?,
        Cmd::Ssh => ssh()?,
        #[cfg(windows)]
        Cmd::Windows => {
            windows()?;
            info!("restart terminal");
        }
        Cmd::Init => init()?,
        Cmd::Reset => reset()?,
        Cmd::Rust => rust()?,
        Cmd::Root => root()?,
        Cmd::Shell => shell()?,
        Cmd::Git => config_git()?,
        Cmd::Node => node().await?,
        Cmd::Ei { args } => handle_ei(args).await?,
    }

    Ok(())
}
