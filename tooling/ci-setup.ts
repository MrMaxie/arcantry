import { appendFile } from 'node:fs/promises';
import { dirname } from 'node:path';

const githubPath = process.env.GITHUB_PATH;
if (githubPath === undefined) throw new Error('GITHUB_PATH is required for CI setup.');

await appendFile(githubPath, `${dirname(process.execPath)}\n`, 'utf8');
process.stdout.write(`Added Nub's Node directory to GITHUB_PATH.\n`);
