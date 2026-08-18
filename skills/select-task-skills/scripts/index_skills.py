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
                "alias_path": str(path.absolute()),
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
    items = (
        scan(home / ".agents" / "skills", "user")
        + scan(home / ".claude" / "skills", "user-compat")
        + scan(codex_home / "skills", "global")
    )
    if args.project_root:
        project = args.project_root.resolve()
        items += scan(project / ".agents" / "skills", "project")
        items += scan(project / ".claude" / "skills", "project-compat")
        items += scan(project / ".local" / "skills", "private")
    scope_priority = {"private": 0, "project": 1, "project-compat": 2, "user": 3, "user-compat": 4, "global": 5}
    deduplicated: dict[str, dict[str, object]] = {}
    for item in items:
        key = os.path.normcase(os.path.realpath(str(item["path"])))
        existing = deduplicated.get(key)
        if existing is None:
            item["aliases"] = [item.pop("alias_path")]
            deduplicated[key] = item
            continue
        existing["aliases"] = sorted(
            set([*existing.get("aliases", []), str(item["alias_path"])]),
            key=str.casefold,
        )
        if scope_priority[str(item["scope"])] < scope_priority[str(existing["scope"] )]:
            existing["scope"] = item["scope"]
    items = list(deduplicated.values())
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
