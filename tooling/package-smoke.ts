import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { Command, Option } from 'commander';
import { execa } from 'execa';
import { builtBinaryPath, nativeTarget, nativeTargets } from './native-targets.js';

type SmokeOptions = {
  binary?: string;
  target?: string;
  buildRoot?: string;
  output?: string;
};

const root = process.cwd();
const nub = 'nub';
const { NODE_OPTIONS: _nodeOptions, ...cleanEnvironment } = process.env;

const parseOptions = (): SmokeOptions => {
  const program = new Command()
    .name('package-smoke.ts')
    .description('Smoke-test the npm launcher and one native platform package.')
    .addOption(new Option('--binary <path>', 'Explicit host binary path.').conflicts('target'))
    .addOption(new Option('--target <triple>', 'Host target built by cargo-dist.').conflicts('binary'))
    .option('--build-root <directory>', 'cargo-dist target root.')
    .option('--output <directory>', 'Retain package archives in this directory.')
    .showHelpAfterError();
  program.parse();
  const options = program.opts<SmokeOptions>();
  if (!options.binary && !options.target) program.error('one of --binary or --target is required');
  if (options.buildRoot && !options.target) program.error('--build-root requires --target');
  return options;
};

const run = async (command: string, args: string[], cwd: string): Promise<string> => {
  const { stdout } = await execa(command, args, {
    cwd,
    env: cleanEnvironment,
    extendEnv: false,
  });
  return stdout;
};

const main = async (): Promise<void> => {
  const options = parseOptions();
  const hostTarget = nativeTargets.find(
    (candidate) => candidate.os === process.platform && candidate.cpu === process.arch,
  );
  if (!hostTarget) throw new Error(`Unsupported package smoke host: ${process.platform}-${process.arch}.`);
  const target = options.target ? nativeTarget(options.target) : hostTarget;
  if (target.triple !== hostTarget.triple) {
    throw new Error(`Package smoke target ${target.triple} does not match host ${hostTarget.triple}.`);
  }
  const binaryPath = options.binary
    ? resolve(options.binary)
    : builtBinaryPath(resolve(options.buildRoot ?? 'target'), target);
  const retainedOutput = options.output ? resolve(options.output) : undefined;
  const temporaryRoot = await mkdtemp(join(tmpdir(), 'arcantry-package-smoke-'));
  const archiveRoot = retainedOutput ?? join(temporaryRoot, 'archives');
  const installRoot = join(temporaryRoot, 'install');

  try {
    await Promise.all([mkdir(archiveRoot, { recursive: true }), mkdir(installRoot, { recursive: true })]);
    await run(
      nub,
      ['tooling/package-native.ts', '--output', archiveRoot, '--main', '--binary', `${target.triple}=${binaryPath}`],
      root,
    );

    const manifest = JSON.parse(await readFile(join(archiveRoot, 'native-packages.json'), 'utf8')) as Array<{
      file: string;
    }>;
    const mainArchive = manifest.find((entry) => entry.file.startsWith('arcantry-1.0.0'))?.file;
    const platformArchive = manifest.find((entry) => entry.file !== mainArchive)?.file;
    if (mainArchive === undefined || platformArchive === undefined) {
      throw new Error('Native npm package set is incomplete.');
    }

    await writeFile(
      join(installRoot, 'package.json'),
      `${JSON.stringify(
        {
          name: 'arcantry-native-package-smoke',
          private: true,
          dependencies: {
            arcantry: `file:${join(archiveRoot, mainArchive)}`,
            [target.packageName]: `file:${join(archiveRoot, platformArchive)}`,
          },
        },
        undefined,
        2,
      )}\n`,
      'utf8',
    );
    await run(nub, ['install', '--ignore-scripts'], installRoot);

    const packageRoot = join(installRoot, 'node_modules', 'arcantry');
    const cli = join(packageRoot, 'dist', 'cli.js');
    const version = (await run(process.execPath, [cli, '--version'], installRoot)).trim();
    if (version !== '1.0.0') throw new Error(`Packed native CLI reported ${version}, expected 1.0.0.`);

    const subpaths = ['.', 'catalog', 'repository', 'project', 'release'].map((subpath) =>
      subpath === '.' ? 'arcantry' : `arcantry/${subpath}`,
    );
    await run(
      process.execPath,
      [
        '--input-type=module',
        '-e',
        `await Promise.all(${JSON.stringify(subpaths)}.map((specifier) => import(specifier)));`,
      ],
      installRoot,
    );
    await run(process.execPath, [cli, '--cwd', installRoot, 'repo', 'inspect', '--json'], installRoot);
    process.stdout.write(`Native npm package smoke passed for ${target.triple}.\n`);
  } finally {
    await rm(temporaryRoot, { force: true, recursive: true, maxRetries: 3 });
  }
};

await main();
