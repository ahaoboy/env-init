#!/usr/bin/env node
import { $ } from "zx";

await $`sudo apt update -y`;
await $`sudo apt upgrade -y`;

const common_str = [
  "net-tools",
  "openssh-server",
  "clang",
  "lld",
  "gdb",
  "default-jre",
  "net-tools",
  "gcc",
  "g++",
  "cmake",
  "openssl",
  "libssl-dev",
  "cmake",
  "git",
  "libgtk2.0-dev",
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
];
await $`sudo apt install ${common_str} -y`;
