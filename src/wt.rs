#[cfg(windows)]
use anyhow::Result;
#[cfg(windows)]
use log::{info, warn};

#[cfg(windows)]
use crate::base::write_file;
#[cfg(windows)]
use crate::common::{is_dry_run, log_operation};

/// Configure Windows Terminal with fish shell profile, SSH hosts from
/// `~/.ssh/config`, and font settings. Writes directly to the WT settings file.
#[cfg(windows)]
pub fn wt() -> Result<()> {
    use terminal_profiles::{
        FontConfig, Profile, ProfilesObject, ProfilesValue, WindowsTerminalSettings,
    };

    log_operation("install_wt: configuring Windows Terminal");

    if is_dry_run() {
        info!("[dry-run] would configure Windows Terminal");
        return Ok(());
    }

    fn get_settings_path() -> Option<String> {
        let mut local = dirs::data_local_dir()?;
        local.push("Packages");

        let Ok(dirs) = local.read_dir() else {
            return None;
        };

        for i in dirs {
            if let Ok(c) = i
                && c.file_name()
                    .to_string_lossy()
                    .starts_with("Microsoft.WindowsTerminal_")
            {
                local.push(c.file_name());
                local.push("LocalState");
                local.push("settings.json");
                return Some(local.to_string_lossy().to_string());
            }
        }
        None
    }

    let Some(p) = get_settings_path() else {
        info!("Windows Terminal settings not found, skipping.");
        return Ok(());
    };

    let Ok(s) = std::fs::read_to_string(&p) else {
        warn!("Failed to read Windows Terminal settings at {}", p);
        return Ok(());
    };
    let Ok(mut json) = serde_json::from_str::<WindowsTerminalSettings>(&s) else {
        warn!("Failed to parse Windows Terminal settings");
        return Ok(());
    };

    json.default_profile = String::new();

    let mut profiles = vec![];

    fn make_profile(name: &str, cmd: &str, icon: Option<&str>, hidden: Option<bool>) -> Profile {
        Profile {
            guid: None,
            name: Some(name.to_owned()),
            hidden,
            commandline: Some(cmd.to_owned()),
            icon: icon.map(|s| s.to_owned()),
            starting_directory: None,
            source: None,
            tab_title: None,
            suppress_application_title: None,
            tab_color: None,
            close_on_exit: None,
            background: None,
            background_image: None,
            background_image_alignment: None,
            background_image_opacity: None,
            background_image_stretch_mode: None,
            cursor_color: None,
            cursor_shape: None,
            cursor_height: None,
            foreground: None,
            selection_background: None,
            color_scheme: None,
            opacity: None,
            use_acrylic: None,
            padding: None,
            font: None,
            font_face: None,
            font_size: None,
            font_weight: None,
            antialiasing_mode: None,
            history_size: None,
            snap_on_input: None,
            alt_gr_aliasing: None,
            bell_style: None,
            bell_sound: None,
            intense_text_style: None,
            adjust_indistinguishable_colors: None,
            scrollbar_state: None,
            path_translation_style: None,
            elevate: None,
            environment: None,
            auto_mark_prompts: None,
            show_marks_on_scrollbar: None,
            unfocused_appearance: None,
        }
    }

    profiles.push(make_profile(
        "fish",
        "C:/msys64/msys2_shell.cmd -defterm -here -no-start -mingw64 -use-full-path -shell fish",
        Some("C:/msys64/mingw64.ico"),
        None,
    ));

    // --- dynamically generate SSH host profiles from ~/.ssh/config ---
    let mut ssh_added = false;
    if let Some(home) = dirs::home_dir() {
        let ssh_config = home.join(".ssh").join("config");
        if let Ok(content) = std::fs::read_to_string(&ssh_config) {
            // Collect concrete (non-wildcard) host names from Host directives
            let host_names: Vec<String> = content
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        return None;
                    }
                    let (kw, val) = trimmed.split_once(' ')?;
                    if !kw.eq_ignore_ascii_case("host") {
                        return None;
                    }
                    Some(
                        val.split_whitespace()
                            .filter(|p| !p.contains('*') && !p.contains('?') && !p.starts_with('!'))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .collect();

            for host_name in host_names {
                match russh_config::parse(&content, &host_name) {
                    Ok(config) => {
                        ssh_added = true;
                        let resolved_host = config.host().to_owned();
                        let user = if config.user().is_empty() {
                            "root".to_owned()
                        } else {
                            config.user().to_owned()
                        };
                        let port = config.port();

                        let mut ssh_cmd = format!(
                            "C:/msys64/msys2_shell.cmd -defterm -here -no-start -mingw64 -use-full-path -shell fish -c \"ssh {}@{}\"",
                            user, resolved_host
                        );
                        if port != 22 {
                            ssh_cmd.push_str(&format!(" -p {}", port));
                        }
                        if let Some(ref ids) = config.host_config.identity_file {
                            for id in ids {
                                ssh_cmd.push_str(&format!(" -i {}", id.display()));
                            }
                        }
                        profiles.push(make_profile(&host_name, &ssh_cmd, None, None));
                    }
                    Err(e) => {
                        warn!("Failed to parse SSH config for host '{}': {}", host_name, e);
                    }
                }
            }
        }
    }

    // Fallback: if no SSH hosts were found, add a default localhost entry (hidden)
    if !ssh_added {
        profiles.push(make_profile(
            "ssh-localhost",
            "C:/msys64/msys2_shell.cmd -defterm -here -no-start -mingw64 -use-full-path -shell fish -c \"ssh root@192.168.0.1 -p 8888 -i 'C:/Users/Admin/.ssh/root' -t '/usr/bin/fish' \"",
            None,
            Some(true),
        ));
    }

    let font = FontConfig {
        face: Some("FiraCode Nerd Font Mono".to_owned()),
        size: None,
        weight: None,
        features: None,
        axes: None,
        cell_width: None,
        cell_height: None,
    };
    let dft = Profile {
        elevate: Some(true),
        name: None,
        hidden: None,
        source: None,
        commandline: None,
        tab_title: None,
        starting_directory: dirs::desktop_dir().map(|p| p.to_string_lossy().to_string()),
        suppress_application_title: None,
        icon: None,
        tab_color: None,
        close_on_exit: None,
        background: None,
        background_image: None,
        background_image_alignment: None,
        background_image_opacity: None,
        background_image_stretch_mode: None,
        cursor_color: None,
        cursor_shape: None,
        cursor_height: None,
        foreground: None,
        selection_background: None,
        color_scheme: None,
        opacity: None,
        use_acrylic: None,
        padding: None,
        font: Some(font),
        font_face: None,
        font_size: None,
        font_weight: None,
        antialiasing_mode: None,
        history_size: None,
        snap_on_input: None,
        alt_gr_aliasing: None,
        bell_style: None,
        bell_sound: None,
        intense_text_style: None,
        adjust_indistinguishable_colors: None,
        scrollbar_state: None,
        path_translation_style: None,
        environment: None,
        auto_mark_prompts: None,
        show_marks_on_scrollbar: None,
        unfocused_appearance: None,
        guid: None,
    };
    let obj: ProfilesObject = ProfilesObject {
        list: profiles,
        defaults: Some(dft),
    };
    json.profiles = ProfilesValue::Object(Box::new(obj));

    let Ok(s) = serde_json::to_string_pretty(&json) else {
        warn!("Failed to serialize Windows Terminal settings");
        return Ok(());
    };

    info!("Writing Windows Terminal settings to {}", p);
    write_file(p, &s)?;
    Ok(())
}
