import { lstat, mkdir, mkdtemp, readFile, readdir, readlink, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, relative } from 'node:path';
import { execa } from 'execa';
import { afterEach, describe, expect, it } from 'vitest';
import { runCli } from './program.js';
import { createFixtureRepository } from './testHelpers.js';

const nativeBinary = process.env.ARCANTRY_NATIVE_BIN;
const roots: string[] = [];

afterEach(async () => {
  await Promise.all(roots.splice(0).map((root) => rm(root, { recursive: true, force: true, maxRetries: 3 })));
});

const runTypeScript = async (cwd: string, args: string[]) => {
  let stdout = '';
  let stderr = '';
  const result = await runCli(['node', 'arcantry', ...args], {
    cwd,
    stdout: (message) => { stdout += message; },
    stderr: (message) => { stderr += message; },
  });
  return { stdout, stderr, exitCode: result.exitCode };
};

const runRust = async (cwd: string, args: string[], input?: string) => {
  const result = await execa(nativeBinary!, args, { cwd, input, reject: false });
  return { stdout: result.stdout.length === 0 ? '' : `${result.stdout}\n`, stderr: result.stderr.length === 0 ? '' : `${result.stderr}\n`, exitCode: result.exitCode ?? 1 };
};

const runTypeScriptProcess = async (cwd: string, args: string[], input?: string) => {
  const result = await execa('nub', [join(process.cwd(), 'src', 'legacy-cli.ts'), ...args], {
    cwd,
    input,
    reject: false,
  });
  return {
    stdout: result.stdout.length === 0 ? '' : `${result.stdout}\n`,
    stderr: result.stderr.length === 0 ? '' : `${result.stderr}\n`,
    exitCode: result.exitCode ?? 1,
  };
};

const files = async (root: string): Promise<Record<string, string>> => {
  const result: Record<string, string> = {};
  const visit = async (directory: string): Promise<void> => {
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      const name = relative(root, path).replaceAll('\\', '/');
      if (name === '.git' || (name.startsWith('.git/') && name !== '.git/info' && name !== '.git/info/exclude')) continue;
      const stats = await lstat(path);
      if (stats.isSymbolicLink()) {
        const target = await readlink(path);
        const normalized = relative(root, target);
        result[name] = `link:${normalized.startsWith('..') ? target : `<root>/${normalized.replaceAll('\\', '/')}`}`;
      }
      else if (stats.isDirectory()) await visit(path);
      else result[name] = await readFile(path, 'utf8');
    }
  };
  await visit(root);
  return result;
};

const pair = async () => {
  const left = await createFixtureRepository();
  const right = await createFixtureRepository();
  roots.push(left, right);
  return { left, right };
};

const releasePair = async () => {
  const create = async () => {
    const root = await mkdtemp(join(tmpdir(), 'arcantry-native-release-'));
    roots.push(root);
    await mkdir(join(root, 'openspec', 'changes', 'archive'), { recursive: true });
    await writeFile(join(root, 'package.json'), '{\r\n    "z": true,\r\n    "version": "0.8.0",\r\n    "name": "fixture"\r\n}\r\n');
    await writeFile(
      join(root, 'arcantry.toml'),
      `config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "changelog"
tag_prefix = "v"
repository_url = "https://github.com/example/project"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`,
    );
    return root;
  };
  return { left: await create(), right: await create() };
};

