use anyhow::{Context, Result};
use log::info;

use crate::base::write_file;
use crate::common::log_operation;

/// Deploy embedded VS Code settings.json to the user's VS Code config directory.
pub fn vscode() -> Result<()> {
    log_operation("vscode: deploying vscode settings");
    info!("Configuring VS Code settings...");

    let settings = include_str!("../files/settings.json");
    let config_dir = dirs::config_dir().context("Failed to get config directory")?;
    let target = config_dir.join("Code").join("User").join("settings.json");

    info!("Writing VS Code settings to {:?}", target);
    write_file(target, settings)?;
    Ok(())
}
