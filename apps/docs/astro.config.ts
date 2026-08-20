import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import skillSidebar from './src/generated/skill-sidebar';

export default defineConfig({
  site: 'https://mrmaxie.github.io',
  base: '/arcantry/',
  integrations: [
    starlight({
      title: 'Arcantry',
      description: 'Composable, versioned project knowledge without a required repository shape.',
      expressiveCode: {
        themes: ['catppuccin-mocha', 'catppuccin-latte'],
        useStarlightUiThemeColors: true,
      },
      customCss: ['./src/styles/arcantry.css', './src/styles/arcantry-concept.css'],
      components: {
        Head: './src/components/ArcantryHead.astro',
        PageFrame: './src/components/ArcantryPageFrame.astro',
        Header: './src/components/ArcantryHeader.astro',
        Footer: './src/components/ArcantryFooter.astro',
        Sidebar: './src/components/ArcantrySidebar.astro',
        Hero: './src/components/ArcantryHero.astro',
      },
      sidebar: [
        {
          label: 'Start',
          items: [
            { label: 'Homepage', slug: 'index' },
            { label: 'Get started', slug: 'getting-started' },
            { label: 'Adoption paths', slug: 'adoption' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Inspect, plan and apply', slug: 'reference/repository-workflow' },
            { label: 'todo.txt queues', slug: 'guides/todo-txt' },
            { label: 'Skills', slug: 'skills' },
          ],
        },
        {
          label: 'Concepts',
          items: [{ label: 'Project knowledge stack', slug: 'reference/repository-contract' }],
        },
        {
          label: 'Reference',
          items: [
            { label: 'CLI', slug: 'reference/cli' },
            { label: 'Configuration', slug: 'reference/configuration' },
            {
              label: 'Skill catalog',
              collapsed: true,
              items: [{ label: 'Catalog overview', slug: 'skills/catalog' }, ...skillSidebar],
            },
          ],
        },
        {
          label: 'Contributing to Arcantry',
          items: [
            { label: 'Change lifecycle', slug: 'lifecycle/changes' },
            { label: 'Release lifecycle', slug: 'lifecycle/releases' },
            { label: 'Contributor commands', slug: 'reference/commands' },
          ],
        },
      ],
    }),
  ],
});
