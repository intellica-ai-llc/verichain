const https = require('https');
const fs = require('fs');
const path = require('path');

const version = require('../package.json').version;
const platform = process.platform;
const arch = process.arch;

let assetName;
if (platform === 'linux') assetName = `seed-linux-${arch === 'x64' ? 'x64' : 'arm64'}`;
else if (platform === 'darwin') assetName = `seed-macos-${arch === 'x64' ? 'x64' : 'arm64'}`;
else if (platform === 'win32') assetName = 'seed-windows-x64.exe';
else throw new Error(`Unsupported platform: ${platform}`);

const url = `https://github.com/agentseedlanguage-cpu/agentseed/releases/download/v${version}/${assetName}`;
const dest = path.join(__dirname, '..', 'bin', platform === 'win32' ? 'seed.exe' : 'seed');

fs.mkdirSync(path.dirname(dest), { recursive: true });
const file = fs.createWriteStream(dest);
https.get(url, (response) => {
  if (response.statusCode >= 400) {
    console.error(`Download failed: HTTP ${response.statusCode}`);
    fs.unlink(dest, () => {});
    process.exit(1);
  }
  response.pipe(file);
  file.on('finish', () => {
    file.close();
    fs.chmodSync(dest, 0o755);
    console.log(`Installed ${assetName}`);
  });
}).on('error', (err) => {
  fs.unlink(dest, () => {});
  console.error(`Download error: ${err.message}`);
  process.exit(1);
});