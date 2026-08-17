import { mkdir, mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';

const packageRoot = fileURLToPath(new URL('..', import.meta.url));
let smokeRoot;

try {
  const dryRun = await execa('npm', ['pack', '--dry-run', '--json', '--ignore-scripts'], { cwd: packageRoot });
  const output = JSON.parse(dryRun.stdout);
  const files = output[0]?.files?.map((entry) => entry.path) ?? [];
  const allowedFiles = new Set(['package.json', 'catalog.json', 'README.md', 'LICENSE']);
  const allowedPrefixes = ['dist/', 'skills/', 'assets/', 'schemas/'];
  const unexpected = files.filter((path) => !allowedFiles.has(path) && !allowedPrefixes.some((prefix) => path.startsWith(prefix)));
  const forbidden = files.filter((path) =>
    /^(?:\.local|\.docs|openspec|node_modules|src|scripts)(?:\/|$)|(?:^|\/)__pycache__(?:\/|$)|(?:\.pyc|\.log)$|(?:^|\/)\.env(?:\.|$)|backup/i.test(path),
  );
  if (files.length === 0 || unexpected.length > 0 || forbidden.length > 0) {
    const errors = [
      ...(files.length === 0 ? ['Package dry run returned no files.'] : []),
      ...unexpected.map((path) => `Unexpected package file: ${path}`),
      ...forbidden.map((path) => `Forbidden package file: ${path}`),
    ];
    throw new Error(errors.join('\n'));
  }

  smokeRoot = await mkdtemp(join(tmpdir(), 'arcantry-package-smoke-'));
  const installRoot = join(smokeRoot, 'install');
  const repositoryRoot = join(smokeRoot, 'repository');
  const linkRoot = join(smokeRoot, 'links');
  await Promise.all([mkdir(installRoot), mkdir(repositoryRoot), mkdir(linkRoot)]);

  const packed = await execa('npm', ['pack', '--json', '--ignore-scripts', '--pack-destination', smokeRoot], { cwd: packageRoot });
  const packedOutput = JSON.parse(packed.stdout);
  const filename = packedOutput[0]?.filename;
  if (typeof filename !== 'string') throw new Error('Package smoke could not resolve the packed archive.');

  await execa('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', '--prefix', installRoot, join(smokeRoot, filename)]);
  const cli = join(installRoot, 'node_modules', '@maxiedev', 'arcantry', 'dist', 'cli.js');
  const runCli = (args, cwd = smokeRoot) => execa(process.execPath, [cli, ...args], { cwd });

  await execa('git', ['init', '--quiet'], { cwd: repositoryRoot });
  await runCli(['--version']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'init', '--docs', 'none']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'update']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'doctor']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'validate']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'remove']);
  await runCli(['skills', 'list']);
  await runCli(['skills', 'inspect', 'adopt-arcantry']);
  await runCli(['skills', 'link', 'adopt-arcantry', '--target', linkRoot]);
  await runCli(['skills', 'doctor', '--target', linkRoot]);
  await runCli(['skills', 'unlink', 'adopt-arcantry', '--target', linkRoot]);

  process.stdout.write(`Package allowlist and packed CLI smoke passed (${files.length} files).\n`);
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
} finally {
  if (smokeRoot !== undefined) await rm(smokeRoot, { force: true, recursive: true, maxRetries: 3 });
}
