if status is-interactive
    # Commands to run in interactive sessions can go here
end
fish_add_path $HOME/.cargo/bin
fish_add_path $HOME/.deno/bin
fish_add_path $HOME/.ei
fish_add_path $HOME/.bun/bin

set -U fish_greeting

# set -x CARGO_TARGET_DIR "$HOME/ct"
set -x DENO_INSTALL "$HOME/.deno"

set -x SHELL /usr/bin/fish

set -g fish_color_command blue

alias ll="ls -lh"

alias p="pnpm"
alias px="pnpm dlx"
alias pd="pnpm dev"
alias pt="pnpm test"
alias pb="pnpm build"
alias pi="pnpm install"

alias c="cargo"
alias cr="cargo run"
alias cb="cargo build"
alias cbr="cargo build --release"

alias j="just"

starship init fish | source
# set -g MSYS2_ENV_CONV_EXCL "*"

function add_sub
    set video $argv[1]
    set sub $argv[2]
    set out $argv[3]

    # echo "ffmpeg -i $video -i $sub -c copy -map 0 -map -0:s -map 1 -metadata:s:s:0 language=chs -metadata:s:s:0 title='chs_eng' -disposition:s:0 default $out"
    command ffmpeg -i $video -i $sub -c copy -map 0 -map -0:s -map 1 -metadata:s:s:0 language=chs -metadata:s:s:0 title='chs_eng' -disposition:s:0 default $out
end


function to_win_path
    set -l p $argv[1]
    set -l winp (cygpath -w "$p")

    echo $winp
end


function emsdk_setup
    . ~/code/emsdk/emsdk_env.fish
end

function rs-setup -a toolchain stability
    # Determine platform and set target/host based on OS
    set -l platform (uname -s | string lower)
    set -l target
    set -l host

    switch $platform
        case linux
            set target x86_64-unknown-linux-gnu
            set host x86_64-unknown-linux-gnu
        # MINGW64_NT-10.0-26100
        case 'mingw*'
            # Default to Windows; adjust toolchain based on input
            switch $toolchain
                case msvc
                    set target x86_64-pc-windows-msvc
                    set host x86_64-pc-windows-msvc
                case gnu
                    set target x86_64-pc-windows-gnu
                    set host x86_64-pc-windows-gnu
                case '*'
                    echo "Error: Invalid toolchain '$toolchain'. Use 'msvc' or 'gnu' for Windows."
                    return 1
            end
    end

    # Set stability (nightly or stable)
    set -l toolchain_prefix $stability
    # if test -n "$toolchain_prefix"
    #     set toolchain_prefix $stability-
    # end

    # Install and configure Rust
    rustup target add $target
    rustup toolchain install $toolchain_prefix-$target
    rustup default $toolchain_prefix-$target
    rustup set default-host $host
end

# Convenience functions for common setups
function rs-msvc
    rs-setup msvc nightly
end

function rs-msvc-stable
    rs-setup msvc stable
end

function rs-gnu
    rs-setup gnu nightly
end

function rs-gnu-stable
    rs-setup gnu stable
end
function rs-nightly
    rustup toolchain install nightly
    rustup default nightly
end

function rs-stable
    rustup toolchain install stable
    rustup default stable
end
