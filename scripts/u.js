const { execSync, exec } = require('child_process');

exports.isWin = process.platform === 'win32';

exports.runSync = function(cmd, cwd) {
  console.log(`executing ${cmd}`);
  cwd = cwd || { cwd: '.' };
  try {
    let output = execSync(cmd, { cwd });
    console.log(output.toString());
  } catch (e) {
    console.log(e);
    process.exit();
  }
};

exports.run = function(cmd, cwd) {
  console.log(`executing ${cmd}`);
  cwd = cwd || { cwd: '.' };
  exec(cmd, { cwd }, (error, stdout, stderr) => {
    if (error) {
      console.error(`error: ${error}`);
      process.exit();
      return;
    }
    console.log(stdout);
    console.log(stderr);
  });
};
