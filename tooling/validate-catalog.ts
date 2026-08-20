import { validateCatalog } from './catalog.js';

const errors = validateCatalog();
if (errors.length > 0) {
  process.stderr.write(`${errors.map((error) => `- ${error}`).join('\n')}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write('Catalog and skill packages are valid.\n');
}
