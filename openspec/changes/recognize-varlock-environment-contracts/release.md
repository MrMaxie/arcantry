---
category: added
impact: minor
visibility: public
components:
  - project-knowledge-stack
  - repository-adoption
  - cli
  - documentation-site
---

# Recognize Varlock environment contracts with resilient loading

Arcantry identifies optional shared and private environment schemas as project context. Operations that need environment values prefer a compatible existing Varlock setup and fall back to dotenv-compatible loading without making optional integration failures block unrelated work.
