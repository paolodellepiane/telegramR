const { execSync } = require('child_process');

run('"./node_modules/.bin/rimraf" dist', '../ui');
run('"./node_modules/.bin/parcel" build src/index.html --public-url ./', '../ui');
run('"node_modules/.bin/inline-assets" dist/index.html ../src/d', '../ui');
run('rm -rf src/d.sz', '..');
run('szip src/d', '..');
run('cargo build --release', '..');

if (process.platform === 'win32')
  run(
    'ResourceHacker.exe  -open ../target/release/telegramr.exe -save ../sfx/tr.exe -action addoverwrite -res ../icon.ico -mask ICONGROUP,1,',
    '../tools'
  );

function run(cmd, cwd) {
  console.log(`executing ${cmd}`);
  cwd = cwd || { cwd: '.' };
  try {
    let output = execSync(cmd, { cwd });
    console.log(output.toString());
  } catch (e) {
    console.log(e);
    process.exit();
  }
}
