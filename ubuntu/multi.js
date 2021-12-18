import fs from "fs";
// plugins
const plugins = JSON.parse(fs.readFileSync("./ubuntu/plugin.json", "utf-8"));
const limit = 8;
const sleep = () =>
  new Promise((r) => setTimeout(r, 1000 + Math.random() * 100));
await new Promise(async (r) => {
  const f = (i) => {
    const name = plugins.pop();
    const p = sleep();
    console.log("i:", i, name, plugins.length);
    const next = async () => {
      if (plugins.length === 0) {
        r();
      } else {
        await p;
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
