#!/usr/bin/env python3
"""Unit tests for the shared evaluation-case validator."""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from validate_cases import validate_file


def valid_records() -> list[dict[str, object]]:
    kinds = ["direct", "direct", "indirect", "indirect", "negative", "negative", "negative", "behavior", "behavior"]
    return [
        {
            "id": f"case-{index}",
            "kind": kind,
            "prompt": f"Synthetic prompt {index}",
            "expected_route": "none" if kind == "negative" else "example-skill",
            "rubric": ["Observable result"],
            "holdout": index in {2, 5, 8},
            "scope": "global",
        }
        for index, kind in enumerate(kinds, 1)
    ]


class ValidateCasesTests(unittest.TestCase):
    def validate(self, records: list[dict[str, object]]) -> list[str]:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "cases.jsonl"
            path.write_text("\n".join(json.dumps(record) for record in records), encoding="utf-8")
            _, errors = validate_file(path)
            return errors

    def test_accepts_minimal_valid_set(self) -> None:
        self.assertEqual(self.validate(valid_records()), [])

    def test_rejects_route_semantics_mismatch(self) -> None:
        records = valid_records()
        records[4]["expected_route"] = "example-skill"
        self.assertTrue(any("negative cases must use" in error for error in self.validate(records)))

    def test_rejects_private_url_and_secret_assignment(self) -> None:
        records = valid_records()
        records[0]["prompt"] = "Read https://project.internal/private.txt with api_key=fixture-placeholder"
        errors = self.validate(records)
        self.assertTrue(any("private URL" in error for error in errors))
        self.assertTrue(any("credential assignment" in error for error in errors))

    def test_rejects_raw_transcript_shape(self) -> None:
        records = valid_records()
        records[0]["prompt"] = "User: do this\nAgent: done"
        self.assertTrue(any("raw transcript-shaped" in error for error in self.validate(records)))


if __name__ == "__main__":
    unittest.main()
