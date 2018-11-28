const u = require('./u');
const path = require('path');

const base = path.join(__dirname, '../');
const tools = path.join(base, 'tools');
// u.runSync('"./node_modules/.bin/rimraf" dist', path.join(base, 'ui'));
// u.runSync('"./node_modules/.bin/parcel" build src/index.html --public-url ./', path.join(base, 'ui'));
// u.runSync('rm -rf src/d', base);
// u.runSync('merger -m ../src/d ../ui/dist css,js,html', tools);
u.runSync('cargo build --release', base);

if (u.isWin)
  u.runSync(
    'ResourceHacker.exe  -open ../target/release/telegramr.exe -save ../sfx/tr.exe -action addoverwrite -res ../icon.ico -mask ICONGROUP,1,',
    tools
  );
