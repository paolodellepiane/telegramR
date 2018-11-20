const u = require('./u');
const path = require('path');

const base = path.join(__dirname, '..');
u.run('cargo run --features "use-ws"', base);
u.run('yarn start', path.join(base, 'ui'));
