import { defineConfig } from 'tsup';

export default defineConfig({
  clean: true,
  dts: true,
  entry: ['src/index.ts', 'src/cli.ts', 'src/catalog.ts', 'src/repository.ts', 'src/project.ts', 'src/release.ts', 'src/agents.ts'],
  format: ['esm'],
  sourcemap: true,
  target: 'node24',
});
