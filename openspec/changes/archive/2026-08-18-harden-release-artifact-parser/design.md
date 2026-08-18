# Approach

After frontmatter validation, inspect only the first content line. Advance across the required horizontal whitespace with a simple loop, derive the title and body with string slices, and reject an empty title or body. This keeps work proportional to input length and retains the documented `# Title` artifact shape.
