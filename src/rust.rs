use anyhow::Result;
use log::info;

use crate::common::{get_home, log_operation};
use crate::exec::exec;

/// Check if a Rust binary exists in the expected location.
fn check(name: &str) -> bool {
    let Ok(home) = get_home() else { return false };

    let name = if cfg!(target_os = "windows") {
        format!("{name}.exe")
    } else {
        name.to_string()
    };

    let p = if !cfg!(target_os = "android") {
        format!("{home}/.cargo/bin/{name}")
    } else {
        path_clean::clean(format!("{home}../usr/bin/{name}"))
            .to_string_lossy()
            .to_string()
    }
    .replace("\\", "/");

    info!("check: {}", p);
    std::fs::exists(&p).unwrap_or(false)
}

/// Install Rust toolchain via rustup if not already installed.
pub fn rust() -> Result<()> {
    log_operation("rust: installing Rust toolchain");

    if !check("cargo") && !check("rustup") && !cfg!(target_os = "android") {
        info!("Installing Rust toolchain...");
        exec(
            "sh",
            vec![
                "-c",
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly",
            ],
        )?;
    } else {
        info!("Rust is already installed, skipping.");
    }
    Ok(())
}
