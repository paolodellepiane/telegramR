const { execSync } = require('child_process');

run('"./node_modules/.bin/rimraf" dist', '../ui');
run('"./node_modules/.bin/parcel" build src/index.html --public-url ./', '../ui')
run('"node_modules/.bin/inline-assets" dist/index.html ../src/d', '../ui')
run('rm -rf src/d.sz', '../')
run('szip src/d', '../')
run('cargo build --release', '../')

function run(cmd, cwd) {
  console.log(`executing ${cmd}`);
  try {
    let output = execSync(cmd, { cwd });
    console.log(output.toString());
  } catch (e) {
    console.log(e);
    process.exit();
  }
}
