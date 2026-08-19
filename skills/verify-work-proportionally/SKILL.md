---
name: verify-work-proportionally
description: Match verification layers to impact, uncertainty, reversibility, and audience when tests alone cannot prove the requested outcome or acceptance boundary.
---

# Verify Work Proportionally

Use the cheapest layer that can disprove the important risk, then add independence when impact or uncertainty justifies it. More checks are not automatically better.

## Choose layers

- Static: schema, formatting, types, lint, contract checks.
- Automated behavior: focused tests, integration tests, build, package validation.
- Live behavior: browser, CLI, service, or device verification against the actual surface.
- Fresh context: review by an agent that did not produce the implementation.
- Cross-agent challenge: bounded adversarial or specialist review with explicit coverage.
- User acceptance: confirmation of taste, business meaning, irreversible action, private judgment, or an unresolved material choice.

## Workflow

1. State the changed surface, consumers, failure cost, reversibility, and uncertain assumptions.
2. Map each material risk to one verification layer and an observable pass condition.
3. Run safe in-scope checks. Delegate only bounded independent questions and reconcile the evidence yourself.
4. Ask the user only for judgments or authority that cannot be established from project evidence.
5. Classify the result as verified behavior, documented plan, source estimate, assumption, or unverified acceptance step.
6. Report exact coverage and failures. Do not claim the whole repository was reviewed when any material component is unaccounted for.

Stop adding layers when remaining checks repeat the same evidence or cannot change the decision. Never use a clean build as proof of deployment, live interaction, or user acceptance.
