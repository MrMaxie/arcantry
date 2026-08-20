import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';

const platformPackages = {
  'win32-x64': '@arcantry/cli-win32-x64',
  'win32-arm64': '@arcantry/cli-win32-arm64',
  'darwin-x64': '@arcantry/cli-darwin-x64',
  'darwin-arm64': '@arcantry/cli-darwin-arm64',
  'linux-x64': '@arcantry/cli-linux-x64',
  'linux-arm64': '@arcantry/cli-linux-arm64',
} as const;

export type NativePlatform = keyof typeof platformPackages;

export const nativePackageFor = (platform: NodeJS.Platform, arch: string): string => {
  const key = `${platform}-${arch}` as NativePlatform;
  const packageName = platformPackages[key];
  if (packageName === undefined) {
    throw new Error(`Arcantry does not provide a native executable for ${platform}-${arch}. Use a supported GitHub Release archive.`);
  }
  return packageName;
};

export const resolveNativeExecutable = (
  platform: NodeJS.Platform = process.platform,
  arch: string = process.arch,
): string => {
  const packageName = nativePackageFor(platform, arch);
  const executable = platform === 'win32' ? 'arcantry.exe' : 'arcantry';
  try {
    return createRequire(import.meta.url).resolve(`${packageName}/bin/${executable}`);
  } catch {
    throw new Error(
      `The platform package ${packageName} is missing. Reinstall arcantry with optional dependencies enabled, or use the matching GitHub Release archive.`,
    );
  }
};

export const runNativeCli = (args: string[]): number => {
  try {
    const result = spawnSync(resolveNativeExecutable(), args, { stdio: 'inherit', windowsHide: true });
    if (result.error !== undefined) throw result.error;
    if (result.signal !== null) throw new Error(`Arcantry native CLI stopped after signal ${result.signal}.`);
    return result.status ?? 1;
  } catch (error) {
    process.stderr.write(`Error: ${error instanceof Error ? error.message : String(error)}\n`);
    return 1;
  }
};
