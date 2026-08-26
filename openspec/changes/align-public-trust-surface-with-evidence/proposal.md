# Why

Public safety language can imply broad guarantees about data flow, network access, permissions, repository writes, skill trust and vulnerability handling while the visible evidence currently concentrates on narrower repository-integrity behavior. Readers cannot reliably distinguish an executable guarantee, a documented responsibility boundary and an unsupported aspiration.

# What changes

- Inventory every public trust and safety claim with its audience, scope, owner and visible evidence.
- Require claims to remain no broader than accepted requirements and evidence available to the same audience.
- Present responsibility boundaries where Arcantry cannot guarantee host, connector, dependency, platform or operator behavior.
- Narrow or remove claims when the required evidence does not exist.
- Keep CLI-specific provenance under the active `preserve-cli-intent-provenance` change.

# Out of scope

- Claiming certification or compliance that Arcantry has not earned.
- Publishing sensitive diagnostics or private evidence.
- Replacing host, dependency or vulnerability-management responsibilities.
