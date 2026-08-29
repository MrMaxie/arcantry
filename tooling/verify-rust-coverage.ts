import { readdir, readFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { type CoveragePolicy, verifyCoverage } from './rust-coverage-policy';

const reportPath = process.argv[2] ?? 'target/rust-coverage.lcov';
const policyPath = process.argv[3] ?? 'contracts/rust-coverage-policy.json';
const report = await readFile(reportPath, 'utf8');
const policy = JSON.parse(await readFile(policyPath, 'utf8')) as CoveragePolicy;
const productionFiles = [
  ...(await rustFiles('crates/arcantry-cli/src')),
  ...(await rustFiles('crates/arcantry-core/src')),
  'crates/arcantry-cli/build.rs',
];
const summaries = verifyCoverage(report, policy, productionFiles);

for (const summary of summaries) {
  const branch = summary.branch.percent === null ? 'n/a' : `${summary.branch.percent.toFixed(2)}%`;
  console.log(`${summary.path}: lines ${summary.line.percent.toFixed(2)}%, branches ${branch}`);
}

async function rustFiles(directory: string): Promise<string[]> {
  const found = [] as string[];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) found.push(...(await rustFiles(path)));
    else if (entry.isFile() && entry.name.endsWith('.rs')) {
      found.push(relative('.', path).replaceAll('\\', '/'));
    }
  }
  return found;
}
