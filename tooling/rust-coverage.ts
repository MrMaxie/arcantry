import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const run = (command: string, args: string[], env = process.env) => {
  const result = spawnSync(command, args, { env, stdio: 'inherit' });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${command} ${args.join(' ')} exited with ${result.status}`);
};

const shown = spawnSync('cargo', ['llvm-cov', 'show-env', '--branch'], { encoding: 'utf8' });
if (shown.error) throw shown.error;
if (shown.status !== 0) throw new Error(shown.stderr);
const coverageEnvironment = { ...process.env };
for (const line of shown.stdout.split(/\r?\n/u)) {
  const match = /^([^=]+)=(.*)$/u.exec(line);
  if (!match) continue;
  const [, key, rawValue] = match;
  const value = rawValue.startsWith("'") && rawValue.endsWith("'") ? rawValue.slice(1, -1) : rawValue;
  coverageEnvironment[key] = value;
}

run('cargo', ['llvm-cov', 'clean', '--workspace'], coverageEnvironment);
run('cargo', ['build', '--workspace'], coverageEnvironment);
coverageEnvironment.ARCANTRY_BIN = `${process.cwd()}/target/debug/arcantry${process.platform === 'win32' ? '.exe' : ''}`;
const fixture = mkdtempSync(join(tmpdir(), 'arcantry-coverage-'));
try {
  run('git', ['init', '--quiet', fixture], coverageEnvironment);
  for (const args of [
    ['--cwd', fixture, 'repo', 'inspect', '--json'],
    ['--cwd', fixture, 'todo', 'list'],
    ['--cwd', fixture, 'release', 'plan'],
    ['--cwd', process.cwd(), 'skills', 'list'],
  ]) {
    const result = spawnSync(coverageEnvironment.ARCANTRY_BIN, args, {
      env: coverageEnvironment,
      encoding: 'utf8',
    });
    if (result.error) throw result.error;
    if (result.status !== 0 && args[2] !== 'release') {
      throw new Error(`Instrumented arcantry ${args.join(' ')} exited with ${result.status}: ${result.stderr}`);
    }
  }
} finally {
  rmSync(fixture, { force: true, recursive: true });
}
run('cargo', ['test', '--workspace'], coverageEnvironment);
run(
  'cargo',
  ['llvm-cov', 'report', '--branch', '--include-build-script', '--lcov', '--output-path', 'target/rust-coverage.lcov'],
  coverageEnvironment,
);
