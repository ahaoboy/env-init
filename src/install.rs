use anyhow::Result;
use log::{info, warn};

use crate::{
    base::write_file,
    common::{command_exists, ei, ei_cargo, get_home, log_operation},
    exec::exec,
};

#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::base::append_file;

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

/// Add fish to `/etc/shells` and set it as the default shell for root.
#[cfg(target_os = "linux")]
fn configure_fish_shell() -> Result<()> {
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

/// Restart the SSH server using the available service manager.
#[cfg(target_os = "linux")]
fn restart_ssh() {
    if command_exists("service") {
        let _ = exec("service", vec!["ssh", "restart"]);
    } else if command_exists("rc-service") {
        // OpenRC (Alpine Linux)
        let _ = exec("rc-service", vec!["sshd", "restart"]);
    } else {
        warn!("No supported service manager found. Cannot restart SSH service.");
    }
}

/// Install development tools on Linux.
///
/// Detects the package manager and dispatches to the matching implementation:
/// `apk` on Alpine Linux, `apt` on Debian/Ubuntu.
#[cfg(target_os = "linux")]
pub async fn install() -> Result<()> {
    if command_exists("apk") {
        install_apk().await
    } else if command_exists("apt") {
        install_apt().await
    } else {
        anyhow::bail!(
            "No supported package manager found. This script requires 'apk' (Alpine) or 'apt' (Debian/Ubuntu)."
        );
    }
}

/// Install development tools on Alpine Linux via apk.
#[cfg(target_os = "linux")]
async fn install_apk() -> Result<()> {
    log_operation("install: installing development tools (alpine)");
    info!("Installing development tools via apk...");

    let v = vec![
        "perl",
        "gedit",
        "net-tools",
        "openssh",
        "clang",
        "lld",
        "gdb",
        "openjdk17-jre",
        "gcc",
        "g++",
        "cmake",
        "openssl",
        "openssl-dev",
        "git",
        "pkgconf",
        "build-base",
        "nasm",
        "wget",
        "curl",
        "xsel",
        "zip",
        "unzip",
        "tar",
        "python3",
        "py3-pip",
        "font-firacode",
        "clang-extra-tools",
        "zsh",
        "ninja",
        "p7zip",
        "autoconf",
        "autoconf-archive",
        "gettext",
        "musl-dev",
        "txt2man",
        "jq",
        "fish",
    ];

    let _ = exec("apk", vec!["update"]);
    let _ = exec("apk", vec!["upgrade"]);

    for p in v {
        if let Err(e) = exec("apk", vec!["add", p]) {
            warn!("Failed to install {}: {}", p, e);
        }
    }

    Ok(())
}

/// Install development tools on Debian/Ubuntu via apt.
#[cfg(target_os = "linux")]
async fn install_apt() -> Result<()> {
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

    configure_fish_shell()?;
    restart_ssh();

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
        "ahaoboy/fsv",
        "ahaoboy/ansi2",
        "ahaoboy/neofetch",
        "ahaoboy/ftp-web",
        "easy-install/easy-install",
        "oven-sh/bun",
        "upx/upx",
        "mobile-shell/mosh",
        "pranshuparmar/witr",
        "ajeetdsouza/zoxide",
        #[cfg(windows)]
        "ahaoboy/copy-path",
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
    crate::wt::wt()?;

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
