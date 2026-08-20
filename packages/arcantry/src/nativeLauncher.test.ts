import { describe, expect, it } from 'vitest';
import { nativePackageFor } from './nativeLauncher.js';

describe('native launcher', () => {
  it('maps every declared operating system and architecture without libc detection', () => {
    expect(nativePackageFor('win32', 'x64')).toBe('@arcantry/cli-win32-x64');
    expect(nativePackageFor('win32', 'arm64')).toBe('@arcantry/cli-win32-arm64');
    expect(nativePackageFor('darwin', 'x64')).toBe('@arcantry/cli-darwin-x64');
    expect(nativePackageFor('darwin', 'arm64')).toBe('@arcantry/cli-darwin-arm64');
    expect(nativePackageFor('linux', 'x64')).toBe('@arcantry/cli-linux-x64');
    expect(nativePackageFor('linux', 'arm64')).toBe('@arcantry/cli-linux-arm64');
  });

  it('rejects unsupported combinations', () => {
    expect(() => nativePackageFor('freebsd', 'x64')).toThrow('does not provide a native executable');
  });
});
