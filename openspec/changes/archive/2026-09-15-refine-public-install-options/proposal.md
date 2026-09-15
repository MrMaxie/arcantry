# Why

The public installation picker currently presents more package launchers than the supported 1.0 guidance needs, while direct GitHub Release downloads are described only in surrounding text. Readers need one short, ordered set of supported choices with the manual download route visible as a link.

# What changes

Present installation choices in this order: `npx`, `npm`, PowerShell, `sh`, then a `Download` link to the versioned GitHub Release. Remove pnpm and Nub from the public installation picker and its accompanying installation copy.

# Out of scope

- Removing pnpm or Nub from contributor tooling or unrelated skill installation examples.
- Adding another package manager or distribution channel.
- Changing installers, release artifacts or package contents.
