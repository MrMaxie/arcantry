import { defineCollection } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

export const collections = {
  docs: defineCollection({
    loader: docsLoader({
      generateId: ({ entry }) => {
        const id = entry.replace(/^_generated\//, '').replace(/\.(?:md|mdx)$/, '');
        return id.endsWith('/index') ? id.slice(0, -'/index'.length) : id;
      },
    }),
    schema: docsSchema(),
  }),
};
