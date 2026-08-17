#!/usr/bin/env python3
"""Build a read-only, cross-platform index of installed and project skills."""

from __future__ import annotations

import argparse
import json
import os
import re
from pathlib import Path


FIELD = re.compile(r"^(name|description):\s*[\"']?(.+?)[\"']?\s*$", re.MULTILINE)


def scan(root: Path, scope: str) -> list[dict[str, object]]:
    if not root.is_dir():
        return []
    items: list[dict[str, object]] = []
    for path in sorted(root.glob("*/SKILL.md")):
        text = path.read_text(encoding="utf-8")
        values = {key: value.strip().strip("\"'") for key, value in FIELD.findall(text)}
        if "name" in values:
            items.append({
                "name": values["name"],
                "description": values.get("description", ""),
                "availability": "use_now",
                "scope": scope,
                "path": str(path.resolve()),
            })
    return items


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path)
    parser.add_argument("--codex-home", type=Path)
    parser.add_argument("--format", choices=("json", "table"), default="json")
    args = parser.parse_args()
    home = Path.home()
    codex_home = (args.codex_home or Path(os.environ.get("CODEX_HOME", home / ".codex"))).resolve()
    items = scan(codex_home / "skills", "global") + scan(home / ".agents" / "skills", "user")
    if args.project_root:
        project = args.project_root.resolve()
        items += scan(project / ".agents" / "skills", "project")
        items += scan(project / ".codex" / "skills", "project")
    counts: dict[str, int] = {}
    for item in items:
        counts[str(item["name"])] = counts.get(str(item["name"]), 0) + 1
    output = sorted(items, key=lambda item: (str(item["name"]), str(item["path"])))
    for item in output:
        item["duplicate_name_count"] = counts[str(item["name"])]
    if args.format == "table":
        for item in output:
            print(f"{item['name']:<36} {item['scope']:<10} {item['path']}")
    else:
        print(json.dumps({"schemaVersion": 1, "readOnly": True, "skills": output}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
