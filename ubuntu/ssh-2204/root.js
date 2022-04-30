#!/usr/bin/env node
import fs from "fs";

const backup = (p) => {
  const s = fs.readFileSync(p, "utf-8");
  fs.writeFileSync(p + ".bk", s);
};

{
  const p = "/usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf";
  backup(p);
  fs.writeFileSync(p, fs.readFileSync("./50-ubuntu.conf", "utf-8"));
}

{
  const p = "/etc/pam.d/gdm-autologin";
  backup(p);
  const s = fs
    .readFileSync(p, "utf-8")
    .replace(
      "auth	required	pam_succeed_if.so user != root quiet_success",
      "#auth	required	pam_succeed_if.so user != root quiet_success"
    );
  fs.writeFileSync(p, s);
}

{
  const p = "/etc/pam.d/gdm-password";
  backup(p);
  const s = fs
    .readFileSync(p, "utf-8")
    .replace(
      "auth	required	pam_succeed_if.so user != root quiet_success",
      "#auth	required	pam_succeed_if.so user != root quiet_success"
    );
  fs.writeFileSync(p, s);
}


{
  const p = "/etc/gdm3/custom.conf";
  backup(p);
  fs.writeFileSync(p, fs.readFileSync("./custom.conf", "utf-8"));
}

{
  const p = "/etc/ssh/sshd_config";
  backup(p);
  fs.writeFileSync(p, fs.readFileSync("./sshd_config", "utf-8"));
}

{
// sudo apt install openssh-server net-tools -y
// sudo /etc/init.d/ssh start 

// ps -e | grep ssh
}