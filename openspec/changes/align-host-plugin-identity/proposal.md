# Why

Arcantry already ships Codex and Claude plugin manifests, but their shared product identity and host-specific presentation are not held by one accepted contract. The surfaces can drift in name, description, version, links or branding even while each manifest remains independently valid.

# What changes

- Define the canonical product identity shared by every supported host plugin.
- Keep host-specific presentation fields appropriate to the host instead of forcing one manifest shape onto another.
- Validate every supported plugin manifest and its packaged projection against the same identity source.

# Out of scope

- Adding host-specific commands or capabilities.
- Making one host's optional manifest fields mandatory in another host.
- Changing the Arcantry product version.
