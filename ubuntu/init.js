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
const cv_dir = `${home}/tool/opencv`;
const zsh_dir = `${home}/.oh-my-zsh`;

if (!fs.existsSync(tool_dir)) {
  await $`mkdir tool`;
}
cd(tool_dir);
await $`sudo apt update -y`;
await $`sudo apt upgrade -y`;

const common_str = [
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
  "zsh"
];
await $`sudo apt install ${common_str} -y`;

try {
  await $`code --version`;
} catch (e) {
  await $`wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -`;
  await $`add-apt-repository "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main" -y`;
  await $`sudo apt install code`;
}

try {
  await $`google-chrome --version`;
} catch (e) {
  await $`wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb`;
  await $`sudo apt install ./google-chrome-stable_current_amd64.deb`;
}

try {
  await $`python --version`;
} catch (e) {
  await $`sudo ln -s /usr/bin/python3 /usr/bin/python`;
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
if (!fs.existsSync(cv_dir)) {
  // await $`sudo add-apt-repository "deb http://security.ubuntu.com/ubuntu xenial-security main" -y`;
  // await $`apt install libavcodec-dev libavformat-dev libswscale-dev -y`;
  cd(tool_dir);
  await $`git clone https://github.com/opencv/opencv.git --depth=1`;
  await $`git clone https://github.com/opencv/opencv_contrib.git --depth=1`;
  cd(cv_dir);
  // await $`mkdir -p build && cd build`
  await $`python3 ./platforms/js/build_js.py --emscripten_dir ~/tool/emsdk/upstream/emscripten build_wasm --build_wasm`;
  const s = `
  sudo add-apt-repository "deb http://security.ubuntu.com/ubuntu xenial-security main" -y

 
sudo apt install build-essential -y
sudo apt install cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev -y
 sudo apt install libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libjasper-dev libdc1394-22-dev -y
  # 如果需要python3支持的话，安装： sudo apt install python3-dev python3-numpy 
  # 如果需要ffmpeg支持的话： sudo apt install ffmpeg

  
sudo apt install build-essential cmake git pkg-config libgtk-3-dev \
    libavcodec-dev libavformat-dev libswscale-dev libv4l-dev \
    libxvidcore-dev libx264-dev libjpeg-dev libpng-dev libtiff-dev \
    gfortran openexr libatlas-base-dev python3-dev python3-numpy \
    libtbb2 libtbb-dev libdc1394-22-dev libopenexr-dev \
    libgstreamer-plugins-base1.0-dev libgstreamer1.0-dev  
    
    mkdir -p build && cd build
使用 CMake 命令配置 OpenCV 构建：
cmake -D CMAKE_BUILD_TYPE=RELEASE \
    -D CMAKE_INSTALL_PREFIX=/usr/local \
    -D INSTALL_C_EXAMPLES=ON \
    -D INSTALL_PYTHON_EXAMPLES=ON \
    -D OPENCV_GENERATE_PKGCONFIG=ON \
    -D OPENCV_EXTRA_MODULES_PATH=~/tool/opencv_contrib/modules \
    -D BUILD_EXAMPLES=ON ..
   
    
make -j4
sudo make install

`;
}

try {
  await $`deno --version`;
} catch (e) {
  await $`curl -fsSL https://deno.land/x/install/install.sh | sh`;
}

// 最后安装zsh, 因为安装后会激活shell
if (!fs.existsSync(zsh_dir)) {
  // code ~/.oh-my-zsh/themes/avit.zsh-theme
  await $`sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"`;
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