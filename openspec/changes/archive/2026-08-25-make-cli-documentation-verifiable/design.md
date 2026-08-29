# Approach

The CLI contract inventory is the structural bridge between OpenSpec, authored documentation and the compiled binary. The CLI package exports the same Clap model used by the executable. Verification scans every Markdown and MDX document, tokenizes each fenced or tabular `arcantry` command with `shell-words`, and parses it through that shared model. A tooling test compares the canonical CLI reference with the inventory and renders every documentation table to inspect its header and body cell structure.

Narrative trust claims remain authored for readers. A small evidence mapping keeps claims about writes, network behavior, removal and rollback tied to executable scenario ids without exposing diagnostic implementation detail in the public page.

# Trade-offs

Canonical command syntax remains coupled to the inventory, while executable examples across the complete documentation surface are coupled directly to Clap. Narrative documentation remains authored because generating explanatory prose from command metadata would make implementation wording the product authority.
