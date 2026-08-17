#!/usr/bin/env python3
"""Resolve the installed skill-creator validator without machine-specific paths."""

from __future__ import annotations

import argparse
import os
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--codex-home", type=Path)
    args = parser.parse_args()
    home = Path.home().resolve()
    codex_home = (args.codex_home or Path(os.environ.get("CODEX_HOME", home / ".codex"))).resolve()
    candidates = [
        codex_home / "skills" / ".system" / "skill-creator" / "scripts" / "quick_validate.py",
        codex_home / "skills" / "skill-creator" / "scripts" / "quick_validate.py",
        home / ".agents" / "skills" / "skill-creator" / "scripts" / "quick_validate.py",
    ]
    matches = sorted({path.resolve() for path in candidates if path.is_file()})
    if not matches:
        raise SystemExit("quick_validate.py was not found in a supported skill-creator location.")
    if len(matches) > 1:
        raise SystemExit("quick_validate.py is ambiguous: " + ", ".join(map(str, matches)))
    print(matches[0])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
