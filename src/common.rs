use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result};
use easy_install::InstallConfig;
use log::info;

static DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Check if the application is running in dry-run mode.
pub fn is_dry_run() -> bool {
    DRY_RUN.load(Ordering::Relaxed)
}

/// Set the dry-run mode flag.
pub fn set_dry_run(v: bool) {
    DRY_RUN.store(v, Ordering::Relaxed);
}

/// Record an operation to the persistent log file at `~/.env-init/operations.log`.
pub fn log_operation(op: &str) {
    let Some(home) = dirs::home_dir() else { return };
    let path = home.join(".env-init").join("operations.log");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        use std::io::Write;
        let _ = writeln!(f, "[{ts}] {op}");
    }
}

/// Get the user's home directory path as a string.
pub fn get_home() -> Result<String> {
    let p = std::env::var("HOME").context("HOME environment variable not set")?;
    let p = PathBuf::from_str(&p).context("Failed to parse HOME path")?;
    Ok(p.to_string_lossy().to_string())
}

/// Check if a command is available in the system PATH.
pub fn command_exists(cmd: &str) -> bool {
    std::process::Command::new(if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    })
    .arg(cmd)
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}

/// Get the MSYS2 root directory on Windows.
#[cfg(target_os = "windows")]
pub fn get_msys64_root() -> Result<String> {
    let p =
        std::env::var("MSYSTEM_PREFIX").context("MSYSTEM_PREFIX environment variable not set")?;
    let binding = PathBuf::from_str(&p).context("Failed to parse MSYSTEM_PREFIX path")?;
    let root = binding
        .parent()
        .context("Failed to get parent directory of MSYSTEM_PREFIX")?;
    Ok(root.to_string_lossy().to_string())
}

/// Install a tool using easy-install.
pub async fn ei(url: &str) -> Result<()> {
    if is_dry_run() {
        info!("[dry-run] would install: {}", url);
        log_operation(&format!("[dry-run] ei: {}", url));
        return Ok(());
    }
    log_operation(&format!("ei: {}", url));
    easy_install::ei(url, &InstallConfig::load()).await?;
    Ok(())
}

/// Install a tool using easy-install into the cargo bin directory.
pub async fn ei_cargo(url: &str) -> Result<()> {
    if is_dry_run() {
        info!("[dry-run] would install to cargo bin: {}", url);
        log_operation(&format!("[dry-run] ei_cargo: {}", url));
        return Ok(());
    }
    log_operation(&format!("ei_cargo: {}", url));
    easy_install::ei(
        url,
        &easy_install::InstallConfig {
            dir: Some("~/.cargo/bin".to_string()),
            ..InstallConfig::load()
        },
    )
    .await?;
    Ok(())
}
