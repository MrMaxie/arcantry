# Approach

Treat the malformed legacy specifications as one migration boundary. Each existing requirement keeps its normative text and gains a canonical `### Requirement:` header plus one observable scenario. The repository gate then validates every current specification and active change in strict, non-interactive mode before other checks run.

The Pages workflow runs `just ci` before it uploads a deployable artifact. This duplicates verification across workflows, but it prevents documentation from deploying when the source tree fails the product gate.

# Trade-offs

Adding all legacy scenarios in one change is intentionally broad. A partial migration cannot be archived because OpenSpec validates the complete rebuilt specification and rejects untouched requirements without scenarios.
