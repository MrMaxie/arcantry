---
category: fixed
impact: patch
visibility: public
components:
  - catalog
---

# Keep skill package identity stable across platforms

Arcantry now produces and transports the same skill package revision from equivalent Windows and Linux text checkouts while preserving exact byte identity for binary resources. Package projection, skill update, native package smoke and filesystem-specific CLI checks also run reliably from clean checkouts.
