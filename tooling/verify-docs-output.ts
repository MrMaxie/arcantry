import { readdir, readFile, stat } from 'node:fs/promises';
import { extname, join, relative } from 'node:path';

const root = process.cwd();
const outputRoot = join(root, 'apps', 'docs', 'dist');
const inspectedExtensions = new Set(['.css', '.html', '.js', '.json', '.md', '.svg', '.tosd', '.xml']);
const stalePatterns = [
  'https://maxie.dev/arcantry/',
  'https://mrmaxie.github.io/arcantry/',
  'href="/arcantry/',
  'src="/arcantry/',
  "href='/arcantry/",
  "src='/arcantry/",
  '/_astro/',
];

const failures: string[] = [];
const outputFiles = await files(outputRoot);
for (const path of outputFiles) {
  if (!inspectedExtensions.has(extname(path))) continue;
  const content = await readFile(path, 'utf8');
  for (const pattern of stalePatterns) {
    if (content.includes(pattern)) failures.push(`${relative(outputRoot, path)} contains ${JSON.stringify(pattern)}`);
  }
}

const homepage = await readFile(join(outputRoot, 'index.html'), 'utf8');
if (!homepage.includes('<link rel="canonical" href="https://arcantry.dev/"')) {
  failures.push('index.html does not declare https://arcantry.dev/ as canonical');
}
if (!homepage.includes('<meta property="og:url" content="https://arcantry.dev/"')) {
  failures.push('index.html does not declare https://arcantry.dev/ as its Open Graph URL');
}

const sitemapIndex = await readFile(join(outputRoot, 'sitemap-index.xml'), 'utf8');
if (!sitemapIndex.includes('<loc>https://arcantry.dev/sitemap-')) {
  failures.push('sitemap-index.xml does not reference the arcantry.dev sitemap');
}

const staticPath = join(outputRoot, 'static');
if (!(await existsDirectory(staticPath)) || (await files(staticPath)).length === 0) {
  failures.push('Astro did not emit generated assets under dist/static');
}
if (await existsDirectory(join(outputRoot, '_astro'))) {
  failures.push('Astro emitted the deprecated dist/_astro asset directory');
}

if (failures.length > 0) {
  process.stderr.write(`${failures.map((failure) => `- ${failure}`).join('\n')}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write('Documentation output uses arcantry.dev and content-hashed /static/ assets.\n');
}

async function files(directory: string): Promise<string[]> {
  if (!(await existsDirectory(directory))) return [];
  const found: string[] = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) found.push(...(await files(path)));
    else if (entry.isFile()) found.push(path);
  }
  return found;
}

async function existsDirectory(path: string): Promise<boolean> {
  try {
    return (await stat(path)).isDirectory();
  } catch {
    return false;
  }
}
