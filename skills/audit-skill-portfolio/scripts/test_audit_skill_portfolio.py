#!/usr/bin/env python3
"""Unit tests for audit_skill_portfolio.py."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import audit_skill_portfolio as audit


class AuditSkillPortfolioTests(unittest.TestCase):
    def test_parse_folded_frontmatter_and_explicit_policy(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            skill_dir = Path(directory) / "sample-skill"
            (skill_dir / "agents").mkdir(parents=True)
            skill_file = skill_dir / "SKILL.md"
            skill_file.write_text(
                "---\nname: sample-skill\ndescription: >\n  Audit one thing.\n  Keep it safe.\n---\n# Sample\n",
                encoding="utf-8",
            )
            (skill_dir / "agents" / "openai.yaml").write_text(
                "policy:\n  allow_implicit_invocation: false\n", encoding="utf-8"
            )

            self.assertEqual(
                audit.parse_frontmatter(skill_file),
                ("sample-skill", "Audit one thing. Keep it safe."),
            )
            self.assertEqual(audit.parse_invocation_policy(skill_file), "explicit")

    def test_invalid_invocation_policy_is_unknown(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            skill_dir = Path(directory) / "sample-skill"
            (skill_dir / "agents").mkdir(parents=True)
            skill_file = skill_dir / "SKILL.md"
            skill_file.write_text(
                "---\nname: sample-skill\ndescription: Sample.\n---\n",
                encoding="utf-8",
            )
            (skill_dir / "agents" / "openai.yaml").write_text(
                "policy:\n  allow_implicit_invocation: maybe\n", encoding="utf-8"
            )

            self.assertEqual(audit.parse_invocation_policy(skill_file), "unknown")

    def test_non_boolean_config_enabled_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "config.toml"
            config.write_text(
                '[[skills.config]]\nname = "sample"\nenabled = "false"\n',
                encoding="utf-8",
            )

            with self.assertRaisesRegex(ValueError, "enabled must be boolean"):
                audit.load_config_rules(config)

    def test_session_index_supports_plugin_prefixed_names(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "skills"
            skill_file = root / "example" / "SKILL.md"
            skill_file.parent.mkdir(parents=True)
            skill_file.write_text(
                "---\nname: plugin:example\ndescription: Full description here.\n---\n",
                encoding="utf-8",
            )
            block = (
                "<skills_instructions>\n"
                f"- `r0` = `{root.as_posix()}`\n"
                "- plugin:example: Full desc (file: r0/example/SKILL.md)\n"
                "</skills_instructions>"
            )
            session = Path(directory) / "session.jsonl"
            session.write_text(
                json.dumps(
                    {"payload": {"content": [{"type": "text", "text": block}]}}
                )
                + "\n",
                encoding="utf-8",
            )

            visible = audit.parse_session_index(session)

            self.assertEqual(len(visible), 1)
            self.assertEqual(next(iter(visible.values())), "Full desc")

    def test_inventory_marks_truncation_and_duplicate_names(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            paths: list[Path] = []
            for parent in ("one", "two"):
                skill_dir = Path(directory) / parent
                skill_dir.mkdir()
                skill_file = skill_dir / "SKILL.md"
                skill_file.write_text(
                    "---\nname: duplicate-skill\n"
                    "description: A deliberately long description for truncation.\n---\n",
                    encoding="utf-8",
                )
                paths.append(skill_file)
            visible = {
                str(paths[0].absolute()).lower(): "A deliberately long"
            }
            normalized_visible = {
                audit.os.path.normcase(key): value for key, value in visible.items()
            }

            records = audit.build_inventory(
                paths, normalized_visible, [], description_budget=20
            )

            self.assertEqual(records[0].duplicate_name_count, 2)
            self.assertTrue(any(record.truncated_in_session for record in records))
            self.assertTrue(
                all("duplicate-name:2" in record.flags for record in records)
            )

    def test_path_config_rule_overrides_name_rule_and_reports_conflict(self) -> None:
        path = str(Path("C:/skills/sample/SKILL.md"))
        normalized = audit.os.path.normcase(audit.os.path.abspath(path))
        rules = [
            audit.ConfigRule("name", "sample", False, 0),
            audit.ConfigRule("path", normalized, True, 1),
        ]

        enabled, source, matches, conflict = audit.resolve_config("sample", normalized, rules)

        self.assertTrue(enabled)
        self.assertEqual(source, "config-path")
        self.assertEqual(len(matches), 2)
        self.assertTrue(conflict)

    def test_collision_report_distinguishes_exact_and_heuristic(self) -> None:
        def record(name: str, description: str) -> audit.SkillRecord:
            return audit.SkillRecord(
                name=name,
                description=description,
                path=f"C:/skills/{name}/SKILL.md",
                source="user",
                scope="global",
                invocation="default-implicit",
                enabled=True,
                config_decision_source="default-enabled",
                config_matches=[],
                in_session_index=None,
                visible_description=None,
                description_chars=len(description),
                description_words=len(description.split()),
                approximate_description_tokens=10,
                truncated_in_session=False,
                recommended_tier="global-implicit",
                recommendation_confidence="low",
                recommendation_reason="test",
            )

        records = [
            record("diagnose", "Debug failing software with evidence and hypotheses."),
            record("systematic-debugging", "Debug failing software with evidence and hypotheses."),
        ]

        collisions = audit.detect_collisions(records, threshold=0.55, limit=20)

        self.assertEqual(collisions[0]["type"], "exact-description")
        self.assertFalse(collisions[0]["heuristic"])
        self.assertEqual(records[0].collision_count, 1)

    def test_invalid_skill_is_isolated_and_reported_as_diagnostic(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "skills"
            valid = root / "valid" / "SKILL.md"
            invalid = root / "invalid" / "SKILL.md"
            valid.parent.mkdir(parents=True)
            invalid.parent.mkdir(parents=True)
            valid.write_text(
                "---\nname: valid\ndescription: A valid audit fixture.\n---\n",
                encoding="utf-8",
            )
            invalid.write_text(
                "---\nname: invalid\n---\n",
                encoding="utf-8",
            )
            config = Path(directory) / "config.toml"
            config.write_text("", encoding="utf-8")
            output = Path(directory) / "report.json"

            exit_code = audit.main(
                [
                    "--root", str(root),
                    "--config", str(config),
                    "--format", "json",
                    "--output", str(output),
                ]
            )
            report = json.loads(output.read_text(encoding="utf-8"))

            self.assertEqual(exit_code, 0)
            self.assertFalse(report["method"]["network_used"])
            self.assertEqual(report["summary"]["skills_total"], 2)
            self.assertTrue(
                any(item.startswith("invalid-skill:") for item in report["diagnostics"])
            )
            self.assertEqual(
                next(skill for skill in report["skills"] if skill["name"] == "invalid")["enabled"],
                False,
            )


if __name__ == "__main__":
    unittest.main()

