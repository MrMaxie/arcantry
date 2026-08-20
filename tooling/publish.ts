import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { Command } from 'commander';
import { execa } from 'execa';
import { validateNpmPublication } from '../packages/arcantry/src/release.js';
import { nativeTargets, npmPackageManifests } from './native-targets.js';

export * from '../packages/arcantry/src/release.js';

type PackageManifest = {
  name: string;
  version: string;
};

type TagOptions = {
  tag: string;
};

type ArchiveOptions = {
  archives: string;
  output: string;
};

type PublishOptions = {
  archives: string;
  existing: string;
};

type ReleaseOptions = {
  tag: string;
  artifacts: string;
};

const { NODE_OPTIONS: _nodeOptions, ...cleanEnvironment } = process.env;

const run = async (command: string, args: string[]): Promise<string> => {
  const { stdout } = await execa(command, args, {
    env: cleanEnvironment,
    extendEnv: false,
    stdin: 'ignore',
  });
  return stdout;
};

const manifests = (): PackageManifest[] =>
  npmPackageManifests.map((path) => JSON.parse(readFileSync(path, 'utf8')) as PackageManifest);

const archiveName = (metadata: PackageManifest): string =>
  `${metadata.name.replace(/^@/, '').replace('/', '-')}-${metadata.version}.tgz`;

const verifyNpmCli = async (): Promise<void> => {
  const [major = 0, minor = 0] = (await run('npm', ['--version'])).trim().split('.').map(Number);
  if (major < 11 || (major === 11 && minor < 5)) {
    throw new Error('npm 11.5.1 or newer is required.');
  }
};

const preflightNpm = async ({ archives, output }: ArchiveOptions): Promise<void> => {
  const archivesRoot = resolve(archives);
  const existing: string[] = [];

  for (const metadata of manifests()) {
    const name = archiveName(metadata);
    const archive = join(archivesRoot, name);
    const expected = `sha512-${createHash('sha512').update(readFileSync(archive)).digest('base64')}`;
    const versionResult = await execa(
      'npm',
      ['view', `${metadata.name}@${metadata.version}`, 'dist.integrity', '--json'],
      {
        env: cleanEnvironment,
        extendEnv: false,
        reject: false,
      },
    );
    if (versionResult.exitCode === 0) {
      if (metadata.name === 'arcantry') {
        throw new Error(
          `${metadata.name}@${metadata.version} already exists; refusing duplicate main-package publication.`,
        );
      }
      const actual = JSON.parse(versionResult.stdout) as string;
      if (actual !== expected) {
        throw new Error(`${metadata.name}@${metadata.version} exists with different integrity.`);
      }
      existing.push(name);
      continue;
    }
    if (!versionResult.stderr.includes('E404')) {
      throw new Error(versionResult.stderr || `npm view failed for ${metadata.name}@${metadata.version}.`);
    }

    const packageResult = await execa('npm', ['view', metadata.name, 'name', '--json'], {
      env: cleanEnvironment,
      extendEnv: false,
      reject: false,
    });
    if (packageResult.exitCode !== 0) {
      if (packageResult.stderr.includes('E404')) {
        throw new Error(`${metadata.name} requires the separately authorized 2FA bootstrap publication.`);
      }
      throw new Error(packageResult.stderr || `npm view failed for ${metadata.name}.`);
    }
  }

  writeFileSync(resolve(output), `${existing.sort().join('\n')}${existing.length === 0 ? '' : '\n'}`);
};

const publishNpm = async ({ archives, existing }: PublishOptions): Promise<void> => {
  const archivesRoot = resolve(archives);
  const existingArchives = new Set(readFileSync(resolve(existing), 'utf8').split(/\r?\n/u).filter(Boolean));
  for (const metadata of manifests()) {
    const name = archiveName(metadata);
    if (existingArchives.has(name)) {
      process.stdout.write(`Already verified on npm: ${name}\n`);
    } else {
      await run('npm', ['publish', join(archivesRoot, name)]);
    }
  }
};

const createDraftRelease = async ({ tag, artifacts }: ReleaseOptions): Promise<void> => {
  const artifactRoot = resolve(artifacts);
  const releaseArtifacts = [
    ...nativeTargets.map((target) => join(artifactRoot, target.archive)),
    join(artifactRoot, 'SHA256SUMS'),
    join(artifactRoot, 'arcantry-installer.sh'),
    join(artifactRoot, 'arcantry-installer.ps1'),
  ];
  const existing = await execa('gh', ['release', 'view', tag, '--json', 'isDraft,name,tagName'], {
    env: cleanEnvironment,
    extendEnv: false,
    reject: false,
    stdin: 'ignore',
  });
  if (existing.exitCode === 0) {
    const release = JSON.parse(existing.stdout) as { isDraft: boolean; name: string; tagName: string };
    if (!release.isDraft) throw new Error(`GitHub Release ${tag} is already public.`);
    if (release.tagName !== tag || release.name !== tag) {
      throw new Error(`Existing draft ${tag} does not match the verified release identity.`);
    }
    await run('gh', ['release', 'upload', tag, ...releaseArtifacts, '--clobber']);
    return;
  }
  if (!`${existing.stdout}\n${existing.stderr}`.toLowerCase().includes('release not found')) {
    throw new Error(existing.stderr || `Unable to inspect GitHub Release ${tag}.`);
  }
  await run('gh', ['release', 'create', tag, '--draft', '--verify-tag', '--title', tag, ...releaseArtifacts]);
};

const publishRelease = async ({ tag }: TagOptions): Promise<void> => {
  await run('gh', ['release', 'edit', tag, '--draft=false']);
};

export const createPublishProgram = (): Command => {
  const program = new Command()
    .name('publish.ts')
    .description('Validate and execute the reviewed Arcantry publication sequence.')
    .showHelpAfterError();

  program
    .command('check')
    .description('Verify sealed release and package identity.')
    .requiredOption('--tag <tag>', 'Version tag to verify.')
    .action((options: TagOptions) => {
      process.stdout.write(`${JSON.stringify(validateNpmPublication(options.tag), null, 2)}\n`);
    });

  program
    .command('verify-npm-cli')
    .description('Require an npm CLI version that supports trusted publishing.')
    .action(verifyNpmCli);

  program
    .command('preflight-npm')
    .description('Verify retry-safe npm publication state.')
    .requiredOption('--archives <directory>', 'Verified npm package archives.')
    .requiredOption('--output <path>', 'Existing-package manifest path.')
    .action(preflightNpm);

  program
    .command('publish-npm')
    .description('Publish missing platform packages followed by the main package.')
    .requiredOption('--archives <directory>', 'Verified npm package archives.')
    .requiredOption('--existing <path>', 'Preflight existing-package manifest.')
    .action(publishNpm);

  program
    .command('create-draft-release')
    .description('Create the draft GitHub Release from verified artifacts.')
    .requiredOption('--tag <tag>', 'Version tag to publish.')
    .requiredOption('--artifacts <directory>', 'Verified native release artifacts.')
    .action(createDraftRelease);

  program
    .command('publish-release')
    .description('Make the draft GitHub Release public.')
    .requiredOption('--tag <tag>', 'Version tag to publish.')
    .action(publishRelease);

  return program;
};

const main = async (): Promise<void> => {
  await createPublishProgram().parseAsync();
};

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) await main();
