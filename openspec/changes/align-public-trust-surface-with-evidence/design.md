# Approach

Create a machine-checked trust-claim inventory covering the public overview, adoption guidance, reference documentation, catalog and package surfaces. Each claim records its subject, audience, scope, accepted requirement, executable or inspectable evidence, responsible boundary and unsupported conditions. Documentation generation and validation reject missing or stale references.

CLI claims continue to use the provenance ledger owned by `preserve-cli-intent-provenance`; the broader inventory references that ledger instead of duplicating its command evidence. Evidence that cannot be exposed safely does not support a broad public claim. In that case the public wording must narrow to the observable boundary and identify the party responsible for the remaining assurance.

# Trade-offs

Maintaining a claim inventory adds review work whenever behavior or evidence changes. The cost is bounded by limiting it to material public trust statements rather than every descriptive sentence.
