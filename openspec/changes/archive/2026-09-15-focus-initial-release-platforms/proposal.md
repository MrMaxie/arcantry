# Why

The first public release currently requires six native targets and seven npm packages even though Arcantry has no published consumers and no demonstrated need for native Windows ARM64 or Linux ARM64 builds. The additional targets expand bootstrap publication, trusted-publisher configuration and release verification before the first release.

# What changes

Use the established initial distribution matrix already used by ttyglass: Windows x64, macOS x64 and ARM64, and Linux x64. Publish one main npm package plus four platform packages. Remove Windows ARM64 and Linux ARM64 from current product, documentation and release contracts while retaining macOS ARM64.

# Out of scope

- Changing the Arcantry version.
- Moving or publishing the `v1.0.0` tag.
- Adding emulation or runtime binary downloads for unsupported platforms.
- Rewriting archived OpenSpec history that recorded the earlier six-target design.
