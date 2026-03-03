use anyhow::Result;
use log::info;

use crate::base::{get_file_dir, FILES};
use crate::common::{is_dry_run, log_operation};

/// Initialize embedded configuration files to the local files directory.
pub fn init() -> Result<()> {
    log_operation("init: extracting embedded files");
    if is_dry_run() {
        for (name, _) in FILES {
            info!("[dry-run] would extract: {}", name);
        }
        return Ok(());
    }
    let file_dir = get_file_dir()?;
    for (name, s) in FILES {
        let file_path = file_dir.join(name);
        info!("Extracting file: {:?}", file_path);
        std::fs::write(&file_path, s)?;
    }
    Ok(())
}
