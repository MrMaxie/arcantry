#!/usr/bin/env python3
"""Validate lightweight self-improvement evaluation case files."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path


REQUIRED_FIELDS = {
    "id",
    "kind",
    "prompt",
    "expected_route",
    "rubric",
    "holdout",
    "scope",
}
KINDS = {"direct", "indirect", "negative", "behavior"}
SCOPES = {"global", "project", "local"}
MINIMUM_COUNTS = {"direct": 2, "indirect": 2, "negative": 3, "behavior": 2}
SENSITIVE_PATTERNS = {
    "Windows absolute path": re.compile(r"(?i)(?<![A-Z0-9])[A-Z]:\\(?:Users|Documents and Settings)\\"),
    "UNC path": re.compile(r"\\\\[^\\\s]+\\[^\\\s]+"),
    "private URL": re.compile(
        r"(?i)https?://(?:localhost|127(?:\.\d{1,3}){3}|10(?:\.\d{1,3}){3}|"
        r"192\.168(?:\.\d{1,3}){2}|172\.(?:1[6-9]|2\d|3[01])(?:\.\d{1,3}){2}|[^/\s]+\.internal)(?::\d+)?(?:/|\b)"
    ),
    "credential assignment": re.compile(
        r"(?i)\b(?:api[_-]?key|access[_-]?token|password|passwd|secret)\s*[:=]\s*['\"]?[^\s,'\"]+"
    ),
    "bearer token": re.compile(r"(?i)\bbearer\s+[A-Za-z0-9._~+/=-]{12,}"),
    "OpenAI-style secret": re.compile(r"\bsk-[A-Za-z0-9_-]{12,}"),
}


def validate_file(path: Path) -> tuple[dict[str, object], list[str]]:
    errors: list[str] = []
    records: list[dict[str, object]] = []
    seen_ids: set[str] = set()

    if not path.is_file():
        return {"path": str(path), "records": 0}, [f"file not found: {path}"]

    for line_number, raw_line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw_line.strip():
            continue
        try:
            record = json.loads(raw_line)
        except json.JSONDecodeError as exc:
            errors.append(f"{path}:{line_number}: invalid JSON: {exc.msg}")
            continue
        if not isinstance(record, dict):
            errors.append(f"{path}:{line_number}: record must be an object")
            continue

        missing = REQUIRED_FIELDS - record.keys()
        extra = record.keys() - REQUIRED_FIELDS
        if missing:
            errors.append(f"{path}:{line_number}: missing fields: {', '.join(sorted(missing))}")
        if extra:
            errors.append(f"{path}:{line_number}: unsupported fields: {', '.join(sorted(extra))}")

        case_id = record.get("id")
        if not isinstance(case_id, str) or not case_id.strip():
            errors.append(f"{path}:{line_number}: id must be a non-empty string")
        elif case_id in seen_ids:
            errors.append(f"{path}:{line_number}: duplicate id: {case_id}")
        else:
            seen_ids.add(case_id)

        if record.get("kind") not in KINDS:
            errors.append(f"{path}:{line_number}: kind must be one of {sorted(KINDS)}")
        if record.get("scope") not in SCOPES:
            errors.append(f"{path}:{line_number}: scope must be one of {sorted(SCOPES)}")
        if not isinstance(record.get("prompt"), str) or not str(record.get("prompt", "")).strip():
            errors.append(f"{path}:{line_number}: prompt must be a non-empty string")
        if not isinstance(record.get("expected_route"), str) or not str(record.get("expected_route", "")).strip():
            errors.append(f"{path}:{line_number}: expected_route must be a non-empty string")
        rubric = record.get("rubric")
        if not isinstance(rubric, list) or not rubric or not all(isinstance(item, str) and item.strip() for item in rubric):
            errors.append(f"{path}:{line_number}: rubric must be a non-empty string list")
        if not isinstance(record.get("holdout"), bool):
            errors.append(f"{path}:{line_number}: holdout must be boolean")

        kind = record.get("kind")
        expected_route = record.get("expected_route")
        if kind == "negative" and expected_route != "none":
            errors.append(f"{path}:{line_number}: negative cases must use expected_route 'none'")
        if kind in {"direct", "indirect", "behavior"} and expected_route == "none":
            errors.append(f"{path}:{line_number}: {kind} cases must name an expected skill route")

        prompt_value = record.get("prompt")
        rubric_value = record.get("rubric")
        inspectable_text = "\n".join(
            [prompt_value if isinstance(prompt_value, str) else ""]
            + ([item for item in rubric_value if isinstance(item, str)] if isinstance(rubric_value, list) else [])
        )
        for label, pattern in SENSITIVE_PATTERNS.items():
            if pattern.search(inspectable_text):
                errors.append(f"{path}:{line_number}: possible {label} is not allowed in shared cases")
        prompt = record.get("prompt")
        if isinstance(prompt, str) and re.search(r"(?im)^\s*user\s*:", prompt) and re.search(
            r"(?im)^\s*(?:agent|assistant)\s*:", prompt
        ):
            errors.append(f"{path}:{line_number}: raw transcript-shaped content is not allowed in shared cases")
        records.append(record)

    counts = Counter(str(record.get("kind")) for record in records)
    for kind, minimum in MINIMUM_COUNTS.items():
        if counts[kind] < minimum:
            errors.append(f"{path}: requires at least {minimum} {kind} cases; found {counts[kind]}")
    holdout_count = sum(record.get("holdout") is True for record in records)
    if holdout_count < 3:
        errors.append(f"{path}: requires at least 3 held-out cases; found {holdout_count}")

    summary: dict[str, object] = {
        "path": str(path),
        "records": len(records),
        "counts": dict(sorted(counts.items())),
        "holdout": holdout_count,
        "valid": not errors,
    }
    return summary, errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path)
    args = parser.parse_args()

    summaries: list[dict[str, object]] = []
    all_errors: list[str] = []
    for path in args.paths:
        summary, errors = validate_file(path)
        summaries.append(summary)
        all_errors.extend(errors)

    print(json.dumps({"valid": not all_errors, "files": summaries, "errors": all_errors}, indent=2))
    return 0 if not all_errors else 1


if __name__ == "__main__":
    sys.exit(main())

