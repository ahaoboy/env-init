echo "=========install nodejs========="
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash
source ~/.bashrc

nvm list 
nvm install 16   # or 10.10.0, 8.9.1, etc 
nvm use 16
npm i pnpm yarn -g
pnpm i serve -g
pnpm i zx -g
echo "=========finish nodejs========="

node ubuntu/init.js