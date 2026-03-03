use anyhow::{Context, Result};
use log::{info, warn};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use crate::common::{is_dry_run, log_operation};

/// Get the embedded files directory next to the executable.
pub fn get_file_dir() -> Result<PathBuf> {
    let exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    let parent = exe_path
        .parent()
        .context("Failed to get parent directory of executable")?;
    let d = parent.join("files");
    if !d.exists() {
        std::fs::create_dir(&d)
            .with_context(|| format!("Failed to create directory {:?}", d))?;
    }
    Ok(d)
}

/// Starship prompt configuration.
pub const STARSHIP: &str = include_str!("../files/starship.toml");

/// Embedded SSH and system configuration files.
pub const FILES: [(&str, &str); 5] = [
    (
        "50-ubuntu.conf",
        include_str!("../files/ssh/50-ubuntu.conf"),
    ),
    (
        "gdm-autologin",
        include_str!("../files/ssh/gdm-autologin"),
    ),
    (
        "gdm-password",
        include_str!("../files/ssh/gdm-password"),
    ),
    ("custom.conf", include_str!("../files/ssh/custom.conf")),
    ("sshd_config", include_str!("../files/ssh/sshd_config")),
];

/// Append text to a file, creating it if it doesn't exist. Creates a backup first.
pub fn append_file<P: AsRef<Path>>(p: P, txt: &str) -> Result<()> {
    let p = p.as_ref();
    log_operation(&format!("append_file: {:?}", p));
    if is_dry_run() {
        info!("[dry-run] would append to {:?}", p);
        return Ok(());
    }
    backup(p);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(p)
        .with_context(|| format!("Failed to open file {:?}", p))?;
    file.write_all(txt.as_bytes())
        .with_context(|| format!("Failed to write to file {:?}", p))?;
    Ok(())
}

/// Write content to a file, creating parent directories as needed. Creates a backup first.
pub fn write_file<P: AsRef<Path>>(p: P, txt: &str) -> Result<()> {
    let path = p.as_ref();
    log_operation(&format!("write_file: {:?}", path));
    if is_dry_run() {
        info!("[dry-run] would write to {:?}", path);
        return Ok(());
    }
    backup(path);
    if let Some(dir) = path.parent()
        && !dir.exists() {
            std::fs::create_dir_all(dir)
                .with_context(|| format!("Failed to create directory {:?}", dir))?;
        }
    std::fs::write(path, txt).with_context(|| format!("Failed to write file {:?}", path))?;
    Ok(())
}

/// Create a backup copy of a file with `.bk` extension.
pub fn backup<P: AsRef<Path>>(p: P) {
    let p = p.as_ref();
    if !p.exists() {
        return;
    }
    let Some(file_name) = p.file_name() else {
        warn!("Failed to get file name of {:?}", p);
        return;
    };
    let name = file_name.to_string_lossy().to_string() + ".bk";
    let Some(parent) = p.parent() else {
        warn!("Failed to get parent directory of {:?}", p);
        return;
    };
    let copy = parent.join(&name);
    if !copy.exists()
        && let Err(e) = std::fs::copy(p, &copy) {
            warn!("Failed to backup {:?} to {:?}: {}", p, copy, e);
        }
}

/// Replace occurrences of a string in a file. Creates a backup first.
pub fn replace_file<P: AsRef<Path>>(p: P, from: &str, to: &str) -> Result<()> {
    let path = p.as_ref();
    log_operation(&format!("replace_file: {:?} ({} -> {})", path, from, to));
    if is_dry_run() {
        info!("[dry-run] would replace in {:?}: '{}' -> '{}'", path, from, to);
        return Ok(());
    }
    backup(path);
    if let Some(dir) = path.parent()
        && !dir.exists() {
            std::fs::create_dir_all(dir)
                .with_context(|| format!("Failed to create directory {:?}", dir))?;
        }
    let s = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file {:?}", path))?;
    std::fs::write(path, s.replace(from, to))
        .with_context(|| format!("Failed to write file {:?}", path))?;
    Ok(())
}
