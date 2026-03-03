use anyhow::Result;
use log::{info, warn};

use crate::{
    base::{append_file, write_file},
    common::{command_exists, ei, ei_cargo, get_home, log_operation},
    exec::exec,
};

/// Configure git with predefined aliases and settings.
pub fn config_git() -> Result<()> {
    log_operation("git: configuring git");
    info!("Configuring git...");

    let gitalias = include_str!("../files/gitalias.txt");
    let git_config = include_str!("../files/.gitconfig");

    let home = get_home()?;
    write_file(home.clone() + "/.gitconfig", git_config)?;
    write_file(home.clone() + "/gitalias.txt", gitalias)?;
    Ok(())
}

/// Install development tools on Linux via apt.
#[cfg(target_os = "linux")]
pub async fn install() -> Result<()> {
    log_operation("install: installing development tools (linux)");

    if !command_exists("apt") {
        anyhow::bail!("'apt' command not found. This script requires apt package manager.");
    }

    if !command_exists("add-apt-repository") {
        warn!("'add-apt-repository' not found. Installing software-properties-common...");
        let _ = exec("apt", vec!["update", "-y"]);
        let _ = exec("apt", vec!["install", "-y", "software-properties-common"]);
    }

    let v = vec![
        "perl",
        "gedit",
        "net-tools",
        "openssh-server",
        "clang",
        "lld",
        "gdb",
        "default-jre",
        "gcc",
        "g++",
        "cmake",
        "openssl",
        "libssl-dev",
        "git",
        "pkg-config",
        "build-essential",
        "nasm",
        "wget",
        "curl",
        "xsel",
        "zip",
        "unzip",
        "tar",
        "software-properties-common",
        "apt-transport-https",
        "python3",
        "python3-pip",
        "fonts-firacode",
        "clang-format",
        "zsh",
        "ninja-build",
        "p7zip-full",
        "autoconf",
        "autoconf-archive",
        "gettext",
        "gettext-base",
        "autopoint",
        "build-essential",
        "libc6-dev",
        "txt2man",
        "jq",
    ];

    if command_exists("add-apt-repository") {
        let _ = exec("add-apt-repository", vec!["ppa:git-core/ppa", "-y"]);
        let _ = exec("add-apt-repository", vec!["ppa:fish-shell/release-4", "-y"]);
    }

    let _ = exec("apt", vec!["update", "-y"]);
    let _ = exec("apt", vec!["upgrade", "-y"]);
    let _ = exec("apt", [vec!["install", "-y"], v].concat());
    let _ = exec("apt", vec!["update", "-y"]);
    let _ = exec("apt", vec!["install", "-f"]);
    let _ = exec("apt", vec!["upgrade", "-y"]);
    let _ = exec("apt", vec!["install", "git", "-y"]);

    config_git()?;

    let _ = exec("apt", vec!["install", "fish", "-y"]);

    let shell_path = "/etc/shells";
    if !std::fs::read_to_string(shell_path)
        .unwrap_or_default()
        .contains("/usr/bin/fish")
    {
        append_file(shell_path, "/usr/bin/fish")?;
    }

    if command_exists("chsh") {
        let _ = exec("chsh", vec!["-s", "/usr/bin/fish", "root"]);
    } else {
        warn!("'chsh' command not found. Cannot change default shell.");
    }

    if command_exists("service") {
        let _ = exec("service", vec!["ssh", "restart"]);
    } else {
        warn!("'service' command not found. Cannot restart SSH service.");
    }

    Ok(())
}

/// Install CLI tools via easy-install.
#[allow(clippy::zombie_processes)]
pub async fn install_ei() -> Result<()> {
    log_operation("install_ei: installing CLI tools");
    info!("Installing CLI tools via easy-install...");

    let v = vec![
        "starship/starship",
        "sigoden/dufs",
        "XAMPPRocky/tokei",
        "sharkdp/hyperfine",
        "ahaoboy/rserve",
        "ahaoboy/ansi2",
        "ahaoboy/neofetch",
        "ahaoboy/ftp-web",
        "ahaoboy/chokidar",
        "easy-install/easy-install",
        "oven-sh/bun",
        "upx/upx",
        "mobile-shell/mosh",
        "pranshuparmar/witr",
        "ajeetdsouza/zoxide",
        #[cfg(windows)]
        "ahaoboy/copy-path",
        #[cfg(windows)]
        "ahaoboy/serve-dav",
        "ahaoboy/mujs-build",
        "quickjs-ng/quickjs",
        "casey/just",
        "ahaoboy/bloaty-build",
        "ahaoboy/bloaty-metafile",
        "ducaale/xh",
        #[cfg(windows)]
        "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/FiraCode.zip",
    ];

    for i in v {
        if let Err(e) = ei(i).await {
            warn!("Failed to install {}: {}", i, e);
        }
    }

    for i in ["cargo-bins/cargo-binstall", "drager/wasm-pack"] {
        if let Err(e) = ei_cargo(i).await {
            warn!("Failed to install {}: {}", i, e);
        }
    }
    Ok(())
}

