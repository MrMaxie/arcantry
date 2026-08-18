import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://mrmaxie.github.io',
  base: '/arcantry/',
  integrations: [
    starlight({
      title: 'Arcantry',
      description: 'Composable, versioned project knowledge without a required repository shape.',
      customCss: ['./src/styles/arcantry.css', './src/styles/arcantry-concept.css'],
      components: {
        Head: './src/components/ArcantryHead.astro',
        PageFrame: './src/components/ArcantryPageFrame.astro',
        Header: './src/components/ArcantryHeader.astro',
        Footer: './src/components/ArcantryFooter.astro',
        Sidebar: './src/components/ArcantrySidebar.astro',
        Hero: './src/components/ArcantryHero.astro'
      },
      sidebar: [
        {
          label: 'Start',
          items: [
            { label: 'Homepage', slug: 'index' },
            { label: 'Get started', slug: 'getting-started' },
            { label: 'Adoption paths', slug: 'adoption' }
          ]
        },
        {
          label: 'Guides',
          items: [
            { label: 'Inspect, plan and apply', slug: 'reference/repository-workflow' },
            { label: 'todo.txt queues', slug: 'guides/todo-txt' },
            { label: 'Skills', slug: 'skills' }
          ]
        },
        {
          label: 'Concepts',
          items: [{ label: 'Project knowledge stack', slug: 'reference/repository-contract' }]
        },
        {
          label: 'Reference',
          items: [
            { label: 'CLI', slug: 'reference/cli' },
            { label: 'Configuration', slug: 'reference/configuration' },
            {
              label: 'Skill catalog',
              collapsed: true,
              items: [
                { label: 'Catalog overview', slug: 'skills/catalog' },
                { label: 'Adopt Arcantry', slug: 'skills/adopt-arcantry' },
                { label: 'Agent Self Improve', slug: 'skills/agent-self-improve' },
                { label: 'Audit Skill Portfolio', slug: 'skills/audit-skill-portfolio' },
                { label: 'Capture Repeatable Work', slug: 'skills/capture-repeatable-work' },
                { label: 'Evaluate Skill Change', slug: 'skills/evaluate-skill-change' },
                { label: 'Forge Skill from Conversations', slug: 'skills/forge-skill-from-conversations' },
                { label: 'Productize Repeatable Work', slug: 'skills/productize-repeatable-work' },
                { label: 'Select Task Skills', slug: 'skills/select-task-skills' },
                { label: 'Audience and Scope Discipline', slug: 'skills/audience-scope-discipline' },
                { label: 'Design Terminal UX', slug: 'skills/design-terminal-ux' },
                { label: 'Intake Linear Work', slug: 'skills/intake-linear-work' },
                { label: 'Intake Repository Work', slug: 'skills/intake-repository-work' },
                { label: 'Promote Meeting Notes', slug: 'skills/promote-meeting-notes' },
                { label: 'Stage Code Review Findings', slug: 'skills/stage-code-review-findings' }
              ]
            }
          ]
        },
        {
          label: 'Contributing to Arcantry',
          items: [
            { label: 'Change lifecycle', slug: 'lifecycle/changes' },
            { label: 'Release lifecycle', slug: 'lifecycle/releases' },
            { label: 'Contributor commands', slug: 'reference/commands' }
          ]
        }
      ]
    })
  ]
});
