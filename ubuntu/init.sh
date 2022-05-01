sudo apt install curl -y

echo "=========install nodejs========="
# curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

# source ~/.bashrc

# nvm list 
# nvm install 16   # or 10.10.0, 8.9.1, etc 
# nvm use 16

curl -fsSL https://fnm.vercel.app/install | bash
source ~/.bashrc
fnm install 16
fnm use 16 
fnm default 16


npm i pnpm yarn trash-cli -g
pnpm i serve -g
pnpm i zx -g
echo "=========finish nodejs========="
pnpm i
node ubuntu/install.js
node ubuntu/init.js
node ubuntu/vscode.js