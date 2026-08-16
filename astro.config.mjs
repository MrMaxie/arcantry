import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://mrmaxie.github.io',
  base: '/arcantry/',
  integrations: [
    starlight({
      title: 'Arcantry',
      description: 'Repository foundations for spec-driven delivery.',
      customCss: ['./src/styles/arcantry.css'],
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/MrMaxie/arcantry' }],
      sidebar: [
        { label: 'Start', items: [{ label: 'Overview', slug: 'index' }, { label: 'Adoption', slug: 'adoption' }] },
        { label: 'Lifecycle', items: [{ label: 'Changes', slug: 'lifecycle/changes' }, { label: 'Releases', slug: 'lifecycle/releases' }] },
        { label: 'Reference', items: [{ label: 'Commands', slug: 'reference/commands' }, { label: 'Repository contract', slug: 'reference/repository-contract' }] }
      ]
    })
  ]
});
