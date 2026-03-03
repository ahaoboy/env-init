use anyhow::Result;
use log::info;

use crate::base::write_file;
use crate::common::log_operation;

/// Configure root auto-login by deploying PAM and GDM configuration files.
pub fn root() -> Result<()> {
    log_operation("root: configuring root auto-login");
    info!("Configuring root auto-login...");

    let gdm_autologin = include_str!("../files/ssh/gdm-autologin");
    let gdm_password = include_str!("../files/ssh/gdm-password");
    let custom = include_str!("../files/ssh/custom.conf");
    let chsh = include_str!("../files/ssh/chsh");

    write_file("/etc/pam.d/gdm-autologin", gdm_autologin)?;
    write_file("/etc/pam.d/gdm-password", gdm_password)?;
    write_file("/etc/gdm3/custom.conf", custom)?;
    write_file("/etc/pam.d/chsh", chsh)?;
    Ok(())
}
