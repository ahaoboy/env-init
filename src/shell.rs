use anyhow::Result;
use log::info;

use crate::base::{write_file, STARSHIP};
use crate::common::{get_home, log_operation};

/// Configure fish shell and starship prompt with predefined settings.
pub fn shell() -> Result<()> {
    log_operation("shell: configuring fish + starship");
    info!("Configuring fish shell and starship prompt...");

    let home = get_home()?;
    let fish = include_str!("../files/config.fish");

    write_file(home.clone() + "/.config/starship.toml", STARSHIP)?;
    write_file(home.clone() + "/.config/fish/config.fish", fish)?;
    Ok(())
}