describe.runIf(nativeBinary !== undefined)('native CLI conformance', () => {
  it('matches missing and invalid arguments', async () => {
    const commands = [
      [],
      ['unknown'],
      ['--cwd'],
      ['repo'],
      ['repo', 'unknown'],
      ['repo', 'inspect', '--unknown'],
      ['repo', 'inspect', 'extra'],
      ['--cwd', process.cwd(), 'repo', 'init'],
      ['repo', 'init', '--scope'],
      ['repo', 'plan'],
      ['repo', 'plan', '--source', 'todo-root'],
      ['repo', 'apply'],
      ['repo', 'remove', '--scope', 'invalid'],
      ['todo', 'add'],
      ['todo', 'move', '1', '--from', 'root'],
      ['release', 'baseline'],
      ['release', 'baseline', '--date', '2026-08-20'],
      ['skills', 'inspect'],
      ['skills', 'list', '--scope', 'invalid'],
      ['--unknown'],
    ];
    for (const args of commands) {
      expect.soft(await runRust(process.cwd(), args), args.join(' ') || '<root>').toEqual(
        await runTypeScript(process.cwd(), args),
      );
    }
  });

  it('matches help for the complete command hierarchy', async () => {
    const commands = [
      [],
      ['repo'],
      ['todo'],
      ['release'],
      ['skills'],
      ...[
        'repo inspect',
        'repo plan',
        'repo apply',
        'repo init',
        'repo update',
        'repo doctor',
        'repo validate',
        'repo remove',
        'todo list',
        'todo add',
        'todo complete',
        'todo move',
        'release baseline',
        'release plan',
        'release cut',
        'release render',
        'release check',
        'skills list',
        'skills inspect',
        'skills link',
        'skills unlink',
        'skills doctor',
      ].map((command) => command.split(' ')),
    ];
    for (const command of commands) {
      const args = [...command, '--help'];
      expect(await runRust(process.cwd(), args), args.join(' ')).toEqual(await runTypeScript(process.cwd(), args));
    }
  });

  it('matches repository inspection JSON exactly', async () => {
    const { left } = await pair();
    expect(await runRust(left, ['repo', 'inspect', '--json'])).toEqual(await runTypeScript(left, ['repo', 'inspect', '--json']));
  });

  it('matches project discovery from an implicit nested cwd', async () => {
    const { left, right } = await pair();
    const config = `config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "todo.txt"
adapter = "todo-txt@1"
`;
    const leftNested = join(left, 'nested');
    const rightNested = join(right, 'nested');
    await Promise.all([
      mkdir(leftNested),
      mkdir(rightNested),
      writeFile(join(left, 'arcantry.toml'), config),
      writeFile(join(right, 'arcantry.toml'), config),
    ]);

    const args = ['todo', 'add', 'Nested task', '--source', 'tasks', '--apply'];
    expect(await runRust(rightNested, args)).toEqual(await runTypeScript(leftNested, args));
    expect(await files(right)).toEqual(await files(left));
    expect(await readFile(join(right, 'todo.txt'), 'utf8')).toBe('Nested task\n');
  });

  it('matches private init, validate, update and remove', async () => {
    const { left, right } = await pair();
    for (const args of [
      ['repo', 'init', '--scope', 'private'],
      ['repo', 'validate'],
      ['repo', 'doctor'],
      ['repo', 'update', '--scope', 'private'],
      ['repo', 'remove', '--scope', 'private'],
    ]) {
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
      expect(await files(right)).toEqual(await files(left));
    }
  });

  it('matches todo additions, completion and cross-scope moves', async () => {
    const { left, right } = await pair();
    const legacy =
      '\uFEFF(B) 2026-08-17 Verify +Arcantry @desk\r\n(a) lowercase priority\r\n  spaced task  \r\nowner:one owner:two +P +P @C @C\r\n';
    await Promise.all([writeFile(join(left, 'todo.txt'), legacy), writeFile(join(right, 'todo.txt'), legacy)]);
    for (const args of [
      ['todo', 'add', '(A) 2026-08-20 Move me +project @desk', '--source', 'root', '--apply'],
      ['todo', 'complete', '1', '--source', 'root', '--date', '2026-08-20', '--apply'],
      ['todo', 'move', '1', '--from', 'root', '--to', 'local', '--apply'],
      ['todo', 'list'],
    ]) {
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
      expect(await files(right)).toEqual(await files(left));
    }
    expect(await readFile(join(right, 'todo.txt'), 'utf8')).toBe(
      '\uFEFF(a) lowercase priority\r\n  spaced task  \r\nowner:one owner:two +P +P @C @C\r\n(A) 2026-08-20 Move me +project @desk\r\n',
    );
  });

  it('matches public catalog listing and inspection', async () => {
    const root = await mkdtemp(join(tmpdir(), 'arcantry-native-catalog-'));
    roots.push(root);
    for (const args of [['skills', 'list'], ['skills', 'inspect', 'adopt-arcantry']]) {
      expect(await runRust(process.cwd(), args)).toEqual(await runTypeScript(process.cwd(), args));
    }
  });

  it('matches private skill inventory and repository links', async () => {
    const { left, right } = await pair();
    for (const root of [left, right]) {
      const skill = join(root, '.local', 'skills', 'private-helper');
      await mkdir(skill, { recursive: true });
      await writeFile(
        join(skill, 'SKILL.md'),
        '---\nname: private-helper\ndescription: Use one private repository workflow without publishing its local instructions.\n---\n',
      );
    }
    for (const args of [
      ['skills', 'list', '--scope', 'private'],
      ['skills', 'inspect', 'private-helper', '--scope', 'private'],
    ]) {
      expect(await runRust(left, args)).toEqual(await runTypeScript(left, args));
    }
    for (const args of [
      [
        'skills',
        'link',
        'private-helper',
        '--scope',
        'private',
        '--compat',
        'claude',
        '--catalog-root',
        process.cwd(),
      ],
      [
        'skills',
        'unlink',
        'private-helper',
        '--scope',
        'private',
        '--compat',
        'claude',
        '--catalog-root',
        process.cwd(),
      ],
    ]) {
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
      expect(await files(right)).toEqual(await files(left));
    }
  });

  it('matches source relocation plans and application', async () => {
    const { left, right } = await pair();
    await Promise.all([writeFile(join(left, 'todo.txt'), 'Move me @desk\n'), writeFile(join(right, 'todo.txt'), 'Move me @desk\n')]);
    const args = [
      'repo',
      'plan',
      '--source',
      'todo-root',
      '--transition',
      'relocate',
      '--to-path',
      '.local/todo.txt',
      '--delete-source',
      '--json',
    ];
    const expected = await runTypeScript(left, args);
    const actual = await runRust(right, args);
    expect({ stderr: actual.stderr, exitCode: actual.exitCode }).toEqual({
      stderr: expected.stderr,
      exitCode: expected.exitCode,
    });
    expect({ ...JSON.parse(actual.stdout), root: '<root>' }).toEqual({ ...JSON.parse(expected.stdout), root: '<root>' });
    await Promise.all([writeFile(join(left, 'plan.json'), expected.stdout), writeFile(join(right, 'plan.json'), actual.stdout)]);
    expect(await runRust(right, ['repo', 'apply', '--plan', 'plan.json'])).toEqual(
      await runTypeScript(left, ['repo', 'apply', '--plan', 'plan.json']),
    );
    await Promise.all([rm(join(left, 'plan.json')), rm(join(right, 'plan.json'))]);
    expect(await files(right)).toEqual(await files(left));
  });

  it('matches OpenSpec tree relocation plans and application', async () => {
    const { left, right } = await pair();
    for (const root of [left, right]) {
      await mkdir(join(root, 'openspec', 'specs', 'example'), { recursive: true });
      await writeFile(join(root, 'openspec', 'config.yaml'), 'schema: arcantry\n');
      await writeFile(join(root, 'openspec', 'specs', 'example', 'spec.md'), '# Example\n');
    }
    const args = [
      'repo',
      'plan',
      '--source',
      'openspec',
      '--transition',
      'relocate',
      '--to-path',
      '.local/openspec',
      '--delete-source',
      '--json',
    ];
    const expected = await runTypeScript(left, args);
    const actual = await runRust(right, args);
    expect({ ...JSON.parse(actual.stdout), root: '<root>' }).toEqual({ ...JSON.parse(expected.stdout), root: '<root>' });
    await Promise.all([writeFile(join(left, 'plan.json'), expected.stdout), writeFile(join(right, 'plan.json'), actual.stdout)]);
    expect(await runRust(right, ['repo', 'apply', '--plan', 'plan.json'])).toEqual(
      await runTypeScript(left, ['repo', 'apply', '--plan', 'plan.json']),
    );
    await Promise.all([rm(join(left, 'plan.json')), rm(join(right, 'plan.json'))]);
    expect(await files(right)).toEqual(await files(left));
  });

  it('matches repository plan application from standard input', async () => {
    const { left, right } = await pair();
    await Promise.all([writeFile(join(left, 'todo.txt'), 'From stdin\n'), writeFile(join(right, 'todo.txt'), 'From stdin\n')]);
    const planArgs = [
      'repo',
      'plan',
      '--source',
      'todo-root',
      '--transition',
      'relocate',
      '--to-path',
      '.local/todo.txt',
      '--delete-source',
      '--json',
    ];
    const expectedPlan = await runTypeScript(left, planArgs);
    const actualPlan = await runRust(right, planArgs);

    expect(await runRust(right, ['repo', 'apply', '--plan', '-'], actualPlan.stdout)).toEqual(
      await runTypeScriptProcess(left, ['repo', 'apply', '--plan', '-'], expectedPlan.stdout),
    );
    expect(await files(right)).toEqual(await files(left));
  });

  it('matches release baseline preview, apply and check', async () => {
    const { left, right } = await releasePair();
    for (const args of [
      ['release', 'baseline', '0.8.0', '--date', '2026-06-11'],
      ['release', 'baseline', '0.8.0', '--date', '2026-06-11', '--apply'],
      ['release', 'check'],
    ]) {
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
      expect(await files(right)).toEqual(await files(left));
    }
    for (const root of [left, right]) {
      const change = join(root, 'openspec', 'changes', 'archive', '2026-08-20-native-cli');
      await mkdir(change, { recursive: true });
      await writeFile(
        join(change, 'release.md'),
        `---
category: added
impact: minor
visibility: public
components:
  - cli
---

# Run the native CLI

Arcantry now runs through the native executable.
`,
      );
    }
    for (const args of [
      ['release', 'plan'],
      ['release', 'cut', '--date', '2026-08-20'],
      ['release', 'cut', '--date', '2026-08-20', '--apply'],
      ['release', 'render'],
      ['release', 'check'],
    ]) {
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
      expect(await files(right)).toEqual(await files(left));
    }
  });

  it('matches rejection of non-stable release baselines', async () => {
    const { left, right } = await releasePair();
    for (const version of ['0.8.0-alpha', '0.8.0+build']) {
      const args = ['release', 'baseline', version, '--date', '2026-06-11'];
      expect(await runRust(right, args)).toEqual(await runTypeScript(left, args));
    }
  });

  it('matches adoption of a missing standard source into configuration', async () => {
    const { left, right } = await pair();
    await Promise.all([
      writeFile(join(left, 'arcantry.toml'), 'config_version = 1\n'),
      writeFile(join(right, 'arcantry.toml'), 'config_version = 1\n'),
    ]);
    const args = ['repo', 'plan', '--source', 'todo-root', '--transition', 'adopt', '--json'];
    const expected = await runTypeScript(left, args);
    const actual = await runRust(right, args);
    expect({ ...JSON.parse(actual.stdout), root: '<root>' }).toEqual({ ...JSON.parse(expected.stdout), root: '<root>' });
    await Promise.all([writeFile(join(left, 'plan.json'), expected.stdout), writeFile(join(right, 'plan.json'), actual.stdout)]);
    expect(await runRust(right, ['repo', 'apply', '--plan', 'plan.json'])).toEqual(
      await runTypeScript(left, ['repo', 'apply', '--plan', 'plan.json']),
    );
    await Promise.all([rm(join(left, 'plan.json')), rm(join(right, 'plan.json'))]);
    expect(await files(right)).toEqual(await files(left));
  });

  it('matches configured changelog cutover', async () => {
    const { left, right } = await pair();
    const config = `config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "observe"
adapter = "keep-a-changelog@1"
from = ["intent"]
`;
    const changelog = '# Changelog\n\n## [Unreleased]\n\n## [0.9.0] - 2026-07-01\n\n### Added\n\n- Legacy\n';
    for (const root of [left, right]) {
      await mkdir(join(root, 'openspec'));
      await writeFile(join(root, 'arcantry.toml'), config);
      await writeFile(join(root, 'CHANGELOG.md'), changelog);
    }
    const args = [
      'repo',
      'plan',
      '--source',
      'history',
      '--transition',
      'cutover',
      '--managed-from',
      '1.0.0',
      '--json',
    ];
    const expected = await runTypeScript(left, args);
    const actual = await runRust(right, args);
    expect({ ...JSON.parse(actual.stdout), root: '<root>' }).toEqual({ ...JSON.parse(expected.stdout), root: '<root>' });
  });
});
