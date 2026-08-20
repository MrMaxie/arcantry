import { join } from 'node:path';

export type NativeTarget = {
  triple: string;
  os: 'win32' | 'darwin' | 'linux';
  cpu: 'x64' | 'arm64';
  packageDirectory: string;
  packageName: string;
  executable: 'arcantry' | 'arcantry.exe';
  archive: string;
};

export const nativeTargets: readonly NativeTarget[] = [
  {
    triple: 'x86_64-pc-windows-msvc',
    os: 'win32',
    cpu: 'x64',
    packageDirectory: 'cli-win32-x64',
    packageName: '@arcantry/cli-win32-x64',
    executable: 'arcantry.exe',
    archive: 'arcantry-cli-x86_64-pc-windows-msvc.zip',
  },
  {
    triple: 'aarch64-pc-windows-msvc',
    os: 'win32',
    cpu: 'arm64',
    packageDirectory: 'cli-win32-arm64',
    packageName: '@arcantry/cli-win32-arm64',
    executable: 'arcantry.exe',
    archive: 'arcantry-cli-aarch64-pc-windows-msvc.zip',
  },
  {
    triple: 'x86_64-apple-darwin',
    os: 'darwin',
    cpu: 'x64',
    packageDirectory: 'cli-darwin-x64',
    packageName: '@arcantry/cli-darwin-x64',
    executable: 'arcantry',
    archive: 'arcantry-cli-x86_64-apple-darwin.tar.xz',
  },
  {
    triple: 'aarch64-apple-darwin',
    os: 'darwin',
    cpu: 'arm64',
    packageDirectory: 'cli-darwin-arm64',
    packageName: '@arcantry/cli-darwin-arm64',
    executable: 'arcantry',
    archive: 'arcantry-cli-aarch64-apple-darwin.tar.xz',
  },
  {
    triple: 'x86_64-unknown-linux-musl',
    os: 'linux',
    cpu: 'x64',
    packageDirectory: 'cli-linux-x64',
    packageName: '@arcantry/cli-linux-x64',
    executable: 'arcantry',
    archive: 'arcantry-cli-x86_64-unknown-linux-musl.tar.xz',
  },
  {
    triple: 'aarch64-unknown-linux-musl',
    os: 'linux',
    cpu: 'arm64',
    packageDirectory: 'cli-linux-arm64',
    packageName: '@arcantry/cli-linux-arm64',
    executable: 'arcantry',
    archive: 'arcantry-cli-aarch64-unknown-linux-musl.tar.xz',
  },
] as const;

export const nativeTarget = (triple: string): NativeTarget => {
  const target = nativeTargets.find((candidate) => candidate.triple === triple);
  if (!target) throw new Error(`Unsupported native target: ${triple}.`);
  return target;
};

export const downloadedBinaryPath = (root: string, target: NativeTarget): string => {
  return join(root, `native-${target.triple}`, target.triple, 'dist', target.executable);
};

export const builtBinaryPath = (root: string, target: NativeTarget): string => {
  return join(root, target.triple, 'dist', target.executable);
};

export const npmPackageManifests = [
  ...nativeTargets.map((target) => join('packages', target.packageDirectory, 'package.json')),
  join('packages', 'arcantry', 'package.json'),
] as const;
