#!/usr/bin/env node
import { $, cd } from "zx";
import os from "os";
import fs from "fs";
import path from "path";

const init_dir = path.resolve("./");
const home = os.homedir();

cd(home);

const tool_dir = `${home}/tool`;
const vcpkg_dir = `${home}/tool/vcpkg`;
const emsdk_dir = `${home}/tool/emsdk`;
const zsh_dir = `${home}/.oh-my-zsh`;

if (!fs.existsSync(tool_dir)) {
  await $`mkdir tool`;
}
cd(tool_dir);
await $`sudo apt update -y`;
await $`sudo apt upgrade -y`;

const common_str = [
  "default-jre",
  "gcc",
  "g++",
  "cmake",
  "openssl",
  "libssl-dev",
  "cmake",
  "git",
  "libgtk2.0-dev",
  "pkg-config",
  "libavcodec-dev",
  "libavformat-dev",
  "libswscale-dev",
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
].join(" ");
await $`sudo apt install ${common_str} -y`;

try {
  await $`code --version`;
} catch (e) {
  await $`wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -`;
  await $`add-apt-repository "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main"`;
  await $`sudo apt install code`;
}

try {
  await $`google-chrome --version`;
} catch (e) {
  await $`wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb`;
  await $`sudo apt install ./google-chrome-stable_current_amd64.deb`;
}

if (!fs.existsSync(vcpkg_dir)) {
  cd(tool_dir);
  await $`git clone https://github.com/Microsoft/vcpkg.git --depth=1`;
  cd(`${tool_dir}/vcpkg`);
  await $`./bootstrap-vcpkg.sh`;
}

if (!fs.existsSync(emsdk_dir)) {
  cd(tool_dir);
  await $`git clone https://github.com/juj/emsdk.git --depth=1`;
  cd(emsdk_dir);
  await $`git pull`;
  await $`./emsdk install latest`;
  await $`./emsdk activate latest`;
  await $`source ./emsdk_env.sh`;
}
try {
  await $`zsh --version`;
} catch (e) {
  await $`curl -fsSL https://deno.land/x/install/install.sh | sh`;
  await $`sudo apt install zsh -y`;
  await $`sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"`;
}

if (!fs.existsSync(zsh_dir)) {
  // code ~/.oh-my-zsh/themes/avit.zsh-theme
  const ace_init_path = path.join(init_dir, "ubuntu", "ace.zsh-theme");
  const ace_zsh_path = "~/.oh-my-zsh/themes/ace.zsh-theme";
  await $`cp -avxf ${ace_init_path} ${ace_zsh_path}`;
  const config_init_path = path.join(init_dir, "ubuntu", ".zshrc");
  const config_zsh__path = "~/.zshrc";

  await $`cp -avxf ${config_init_path} ${config_zsh__path}`;
  await $`git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting --depth=1`;
  await $`git clone https://github.com/zsh-users/zsh-autosuggestions ~/.oh-my-zsh/custom/plugins/zsh-autosuggestions --depth=1`;
  await $`chsh -s /bin/zsh`;
}
