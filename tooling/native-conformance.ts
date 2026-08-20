import { resolve } from 'node:path';
import { execa } from 'execa';

const executable = process.platform === 'win32' ? 'arcantry.exe' : 'arcantry';
const binary = resolve('target', 'debug', executable);
const nub = 'nub';
const { NODE_OPTIONS: _nodeOptions, ...cleanEnvironment } = process.env;
await execa(nub, ['exec', '--cwd', 'packages/arcantry', 'vitest', 'run', 'src/nativeConformance.test.ts'], {
  env: { ...cleanEnvironment, ARCANTRY_NATIVE_BIN: binary },
  extendEnv: false,
  stdio: 'inherit',
});
