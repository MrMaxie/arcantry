---
category: fixed
impact: patch
visibility: public
components:
  - catalog
---

# Keep skill package identity stable across platforms

Arcantry now produces the same skill package hashes from equivalent Windows and Linux text checkouts while preserving exact byte identity for binary resources. Package projection, skill update and native package smoke checks also run reliably from clean checkouts.
