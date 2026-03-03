use std::path::Path;

use anyhow::Result;
use log::{info, warn};

use crate::base::get_file_dir;
use crate::common::log_operation;

/// Reset modified system configuration files from their backup copies.
pub fn reset() -> Result<()> {
    log_operation("reset: restoring system configuration files");
    info!("Restoring system configuration files from backup...");

    let v = vec![
        "/usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf",
        "/etc/pam.d/gdm-autologin",
        "/etc/pam.d/gdm-password",
        "/etc/gdm3/custom.conf",
        "/etc/ssh/sshd_config",
    ];

    let file_dir = get_file_dir()?;

    for s in v {
        let out_path = Path::new(s);
        let Some(file_name) = out_path.file_name() else {
            warn!("Failed to get file name of {:?}", out_path);
            continue;
        };
        let name = file_name.to_string_lossy().to_string() + ".bk";
        let file_path = file_dir.join(&name);
        let contents = match std::fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to read backup {:?}: {}", file_path, e);
                continue;
            }
        };
        if let Err(e) = std::fs::write(out_path, contents) {
            warn!("Failed to restore {:?}: {}", out_path, e);
        } else {
            info!("Restored: {:?}", out_path);
        }
    }
    Ok(())
}
