import { createHash } from 'node:crypto';
import { copyFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, join, resolve } from 'node:path';
import { Command, Option } from 'commander';
import { execa } from 'execa';
import { downloadedBinaryPath, nativeTarget, nativeTargets } from './native-targets.js';

type PackageOptions = {
  output: string;
  main?: boolean;
  artifacts?: string;
  binary: string[];
};

type PackedFile = {
  path: string;
};

type PackResult = {
  filename?: string;
  files?: PackedFile[];
};

const root = process.cwd();
const nub = 'nub';
const { NODE_OPTIONS: _nodeOptions, ...cleanEnvironment } = process.env;

const collect = (value: string, previous: string[]): string[] => [...previous, value];

const parseOptions = (): PackageOptions => {
  const program = new Command()
    .name('package-native.ts')
    .description('Build the reviewed Arcantry npm package set.')
    .requiredOption('--output <directory>', 'Directory for package archives.')
    .option('--main', 'Include the main arcantry package.')
    .addOption(new Option('--artifacts <directory>', 'Downloaded native artifact root.').conflicts('binary'))
    .addOption(
      new Option('--binary <target=path>', 'Native target and explicit binary path.')
        .argParser(collect)
        .default([])
        .conflicts('artifacts'),
    )
    .showHelpAfterError();
  program.parse();
  return program.opts<PackageOptions>();
};

const runNubPack = async (packageRoot: string, outputDirectory: string): Promise<string> => {
  const { stdout } = await execa(nub, ['pack', '--json', '--ignore-scripts', '--pack-destination', outputDirectory], {
    cwd: packageRoot,
    env: cleanEnvironment,
    extendEnv: false,
  });
  const output = JSON.parse(stdout) as PackResult[];
  if (typeof output[0]?.filename !== 'string') throw new Error(`nub pack returned no archive for ${packageRoot}.`);
  return basename(output[0].filename);
};

const dryRunFiles = async (packageRoot: string): Promise<string[]> => {
  const { stdout } = await execa(nub, ['pack', '--dry-run', '--json', '--ignore-scripts'], {
    cwd: packageRoot,
    env: cleanEnvironment,
    extendEnv: false,
  });
  const output = JSON.parse(stdout) as PackResult[];
  return (output[0]?.files ?? []).map((entry) => entry.path);
};

const assertPlatformPackage = async (packageRoot: string, executable: string): Promise<void> => {
  const actual = (await dryRunFiles(packageRoot)).sort();
  const expected = ['LICENSE', 'README.md', `bin/${executable}`, 'package.json'].sort();
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    throw new Error(
      `Platform package allowlist mismatch. Expected ${expected.join(', ')}, received ${actual.join(', ')}.`,
    );
  }
};

const assertMainPackage = async (packageRoot: string): Promise<void> => {
  const files = await dryRunFiles(packageRoot);
  const allowedFiles = new Set(['package.json', 'catalog.json', 'README.md', 'LICENSE']);
  const allowedPrefixes = ['.claude-plugin/', '.codex-plugin/', 'dist/', 'skills/', 'assets/', 'schemas/'];
  const requiredFiles = ['.claude-plugin/plugin.json', '.codex-plugin/plugin.json', 'dist/cli.js'];
  const unexpected = files.filter(
    (path) => !allowedFiles.has(path) && !allowedPrefixes.some((prefix) => path.startsWith(prefix)),
  );
  const missing = requiredFiles.filter((path) => !files.includes(path));
  const forbidden = files.filter((path) =>
    /^(?:\.local|openspec|node_modules|src|scripts)(?:\/|$)|(?:^|\/)__pycache__(?:\/|$)|(?:\.pyc|\.log)$|(?:^|\/)\.env(?:\.|$)|backup/i.test(
      path,
    ),
  );
  if (files.length === 0 || unexpected.length > 0 || missing.length > 0 || forbidden.length > 0) {
    throw new Error(
      [
        ...(files.length === 0 ? ['Main package dry run returned no files.'] : []),
        ...unexpected.map((path) => `Unexpected main package file: ${path}`),
        ...missing.map((path) => `Missing required main package file: ${path}`),
        ...forbidden.map((path) => `Forbidden main package file: ${path}`),
      ].join('\n'),
    );
  }
};

const main = async (): Promise<void> => {
  const options = parseOptions();
  const outputDirectory = resolve(options.output);
  const artifactRoot = options.artifacts ? resolve(options.artifacts) : undefined;
  const binaries = artifactRoot
    ? nativeTargets.map((target) => ({
        target,
        path: downloadedBinaryPath(artifactRoot, target),
      }))
    : options.binary.map((binary) => {
        const separator = binary.indexOf('=');
        if (separator <= 0 || separator === binary.length - 1) {
          throw new Error(`Invalid --binary value ${binary}. Expected <target>=<path>.`);
        }
        const triple = binary.slice(0, separator);
        const path = binary.slice(separator + 1);
        return { target: nativeTarget(triple), path: resolve(path) };
      });
  if (binaries.length === 0 && !options.main) {
    throw new Error('At least one --binary or --main input is required.');
  }

  await mkdir(outputDirectory, { recursive: true });
  const temporaryRoot = await mkdtemp(join(tmpdir(), 'arcantry-native-pack-'));
  const archives: string[] = [];

  try {
    for (const { target, path: binaryPath } of binaries) {
      const sourceRoot = join(root, 'packages', target.packageDirectory);
      const stageRoot = join(temporaryRoot, target.packageDirectory);
      await mkdir(join(stageRoot, 'bin'), { recursive: true });
      await Promise.all([
        copyFile(join(sourceRoot, 'package.json'), join(stageRoot, 'package.json')),
        copyFile(join(sourceRoot, 'README.md'), join(stageRoot, 'README.md')),
        copyFile(join(root, 'LICENSE'), join(stageRoot, 'LICENSE')),
        copyFile(binaryPath, join(stageRoot, 'bin', target.executable)),
      ]);
      await assertPlatformPackage(stageRoot, target.executable);
      archives.push(await runNubPack(stageRoot, outputDirectory));
    }

    if (options.main) {
      const mainRoot = join(root, 'packages', 'arcantry');
      await assertMainPackage(mainRoot);
      archives.push(await runNubPack(mainRoot, outputDirectory));
    }

    const manifest = [];
    for (const archive of archives.sort()) {
      const bytes = await readFile(join(outputDirectory, archive));
      manifest.push({ file: archive, sha256: createHash('sha256').update(bytes).digest('hex') });
    }
    await writeFile(join(outputDirectory, 'native-packages.json'), `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
    process.stdout.write(`Packed ${archives.length} npm archive(s) into ${outputDirectory}.\n`);
  } finally {
    await rm(temporaryRoot, { force: true, recursive: true, maxRetries: 3 });
  }
};

await main();
