## ADDED Requirements

### Requirement: Documentation has one canonical public origin

The documentation site, canonical and social metadata, sitemaps, public schema identities, runtime schema defaults, package metadata and plugin metadata MUST use `https://arcantry.dev/` as the public Arcantry origin. The site MUST be published from the domain root through the repository's GitHub Pages workflow. Public documentation links MUST NOT depend on the previous `/arcantry/` repository base path or identify the GitHub Pages origin as canonical. The independent `https://maxie.dev` author identity MAY remain linked as author information.

#### Scenario: A public Arcantry URL is generated

- **WHEN** documentation and package projections are generated from a clean checkout
- **THEN** canonical metadata, Open Graph URLs, sitemap entries, schema locations and product homepage links use `https://arcantry.dev/`
- **AND** internal documentation links resolve from the domain root
- **AND** generated-output verification rejects the previous public origins and repository base path

### Requirement: Hashed documentation assets use a dedicated cache path

Astro-generated JavaScript, CSS, optimized images, fonts and other content-hashed documentation assets MUST be emitted under `/static/`. Requests under `/static/` MUST be served through Cloudflare with an edge and browser cache TTL of 15,552,000 seconds. HTML and non-hashed public files MUST NOT inherit this long-lived cache policy.

#### Scenario: A visitor requests a generated asset twice

- **WHEN** a deployed content-hashed asset under `/static/` is requested through `arcantry.dev`
- **THEN** the response advertises a 15,552,000-second browser cache lifetime
- **AND** a repeated request can be served as a Cloudflare cache hit
- **AND** an HTML response remains outside the long-lived asset rule

### Requirement: The custom domain preserves secure canonical routing

The GitHub Pages site MUST assign `arcantry.dev` as its custom domain, serve it over enforced HTTPS and accept both the apex and `www` DNS variants. Cloudflare MUST proxy the web records only after GitHub Pages has provisioned the custom-domain certificate, use strict TLS to the origin and redirect HTTP to HTTPS. The `www` variant MUST redirect to the canonical apex origin.

#### Scenario: A visitor uses a non-canonical entry point

- **WHEN** they request HTTP or the `www` hostname
- **THEN** they reach the corresponding HTTPS route on `https://arcantry.dev/`
- **AND** the delivered page identifies the apex origin as canonical
