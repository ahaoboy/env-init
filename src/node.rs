use anyhow::Result;
use log::{info, warn};

use crate::common::{command_exists, log_operation};
use crate::exec::{exec, exec_piped_shell};

/// Install the latest LTS version of Node.js and pnpm using platform-native commands.
pub async fn node() -> Result<()> {
    log_operation("node: installing Node.js and pnpm");

    if command_exists("node") {
        info!("Node.js is already installed, skipping.");
    } else {
        #[cfg(target_os = "linux")]
        install_node_linux()?;

        #[cfg(target_os = "windows")]
        install_node_windows()?;

        #[cfg(target_os = "android")]
        install_node_android()?;

        #[cfg(target_os = "macos")]
        install_node_macos()?;
    }

    install_pnpm()?;
    Ok(())
}

/// Linux: Install Node.js via NodeSource setup script + apt.
#[cfg(target_os = "linux")]
fn install_node_linux() -> Result<()> {
    info!("Installing Node.js via NodeSource (Linux)...");
    if let Err(e) = exec_piped_shell(
        &["-fsSL", "https://deb.nodesource.com/setup_lts.x"],
        &["-"],
    ) {
        warn!("Failed to run NodeSource setup script: {}", e);
        return Ok(());
    }
    exec("apt", vec!["install", "-y", "nodejs"])?;
    Ok(())
}

/// Windows (MSYS2): Install Node.js via pacman.
#[cfg(target_os = "windows")]
fn install_node_windows() -> Result<()> {
    info!("Installing Node.js via pacman (MSYS2)...");
    exec("pacman", vec!["-S", "--noconfirm", "mingw-w64-x86_64-nodejs"])?;
    Ok(())
}

/// Android (Termux): Install Node.js via pkg.
#[cfg(target_os = "android")]
fn install_node_android() -> Result<()> {
    info!("Installing Node.js via pkg (Termux)...");
    exec("pkg", vec!["install", "-y", "nodejs"])?;
    Ok(())
}

/// macOS: Install Node.js via Homebrew.
#[cfg(target_os = "macos")]
fn install_node_macos() -> Result<()> {
    info!("Installing Node.js via Homebrew (macOS)...");
    if !command_exists("brew") {
        anyhow::bail!("Homebrew is not installed. Install it from https://brew.sh");
    }
    exec("brew", vec!["install", "node"])?;
    Ok(())
}

/// Install pnpm via the official install script.
fn install_pnpm() -> Result<()> {
    if command_exists("pnpm") {
        info!("pnpm is already installed, skipping.");
        return Ok(());
    }
    info!("Installing pnpm...");
    if let Err(e) = exec_piped_shell(&["-fsSL", "https://get.pnpm.io/install.sh"], &["-"]) {
        warn!("Failed to install pnpm: {}", e);
    }
    Ok(())
}
