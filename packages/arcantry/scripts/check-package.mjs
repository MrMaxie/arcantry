import { copyFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, isAbsolute, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';

const packageRoot = fileURLToPath(new URL('..', import.meta.url));
const nubCommand = process.platform === 'win32' ? 'nub.cmd' : 'nub';
const outputFlag = process.argv.indexOf('--output');
if (outputFlag !== -1 && !process.argv[outputFlag + 1]) throw new Error('usage: check-package.mjs [--output <directory>]');
const retainedOutput = outputFlag === -1 ? undefined : resolve(process.argv[outputFlag + 1]);
let smokeRoot;

try {
  const dryRun = await execa(nubCommand, ['pack', '--dry-run', '--json', '--ignore-scripts'], { cwd: packageRoot });
  const output = JSON.parse(dryRun.stdout);
  const files = output[0]?.files?.map((entry) => entry.path) ?? [];
  const allowedFiles = new Set(['package.json', 'catalog.json', 'README.md', 'LICENSE']);
  const allowedPrefixes = ['.claude-plugin/', '.codex-plugin/', 'dist/', 'skills/', 'assets/', 'schemas/'];
  const requiredFiles = ['.claude-plugin/plugin.json', '.codex-plugin/plugin.json'];
  const unexpected = files.filter((path) => !allowedFiles.has(path) && !allowedPrefixes.some((prefix) => path.startsWith(prefix)));
  const missing = requiredFiles.filter((path) => !files.includes(path));
  const forbidden = files.filter((path) =>
    /^(?:\.local|openspec|node_modules|src|scripts)(?:\/|$)|(?:^|\/)__pycache__(?:\/|$)|(?:\.pyc|\.log)$|(?:^|\/)\.env(?:\.|$)|backup/i.test(path),
  );
  if (files.length === 0 || unexpected.length > 0 || missing.length > 0 || forbidden.length > 0) {
    const errors = [
      ...(files.length === 0 ? ['Package dry run returned no files.'] : []),
      ...unexpected.map((path) => `Unexpected package file: ${path}`),
      ...missing.map((path) => `Missing required package file: ${path}`),
      ...forbidden.map((path) => `Forbidden package file: ${path}`),
    ];
    throw new Error(errors.join('\n'));
  }

  smokeRoot = await mkdtemp(join(tmpdir(), 'arcantry-package-smoke-'));
  const installRoot = join(smokeRoot, 'install');
  const repositoryRoot = join(smokeRoot, 'repository');
  const wildRoot = join(smokeRoot, 'wild');
  const linkRoot = join(smokeRoot, 'links');
  await Promise.all([mkdir(installRoot), mkdir(repositoryRoot), mkdir(wildRoot), mkdir(linkRoot)]);
  if (retainedOutput) await mkdir(retainedOutput, { recursive: true });

  const packDestination = smokeRoot;
  const packed = await execa(nubCommand, ['pack', '--json', '--ignore-scripts', '--pack-destination', packDestination], { cwd: packageRoot });
  const packedOutput = JSON.parse(packed.stdout);
  const filename = packedOutput[0]?.filename;
  if (typeof filename !== 'string') throw new Error('Package smoke could not resolve the packed archive.');
  const archiveName = basename(filename);

  const packageManifest = JSON.parse(await readFile(join(packageRoot, 'package.json'), 'utf8'));
  const packagePath = packageManifest.name.split('/');
  const archivePath = isAbsolute(filename) ? filename : join(packDestination, filename);
  await writeFile(
    join(installRoot, 'package.json'),
    `${JSON.stringify(
      {
        name: 'arcantry-package-smoke',
        private: true,
        dependencies: {
          [packageManifest.name]: `file:../${archiveName}`,
        },
      },
      undefined,
      2,
    )}\n`,
    'utf8',
  );
  await execa(nubCommand, ['install', '--ignore-scripts'], { cwd: installRoot });
  const installedPackageRoot = join(installRoot, 'node_modules', ...packagePath);
  const agentManifests = await Promise.all([
    readFile(join(installedPackageRoot, '.codex-plugin', 'plugin.json'), 'utf8').then(JSON.parse),
    readFile(join(installedPackageRoot, '.claude-plugin', 'plugin.json'), 'utf8').then(JSON.parse),
  ]);
  for (const manifest of agentManifests) {
    if (manifest.name !== packageManifest.name || manifest.version !== packageManifest.version) {
      throw new Error(`Agent manifest ${manifest.name ?? 'unknown'}@${manifest.version ?? 'unknown'} does not match package ${packageManifest.name}@${packageManifest.version}.`);
    }
  }
  const cli = join(installedPackageRoot, 'dist', 'cli.js');
  const runCli = (args, cwd = smokeRoot) => execa(process.execPath, [cli, ...args], { cwd });

  await execa('git', ['init', '--quiet'], { cwd: repositoryRoot });
  const versionResult = await runCli(['--version']);
  if (versionResult.stdout.trim() !== packageManifest.version) {
    throw new Error(`Packed CLI version ${versionResult.stdout.trim()} does not match package ${packageManifest.version}.`);
  }
  const subpaths = ['project', 'release'].map((subpath) => `${packageManifest.name}/${subpath}`);
  await execa(process.execPath, ['--input-type=module', '-e', `await Promise.all(${JSON.stringify(subpaths)}.map((specifier) => import(specifier)));`], { cwd: installRoot });
  await runCli(['--cwd', wildRoot, 'repo', 'inspect', '--json']);
  await runCli(['--cwd', wildRoot, 'todo', 'add', 'Package smoke +Arcantry', '--source', 'root', '--apply']);
  const transition = await runCli([
    '--cwd', wildRoot,
    'repo', 'plan',
    '--source', 'todo-root',
    '--transition', 'relocate',
    '--to-path', '.local/todo.txt',
    '--delete-source',
    '--json',
  ]);
  const planPath = join(smokeRoot, 'plan.json');
  await writeFile(planPath, transition.stdout, 'utf8');
  await runCli(['--cwd', wildRoot, 'repo', 'apply', '--plan', planPath]);
  if ((await readFile(join(wildRoot, '.local', 'todo.txt'), 'utf8')) !== 'Package smoke +Arcantry\n') {
    throw new Error('Packed CLI did not preserve todo.txt content during relocation.');
  }
  await runCli(['--cwd', repositoryRoot, 'repo', 'init', '--scope', 'private']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'update', '--scope', 'private']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'doctor']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'validate']);
  await runCli(['--cwd', repositoryRoot, 'repo', 'remove', '--scope', 'private']);
  await runCli(['skills', 'list']);
  await runCli(['skills', 'inspect', 'adopt-arcantry']);
  await runCli(['skills', 'link', 'adopt-arcantry', '--target', linkRoot]);
  await runCli(['skills', 'doctor', '--target', linkRoot]);
  await runCli(['skills', 'unlink', 'adopt-arcantry', '--target', linkRoot]);
  await runCli(['--cwd', repositoryRoot, 'skills', 'link', 'adopt-arcantry', '--scope', 'repo', '--compat', 'claude']);
  await runCli(['--cwd', repositoryRoot, 'skills', 'doctor', '--scope', 'repo', '--compat', 'claude']);
  await runCli(['--cwd', repositoryRoot, 'skills', 'unlink', 'adopt-arcantry', '--scope', 'repo', '--compat', 'claude']);

  const verifiedArchivePath = retainedOutput === undefined ? archivePath : join(retainedOutput, archiveName);
  if (retainedOutput !== undefined) await copyFile(archivePath, verifiedArchivePath);
  process.stdout.write(`Package allowlist and packed CLI smoke passed (${files.length} files): ${verifiedArchivePath}\n`);
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
} finally {
  if (smokeRoot !== undefined) await rm(smokeRoot, { force: true, recursive: true, maxRetries: 3 });
}
