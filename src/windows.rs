#[cfg(windows)]
use anyhow::Result;
#[cfg(windows)]
use log::info;

#[cfg(windows)]
use crate::{
    base::{append_file, replace_file},
    common::{get_msys64_root, log_operation},
};

/// Configure Windows/MSYS2 shell and path settings.
#[cfg(windows)]
pub fn windows() -> Result<()> {
    log_operation("windows: configuring MSYS2 settings");
    info!("Configuring Windows/MSYS2 settings...");

    let shell_path = get_msys64_root()? + "/etc/shells";
    if !std::fs::read_to_string(&shell_path)
        .unwrap_or_default()
        .contains("/usr/bin/fish")
    {
        append_file(&shell_path, "/usr/bin/fish")?;
    }

    replace_file(
        "C:/msys64/msys2_shell.cmd",
        "rem set MSYS2_PATH_TYPE=inherit",
        "set MSYS2_PATH_TYPE=inherit",
    )?;
    replace_file(
        "C:/msys64/etc/nsswitch.conf",
        "db_home: cygwin desc",
        "db_home: windows",
    )?;
    Ok(())
}
