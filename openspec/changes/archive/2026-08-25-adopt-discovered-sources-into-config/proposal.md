# Why

`repo plan --transition adopt` can initialize a discovered or standard missing source, but it does not add that source to an existing project configuration. The applied result is therefore not durable: a subsequent inspection still treats the source as unconfigured and does not preserve requested dependencies.

# What changes

- Make adoption add discovered and standard missing sources to the active shared, private or external `arcantry.toml`.
- Let callers declare source dependencies during adoption with `--from` while preserving the existing privacy and adapter validation rules.
- Keep planning read-only and make configuration, source initialization and local exclusion updates one drift-checked apply operation.

# Out of scope

- Merging shared and private configuration files.
- Automatically choosing dependencies that the caller did not request.
- Changing preserve, relocate or cutover semantics beyond reusing the same configuration update path.
