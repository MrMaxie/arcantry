# Why

The temporary continuous-1.0 policy prevents the repository from using its own OpenSpec release impact and SemVer workflow after the first public release. The first public version is still unpublished, so accepted work belongs in 1.0.0; later releases should advance normally.

# What changes

- Finalize all accepted work in the first public 1.0.0 while no npm package or public GitHub Release exists.
- Treat a failed tag or draft release as replaceable release preparation rather than a live version.
- Resume normal OpenSpec-driven SemVer after 1.0.0 is public.
- Keep every product and platform version source aligned through one repository release plan.

# Out of scope

- Changing schema or independent skill package versions solely to match the product.
- Reusing or replacing a version after its main npm package or public GitHub Release exists.
