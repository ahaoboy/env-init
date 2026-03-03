use anyhow::{Context, bail, Result};
use log::info;
use std::{ffi::OsStr, process::Stdio};

use crate::common::{is_dry_run, log_operation};

/// Execute a command with the given arguments, inheriting stdio.
pub fn exec<S: AsRef<OsStr>>(cmd: S, args: Vec<&str>) -> Result<()> {
    let cmd = cmd.as_ref();
    let cmd_str = format!("{:?} {}", cmd, args.join(" "));

    if is_dry_run() {
        info!("[dry-run] would exec: {}", cmd_str);
        log_operation(&format!("[dry-run] exec: {}", cmd_str));
        return Ok(());
    }

    info!("exec: {}", cmd_str);
    log_operation(&format!("exec: {}", cmd_str));

    let output = std::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Command {} failed with exit code: {:?}",
            cmd_str,
            output.status.code()
        );
    }
    Ok(())
}

/// Execute a piped shell command: `curl <curl_args...> | sh <sh_args...>`.
///
/// Commonly used for installing software via a remote script.
#[allow(clippy::zombie_processes)]
pub fn exec_piped_shell(curl_args: &[&str], sh_args: &[&str]) -> Result<()> {
    let cmd_str = format!("curl {} | sh {}", curl_args.join(" "), sh_args.join(" "));

    if is_dry_run() {
        info!("[dry-run] would exec piped: {}", cmd_str);
        log_operation(&format!("[dry-run] exec_piped: {}", cmd_str));
        return Ok(());
    }

    info!("exec_piped_shell: {}", cmd_str);
    log_operation(&format!("exec_piped: {}", cmd_str));

    let child = std::process::Command::new("curl")
        .args(curl_args)
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdout = child.stdout.context("Failed to capture curl stdout")?;

    let output = std::process::Command::new("sh")
        .args(sh_args)
        .stdin(Stdio::from(child_stdout))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Piped shell command failed with exit code: {:?}",
            output.status.code()
        );
    }
    Ok(())
}
