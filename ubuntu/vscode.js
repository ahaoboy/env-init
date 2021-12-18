#!/usr/bin/env node
import { $, cd } from "zx";
import os from "os";
import fs from "fs";
const home = os.homedir();
cd(home);

try {
  await $`code --version`;
} catch (e) {
  await $`wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -`;
  await $`add-apt-repository "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main" -y`;
  await $`sudo apt install code -y`;

  // plugins
  const plugins = JSON.parse(fs.readFileSync("./ubuntu/plugin.json", "utf-8"));
  const limit = 8;

  await new Promise(async (r) => {
    const f = (i) => {
      const name = plugins.pop();
      const p = $`code --install-extension ${name} --force`;
      console.log("i:", i, name, plugins.length);
      const next = async () => {
        if (plugins.length === 0) {
          r();
        } else {
          try {
            await p;
          } catch (err) {
            console.log("err:", err);
          }
          f(i);
        }
      };
      p.then(next);
      p.catch(next);
    };

    for (let i = 0; i < limit; i++) {
      f(i);
    }
  });
}
