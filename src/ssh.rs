use anyhow::Result;
use log::info;

use crate::base::{replace_file, write_file};
use crate::common::log_operation;

/// Configure SSH server with predefined sshd_config and disable login messages.
pub fn ssh() -> Result<()> {
    log_operation("ssh: configuring SSH server");
    info!("Configuring SSH server...");

    let sshd_config = include_str!("../files/ssh/sshd_config");
    write_file("/etc/ssh/sshd_config", sshd_config)?;

    // Disable sshd login message
    replace_file(
        "/etc/pam.d/sshd",
        "session    optional     pam_motd.so  motd=/run/motd.dynamic",
        "- session    optional     pam_motd.so  motd=/run/motd.dynamic",
    )?;
    replace_file(
        "/etc/pam.d/sshd",
        "session    optional     pam_motd.so noupdate",
        "- session    optional     pam_motd.so noupdate",
    )?;
    Ok(())
}
