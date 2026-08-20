import { mkdir, mkdtemp, readdir, readFile, rm, writeFile } from 'node:fs/promises';
import { createServer } from 'node:net';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { Command } from 'commander';
import { execa } from 'execa';
import { nativeTargets } from './native-targets.js';

type RegistryOptions = {
  archives: string;
};

const { NODE_OPTIONS: _nodeOptions, ...cleanEnvironment } = process.env;

const parseOptions = (): RegistryOptions => {
  const program = new Command()
    .name('registry-smoke.ts')
    .description('Smoke-test all npm packages through a local registry.')
    .requiredOption('--archives <directory>', 'Directory containing all seven npm archives.')
    .showHelpAfterError();
  program.parse();
  return program.opts<RegistryOptions>();
};

const run = async (command: string, args: string[], cwd: string): Promise<string> => {
  const { stdout } = await execa(command, args, {
    cwd,
    env: cleanEnvironment,
    extendEnv: false,
    stdin: 'ignore',
  });
  return stdout;
};

const availablePort = async (): Promise<number> =>
  await new Promise((resolvePort, reject) => {
    const server = createServer();
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      if (address === null || typeof address === 'string') {
        server.close();
        reject(new Error('Could not allocate a local registry port.'));
        return;
      }
      server.close((error) => (error === undefined ? resolvePort(address.port) : reject(error)));
    });
  });

const delay = async (milliseconds: number): Promise<void> =>
  await new Promise((resolveDelay) => setTimeout(resolveDelay, milliseconds));

const waitForRegistry = async (url: string, errors: () => string): Promise<void> => {
  for (let attempt = 0; attempt <= 80; attempt += 1) {
    try {
      const response = await fetch(`${url}/-/ping`);
      if (response.ok) return;
    } catch {
      // The registry may still be binding its local port.
    }
    if (attempt === 80) throw new Error(`Local registry did not start.\n${errors()}`);
    await delay(100);
  }
};

const startRegistry = (configPath: string, port: number) =>
  execa('nub', ['exec', 'verdaccio', '--config', configPath, '--listen', `127.0.0.1:${port}`], {
    cwd: process.cwd(),
    env: cleanEnvironment,
    extendEnv: false,
    stdin: 'ignore',
    stdout: 'pipe',
    stderr: 'pipe',
    forceKillAfterDelay: 1_000,
  });

const main = async (): Promise<void> => {
  const archivesRoot = resolve(parseOptions().archives);
  const temporary = await mkdtemp(join(tmpdir(), 'arcantry-registry-smoke-'));
  let registry: ReturnType<typeof startRegistry> | undefined;

  try {
    const port = await availablePort();
    const registryUrl = `http://127.0.0.1:${port}`;
    const configPath = join(temporary, 'verdaccio.yaml');
    await writeFile(
      configPath,
      `storage: ${JSON.stringify(join(temporary, 'storage'))}
auth:
  htpasswd:
    file: ${JSON.stringify(join(temporary, 'htpasswd'))}
    max_users: 1000
uplinks:
  npmjs:
    url: https://registry.npmjs.org/
packages:
  '@*/*':
    access: $all
    publish: $all
    unpublish: $all
    proxy: npmjs
  '**':
    access: $all
    publish: $all
    unpublish: $all
    proxy: npmjs
log:
  type: stdout
  format: pretty
  level: warn
`,
      'utf8',
    );
    registry = startRegistry(configPath, port);
    let registryErrors = '';
    registry.stderr?.on('data', (chunk) => {
      registryErrors += String(chunk);
    });
    void registry.catch(() => undefined);
    await waitForRegistry(registryUrl, () => registryErrors);

    const userResponse = await fetch(`${registryUrl}/-/user/org.couchdb.user:arcantry-smoke`, {
      method: 'PUT',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        name: 'arcantry-smoke',
        password: 'arcantry-local-registry-smoke',
        email: 'local-smoke@arcantry.invalid',
        type: 'user',
        roles: [],
      }),
    });
    if (!userResponse.ok) throw new Error(`Local registry user creation failed: ${await userResponse.text()}`);
    const user = (await userResponse.json()) as { token?: string };
    if (user.token === undefined) throw new Error('Local registry returned no authentication token.');
    const userConfig = join(temporary, 'npmrc');
    await writeFile(userConfig, `registry=${registryUrl}\n//127.0.0.1:${port}/:_authToken=${user.token}\n`, 'utf8');
    cleanEnvironment.NPM_CONFIG_USERCONFIG = userConfig;

    const archives = (await readdir(archivesRoot)).filter((name) => name.endsWith('.tgz')).sort();
    if (archives.length !== 7) throw new Error(`Expected seven npm archives, received ${archives.length}.`);
    for (const [index, archive] of archives.entries()) {
      const extraction = join(temporary, 'publish', String(index));
      await mkdir(extraction, { recursive: true });
      await run('tar', ['-xzf', join(archivesRoot, archive), '-C', extraction], temporary);
      await run(
        'nub',
        ['publish', '--registry', registryUrl, '--ignore-scripts', '--no-git-checks', '--access', 'public'],
        join(extraction, 'package'),
      );
    }

    for (const { os, cpu, packageName: platformPackage } of nativeTargets) {
      const install = join(temporary, 'install', `${os}-${cpu}`);
      await mkdir(install, { recursive: true });
      await writeFile(
        join(install, 'package.json'),
        `${JSON.stringify(
          {
            name: `arcantry-registry-${os}-${cpu}`,
            private: true,
            dependencies: { arcantry: '1.0.0', [platformPackage]: '1.0.0' },
          },
          undefined,
          2,
        )}\n`,
        'utf8',
      );
      await run(
        'nub',
        [
          'install',
          '--registry',
          registryUrl,
          '--ignore-scripts',
          '--minimum-release-age',
          '0',
          '--os',
          os,
          '--cpu',
          cpu,
        ],
        install,
      );
      await readFile(join(install, 'node_modules', ...platformPackage.split('/'), 'package.json'), 'utf8');
      await run(
        'nub',
        [
          '--node',
          '-e',
          "await Promise.all(['arcantry','arcantry/catalog','arcantry/repository','arcantry/project','arcantry/release'].map((value) => import(value)))",
        ],
        install,
      );
      if (os === process.platform && cpu === process.arch) {
        const runners: Array<[string, string[]]> = [
          ['npm', ['exec', '--offline', '--', 'arcantry', '--version']],
          ['npx', ['--offline', 'arcantry', '--version']],
          ['pnpm', ['exec', 'arcantry', '--version']],
          ['bun', ['run', 'arcantry', '--version']],
          ['nub', ['exec', 'arcantry', '--version']],
        ];
        for (const [runner, args] of runners) {
          const version = (await run(runner, args, install)).trim();
          if (version !== '1.0.0') {
            throw new Error(`${runner} reported ${version}, expected 1.0.0 from the registry-installed CLI.`);
          }
        }
      }
    }
    process.stdout.write('Local registry smoke passed for all six npm platform packages.\n');
  } finally {
    if (registry) {
      if (process.platform === 'win32' && registry.pid !== undefined) {
        await execa('taskkill.exe', ['/pid', String(registry.pid), '/t', '/f'], { reject: false, windowsHide: true });
      } else {
        registry.kill('SIGTERM');
      }
      await registry.catch(() => undefined);
    }
    await rm(temporary, { force: true, recursive: true, maxRetries: 3 });
  }
};

await main();
