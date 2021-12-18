#!/usr/bin/env node
import { $, cd } from "zx";
import os from "os";
import { chunk } from "lodash-es";

const home = os.homedir();

cd(home);

try {
    await $`code --version`;
  } catch (e) {
    await $`wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -`;
    await $`add-apt-repository "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main" -y`;
    await $`sudo apt install code -y`;
  
    // plugins
    const plugins = `alefragnani.project-manager
    amatiasq.sort-imports
    be5invis.toml
    CodeInChinese.EnglishChineseDictionary
    cschlosser.doxdocgen
    cssho.vscode-svgviewer
    dbaeumer.vscode-eslint
    donjayamanne.githistory
    eamodio.gitlens
    formulahendry.code-runner
    jawandarajbir.react-vscode-extension-pack
    jeff-hykin.better-cpp-syntax
    jock.svg
    johnsoncodehk.volar
    MaxvanderSchee.web-accessibility
    mhutchie.git-graph
    ms-dotnettools.csharp
    ms-python.python
    ms-python.vscode-pylance
    ms-vscode-remote.remote-containers
    ms-vscode-remote.remote-ssh
    ms-vscode-remote.remote-ssh-edit
    ms-vscode-remote.remote-wsl
    ms-vscode.cmake-tools
    ms-vscode.cpptools
    ms-vscode.cpptools-extension-pack
    ms-vscode.cpptools-themes
    ms-vscode.powershell
    msjsdiag.debugger-for-chrome
    msjsdiag.debugger-for-edge
    rust-lang.rust
    rvest.vs-code-prettier-eslint
    streetsidesoftware.code-spell-checker
    svipas.prettier-plus
    twxs.cmake
    wix.vscode-import-cost
    yzhang.markdown-all-in-one
    bungcip.better-toml
    cheshirekow.cmake-format
    codezombiech.gitignore
    donjayamanne.git-extension-pack
    donjayamanne.python-extension-pack
    dunstontc.vscode-rust-syntax
    dustypomerleau.rust-syntax
    evgeniypeshkov.syntax-highlighter
    formulahendry.auto-close-tag
    formulahendry.auto-rename-tag
    foxundermoon.shell-format
    Gruntfuggly.todo-tree
    JScearcy.rust-doc-viewer
    kisstkondoros.vscode-gutter-preview
    lorenzopirro.rust-flash-snippets
    magicstack.MagicPython
    matklad.rust-analyzer
    mike-co.import-sorter
    mikestead.dotenv
    miqh.vscode-language-rust
    mooman219.rust-assist
    ms-azuretools.vscode-docker
    ms-vscode-remote.vscode-remote-extensionpack
    nyxiative.rust-and-friends
    Orta.vscode-jest
    PolyMeilex.rust-targets
    ritwickdey.LiveServer
    serayuzgur.crates
    Shan.code-settings-sync
    statiolake.vscode-rustfmt
    steoates.autoimport
    stevencl.addDocComments
    Swellaby.rust-pack
    TabNine.tabnine-vscode
    tamasfe.even-better-toml
    techer.open-in-browser
    tht13.python
    usernamehw.errorlens
    vadimcn.vscode-lldb
    VisualStudioExptTeam.vscodeintellicode
    vscode-icons-team.vscode-icons
    wayou.vscode-todo-highlight
    xaver.clang-format
    ZhangYue.rust-mod-generator
    ziyasal.vscode-open-in-github`
      .split("\n")
      .map((i) => i.trim());
    const limit = 8;
    try {
      for (const list of chunk(plugins, limit)) {
        await Promise.all(
          list.map(
            (name) => name.length && $`code --install-extension ${name} --force`
          )
        );
      }
    } catch (e) {
      console.log(e);
    }
  }
  