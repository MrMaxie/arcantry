import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { createServer } from 'node:http';
import { tmpdir } from 'node:os';
import { basename, join, resolve } from 'node:path';
import { Command } from 'commander';
import { execa } from 'execa';

type InstallerOptions = {
  artifacts: string;
};

type Execution = {
  code: number;
  stdout: string;
  stderr: string;
};

const parseOptions = (): InstallerOptions => {
  const program = new Command()
    .name('installer-smoke.ts')
    .description('Smoke-test the shell and PowerShell release installers.')
    .requiredOption('--artifacts <directory>', 'Native release artifact directory.')
    .showHelpAfterError();
  program.parse();
  return program.opts<InstallerOptions>();
};

const main = async (): Promise<void> => {
  const artifacts = resolve(parseOptions().artifacts);
  const temporary = await mkdtemp(join(tmpdir(), 'arcantry-installer-smoke-'));
  let corruptDownloads = false;

  const server = createServer(async (request, response) => {
    try {
      const name = basename(new URL(request.url ?? '/', 'http://127.0.0.1').pathname);
      let bytes = await readFile(join(artifacts, name));
      if (corruptDownloads && /\.(?:zip|tar\.xz)$/.test(name)) bytes = Buffer.concat([bytes, Buffer.from('corrupt')]);
      response.writeHead(200, { 'content-length': bytes.length });
      response.end(bytes);
    } catch {
      response.writeHead(404);
      response.end();
    }
  });

  await new Promise<void>((resolveListen, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolveListen);
  });
  const address = server.address();
  if (address === null || typeof address === 'string') throw new Error('Could not start the installer smoke server.');
  const downloadUrl = `http://127.0.0.1:${address.port}`;

  const execute = async (command: string, args: string[], installDirectory: string): Promise<Execution> => {
    const result = await execa(command, args, {
      env: {
        ...process.env,
        CARGO_DIST_FORCE_INSTALL_DIR: installDirectory,
        INSTALLER_DOWNLOAD_URL: downloadUrl,
      },
      reject: false,
      stdin: 'ignore',
    });
    return { code: result.exitCode ?? 1, stdout: result.stdout, stderr: result.stderr };
  };

  try {
    const installers: Array<[string, string[]]> =
      process.platform === 'win32'
        ? [['pwsh', ['-NoProfile', '-File', join(artifacts, 'arcantry-installer.ps1'), '-NoModifyPath']]]
        : [
            ['bash', [join(artifacts, 'arcantry-installer.sh'), '--no-modify-path']],
            ['pwsh', ['-NoProfile', '-File', join(artifacts, 'arcantry-installer.ps1'), '-NoModifyPath']],
          ];
    for (const [index, [command, args]] of installers.entries()) {
      const installDirectory = join(temporary, `install-${index}`);
      const installed = await execute(command, args, installDirectory);
      if (installed.code !== 0) throw new Error(installed.stderr || installed.stdout || `${command} installer failed`);

      const executable = join(installDirectory, 'bin', process.platform === 'win32' ? 'arcantry.exe' : 'arcantry');
      const version = await execute(executable, ['--version'], installDirectory);
      if (version.code !== 0 || version.stdout.trim() !== '1.0.0') {
        throw new Error(
          `${command} installer produced an invalid native executable: ${version.stderr || version.stdout}`,
        );
      }

      corruptDownloads = true;
      const rejected = await execute(command, args, join(temporary, `corrupt-${index}`));
      corruptDownloads = false;
      if (
        rejected.code === 0 ||
        !`${rejected.stdout}\n${rejected.stderr}`.toLowerCase().includes('checksum mismatch')
      ) {
        throw new Error(`${command} installer did not reject a corrupted archive.`);
      }
    }
    process.stdout.write(
      `${process.platform === 'win32' ? 'PowerShell installer' : 'Shell and PowerShell installers'} passed install, version, and checksum rejection smoke tests.\n`,
    );
  } finally {
    await new Promise<void>((resolveClose) => server.close(() => resolveClose()));
    await rm(temporary, { force: true, recursive: true, maxRetries: 3 });
  }
};

await main();
