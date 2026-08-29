import { readdir, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { load } from 'cheerio';
import { marked } from 'marked';
import { describe, expect, it } from 'vitest';

type Contract = {
  globalOptions: Array<{ syntax: string; evidence: string }>;
  commands: Array<{ path: string; syntax: string; evidence: string }>;
  trustClaims: Array<{ id: string; evidence: string }>;
  scenarios: Array<{ id: string; command: string }>;
};

const root = process.cwd();
const contract = JSON.parse(await readFile(join(root, 'contracts', 'cli-contract.json'), 'utf8')) as Contract;
const cliReferencePath = join(root, 'apps', 'docs', 'src', 'content', 'docs', 'reference', 'cli.md');
const docsRoot = join(root, 'apps', 'docs', 'src', 'content', 'docs');

const documentationFiles = async (directory: string): Promise<string[]> => {
  const files: string[] = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) files.push(...(await documentationFiles(path)));
    else if (/\.mdx?$/u.test(entry.name)) files.push(path);
  }
  return files;
};

const normalize = (value: string) =>
  value
    .replace(/^arcantry\s+/u, '')
    .replace(/\s+/gu, ' ')
    .trim();

describe('CLI documentation contract', () => {
  it('renders every command syntax as one intact two-column row or syntax block', async () => {
    const markdown = await readFile(cliReferencePath, 'utf8');
    const html = await marked.parse(markdown.replace(/^---[\s\S]*?---\s*/u, ''));
    const $ = load(html);

    const renderedSyntax = new Set<string>();
    $('table tbody tr td:first-child code, pre code').each((_index, code) => {
      const syntax = normalize($(code).text());
      if (/^(repo|todo|release|skills)\s/u.test(syntax)) renderedSyntax.add(syntax);
    });

    expect(renderedSyntax).toEqual(new Set(contract.commands.map(({ syntax }) => syntax)));
    expect(renderedSyntax).toContain('repo init --scope <shared|private> [--compat claude]');
    expect(renderedSyntax).toContain(
      'skills link <name> (--scope <user|repo|private> [--compat claude] | --target <path>) [--replace]',
    );

    const renderedGlobalOptions = new Set<string>();
    $('table tbody tr td:first-child code').each((_index, code) => {
      const syntax = normalize($(code).text());
      if (syntax.startsWith('-')) renderedGlobalOptions.add(syntax);
    });
    expect(renderedGlobalOptions).toEqual(new Set(contract.globalOptions.map(({ syntax }) => syntax)));
  });

  it('renders every Markdown table with the same number of cells as its header', async () => {
    for (const path of await documentationFiles(docsRoot)) {
      const markdown = await readFile(path, 'utf8');
      const $ = load(await marked.parse(markdown.replace(/^---[\s\S]*?---\s*/u, '')));
      $('table').each((_tableIndex, table) => {
        const columns = $(table).find('thead tr').first().children('th').length;
        expect(columns, path).toBeGreaterThan(0);
        $(table)
          .find('tbody tr')
          .each((_rowIndex, row) => expect($(row).children('td'), path).toHaveLength(columns));
      });
    }
  });

  it('keeps every bounded public trust claim mapped to executable evidence', async () => {
    const documentation = await Promise.all(
      [
        'apps/docs/src/content/docs/reference/cli.md',
        'apps/docs/src/content/docs/reference/repository-workflow.md',
        'apps/docs/src/content/docs/guides/todo-txt.md',
        'apps/docs/src/content/docs/lifecycle/releases.mdx',
      ].map((path) => readFile(join(root, path), 'utf8')),
    );
    const source = documentation.join('\n');
    const executableEvidence = new Set(contract.scenarios.map(({ id }) => id));
    for (const claim of contract.trustClaims) {
      expect(executableEvidence.has(claim.evidence), claim.id).toBe(true);
      expect(source, claim.id).toContain(`cli-evidence: ${claim.id}`);
    }
  });
});