/// Configure Windows/MSYS2 shell and path settings.
#[cfg(windows)]
pub fn windows() -> Result<()> {
    use crate::{base::replace_file, common::get_msys64_root};

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

/// Install development tools on Windows via pacman (MSYS2).
#[cfg(target_os = "windows")]
pub async fn install() -> Result<()> {
    log_operation("install: installing development tools (windows)");

    if !command_exists("pacman") {
        anyhow::bail!(
            "'pacman' command not found. This script requires MSYS2 with pacman package manager."
        );
    }

    let _ = exec("pacman", vec!["-Syy", "--noconfirm"]);

    let v = vec![
        "mingw-w64-x86_64-gcc",
        "mingw-w64-x86_64-cmake",
        "mingw-w64-x86_64-make",
        "mingw-w64-x86_64-jq",
        "fish",
        "tree",
        "mingw-w64-x86_64-7zip",
        "zip",
        "unzip",
        "mingw-w64-x86_64-curl",
        "wget",
        "bash",
        "nano",
        "make",
        "patch",
        "autotools",
        "gettext-devel",
        "autoconf-archive",
        "openssh",
    ];

    let _ = exec("pacman", vec!["-Syu", "--noconfirm"]);

    for i in v {
        if let Err(e) = exec("pacman", vec!["-S", "--noconfirm", i]) {
            warn!("Failed to install {}: {}", i, e);
        }
    }

    config_git()?;
    install_wt()?;

    Ok(())
}

/// Install development tools on Android via pkg (Termux).
#[cfg(target_os = "android")]
pub async fn install() -> Result<()> {
    log_operation("install: installing development tools (android)");

    config_git()?;

    if !command_exists("pkg") {
        anyhow::bail!("'pkg' command not found. This script requires Termux package manager.");
    }

    let v = vec![
        "openssh",
        "openssl-tool",
        "termux-services",
        "tsu",
        "rust",
        "wget",
        "tar",
        "git",
        "perl",
        "cmake",
        "fish",
        "zip",
        "curl",
        "clang",
        "nodejs",
        "which",
    ];

    let _ = exec("pkg", vec!["upgrade", "-y"]);
    let _ = exec("pkg", [vec!["install", "-y"], v].concat());

    let shell_path = "/etc/shells";
    if !std::fs::read_to_string(shell_path)
        .unwrap_or_default()
        .contains("/usr/bin/fish")
    {
        append_file(shell_path, "/usr/bin/fish")?;
    }

    if command_exists("chsh") {
        let _ = exec("chsh", vec!["-s", "/usr/bin/fish", "root"]);
    } else {
        warn!("'chsh' command not found. Cannot change default shell.");
    }

    Ok(())
}

/// Configure Windows Terminal with fish shell profile and font settings.
#[cfg(windows)]
fn install_wt() -> Result<()> {
    use crate::common::is_dry_run;
    use terminal_profiles::{FontConfig, ProfilesObject};
    use terminal_profiles::{Profile, ProfilesValue, WindowsTerminalSettings};

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

    for (commandline, icon, name) in [(
        "C:/msys64/msys2_shell.cmd -defterm -here -no-start -mingw64  -use-full-path -shell fish",
        "C:/msys64/mingw64.ico",
        "fish",
    )] {
        profiles.push(Profile {
            guid: None,
            name: Some(name.to_owned()),
            starting_directory: None,
            hidden: None,
            source: None,
            commandline: Some(commandline.to_owned()),
            tab_title: None,
            suppress_application_title: None,
            icon: Some(icon.to_owned()),
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
        })
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
        starting_directory: Some("c:/code".to_owned()),
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
    let _ = std::fs::write(p, s);
    Ok(())
}
