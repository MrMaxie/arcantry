# Why

Arcantry's public documentation is currently served beneath `maxie.dev/arcantry`, while generated canonical metadata still identifies the GitHub Pages origin. Public links, schema identifiers and deployment metadata therefore do not describe one stable Arcantry-owned origin, and GitHub Pages' short asset cache limits repeat-visit performance.

# What changes

- Make `https://arcantry.dev/` the only canonical public origin for Arcantry documentation, metadata, schemas and package or plugin links.
- Serve the documentation from the domain root and place Astro-generated, content-hashed assets under `/static/`.
- Configure GitHub Pages and Cloudflare so the apex domain is canonical, `www` redirects to it over HTTPS and only hashed assets receive a six-month edge and browser cache policy.
- Validate generated output and live delivery so the previous Pages and `maxie.dev/arcantry` origins cannot reappear unnoticed.

# Out of scope

- Changing the separate `https://maxie.dev` author link.
- Applying the six-month policy to HTML or non-hashed public files.
- Changing Arcantry's version, creating a tag or GitHub Release, or publishing packages.
