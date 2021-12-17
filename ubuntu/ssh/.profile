# ~/.profile: executed by Bourne-compatible login shells.

if [ "$BASH" ]; then
  if [ -f ~/.bashrc ]; then
    . ~/.bashrc
  fi
fi

# mesg n 2> /dev/null || true
tty -s&&mesg n || true
. "$HOME/.cargo/env"
