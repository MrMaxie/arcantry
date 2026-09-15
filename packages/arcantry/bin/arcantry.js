#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';

const platformPackages = {
  'win32-x64': '@arcantry/cli-win32-x64',
  'darwin-x64': '@arcantry/cli-darwin-x64',
  'darwin-arm64': '@arcantry/cli-darwin-arm64',
  'linux-x64': '@arcantry/cli-linux-x64',
};

function resolveNativeExecutable() {
  const platform = process.platform;
  const architecture = process.arch;
  const packageName = platformPackages[`${platform}-${architecture}`];
  if (packageName === undefined) {
    throw new Error(
      `Arcantry does not provide a native executable for ${platform}-${architecture}. Use a supported GitHub Release archive.`,
    );
  }

  const executable = platform === 'win32' ? 'arcantry.exe' : 'arcantry';
  try {
    return createRequire(import.meta.url).resolve(`${packageName}/bin/${executable}`);
  } catch {
    throw new Error(
      `The platform package ${packageName} is missing. Reinstall arcantry with optional dependencies enabled, or use the matching GitHub Release archive.`,
    );
  }
}

try {
  const result = spawnSync(resolveNativeExecutable(), process.argv.slice(2), {
    stdio: 'inherit',
    windowsHide: true,
  });
  if (result.error !== undefined) throw result.error;
  if (result.signal !== null) {
    throw new Error(`Arcantry native CLI stopped after signal ${result.signal}.`);
  }
  process.exitCode = result.status ?? 1;
} catch (error) {
  process.stderr.write(`Error: ${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
}
