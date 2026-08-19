import { cpSync, mkdirSync, rmSync } from 'node:fs';
import { join, resolve } from 'node:path';

const root = process.cwd();
const packageRoot = resolve(root, 'packages', 'arcantry');
const projections = [
  join(packageRoot, '.claude-plugin'),
  join(packageRoot, '.codex-plugin'),
  join(packageRoot, 'assets'),
  join(packageRoot, 'skills'),
  join(packageRoot, 'schemas'),
  join(packageRoot, 'catalog.json'),
  join(packageRoot, 'LICENSE'),
  join(packageRoot, 'README.md'),
];

for (const path of projections) {
  if (!resolve(path).startsWith(`${packageRoot}\\`) && !resolve(path).startsWith(`${packageRoot}/`)) {
    throw new Error(`Refusing to clean package projection outside ${packageRoot}: ${path}`);
  }
  rmSync(path, { recursive: true, force: true });
}

mkdirSync(join(packageRoot, 'assets'), { recursive: true });
cpSync(join(root, '.claude-plugin'), join(packageRoot, '.claude-plugin'), { recursive: true });
cpSync(join(root, '.codex-plugin'), join(packageRoot, '.codex-plugin'), { recursive: true });
cpSync(join(root, 'catalog.json'), join(packageRoot, 'catalog.json'));
cpSync(join(root, 'skills'), join(packageRoot, 'skills'), { recursive: true });
cpSync(join(root, 'schemas'), join(packageRoot, 'schemas'), { recursive: true });
cpSync(join(root, 'openspec', 'schemas', 'arcantry'), join(packageRoot, 'assets', 'openspec'), { recursive: true });
cpSync(join(root, 'LICENSE'), join(packageRoot, 'LICENSE'));
cpSync(join(root, 'README.md'), join(packageRoot, 'README.md'));

process.stdout.write('Prepared package projections from canonical Arcantry sources.\n');
