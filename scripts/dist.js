const u = require('./u');
const path = require('path');
console.log(process.env.RPC);
const base = path.join(__dirname, '../');
u.runSync('"./node_modules/.bin/rimraf" dist', path.join(base, 'ui'));
u.runSync('"./node_modules/.bin/parcel" build src/index.html --public-url ./', path.join(base, 'ui'));
