const u = require('./u');
const path = require('path');

const base = path.join(__dirname, '..');
u.runSync('"./node_modules/.bin/rimraf" dist', path.join(base, 'ui'));
u.runSync('"./node_modules/.bin/parcel" build src/index.html --public-url ./', path.join(base, 'ui'));
u.runSync('"node_modules/.bin/inline-assets" dist/index.html ../src/d', path.join(base, 'ui'));
u.runSync('rm -rf src/d.sz', base);
u.runSync('szip src/d', base);
u.runSync('cargo build --release', base);

if (u.isWin)
  u.runSync(
    'ResourceHacker.exe  -open ../target/release/telegramr.exe -save ../sfx/tr.exe -action addoverwrite -res ../icon.ico -mask ICONGROUP,1,',
    path.join(base, 'tools')
  );
