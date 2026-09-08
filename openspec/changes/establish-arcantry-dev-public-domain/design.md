# Approach

Configure Astro with `site: 'https://arcantry.dev'`, no repository base path and `build.assets: 'static'`. Replace authored and generated public URLs with the canonical root origin, including schema locations and identifiers, while preserving repository URLs and the independent Maxie author link.

Add a post-build verifier that inventories generated documentation output, rejects stale origins and repository-base links, checks the canonical and Open Graph URLs, checks sitemap ownership and confirms that generated assets live under `static/` rather than `_astro/`.

Cut over external delivery in stages. Verify the domain for the GitHub account, assign `arcantry.dev` to this repository's GitHub Pages site, publish the root-based build, add the documented GitHub Pages apex and `www` DNS records without proxying, and wait for GitHub's certificate and HTTPS enforcement. Only then enable Cloudflare proxying, `Full (strict)`, HTTPS redirection and a six-month HSTS policy without subdomain inclusion or preload. Apply the six-month edge and browser TTL only when the request path starts with `/static/`.

# Trade-offs

Renaming Astro's default `_astro` directory is an explicit convention that must remain aligned with the Cloudflare rule. Generated filenames remain content-hashed, so a long browser TTL is safe for those assets, while HTML and stable public filenames retain shorter policies so changes can propagate without renaming them.

The staged DNS and TLS cutover takes longer than enabling the proxy immediately, but it lets GitHub provision and prove the origin certificate before Cloudflare enforces strict origin TLS.
