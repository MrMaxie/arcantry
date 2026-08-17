#!/usr/bin/env python3
"""Audit Codex skill metadata without changing skills or configuration."""

from __future__ import annotations

import argparse
import json
import math
import os
import re
import statistics
import sys
import tomllib
from collections import defaultdict
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Iterable


STYLE_OR_META = re.compile(
    r"(?:adhd|caveman|talk-normal|full-output|agent-self|review-agent|"
    r"skill-portfolio|skill-selector|memory-manage|doubt-driven)",
    re.IGNORECASE,
)
REPO_SPECIFIC = re.compile(
    r"(?:hootlog|voxedit|winui|gpui|surreal|backlogmd|openspec)",
    re.IGNORECASE,
)


@dataclass
class SkillRecord:
    name: str
    description: str
    path: str
    source: str
    scope: str
    invocation: str
    enabled: bool
    config_decision_source: str
    config_matches: list[str]
    in_session_index: bool | None
    visible_description: str | None
    description_chars: int
    description_words: int
    approximate_description_tokens: int
    truncated_in_session: bool
    recommended_tier: str
    recommendation_confidence: str
    recommendation_reason: str
    flags: list[str] = field(default_factory=list)
    duplicate_name_count: int = 1
    collision_count: int = 0


@dataclass(frozen=True)
class ConfigRule:
    selector: str
    value: str
    enabled: bool
    index: int


def default_roots() -> list[Path]:
    home = Path.home()
    return [home / ".codex" / "skills", home / ".agents" / "skills"]


def strip_yaml_scalar(value: str) -> str:
    value = value.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {"'", '"'}:
        return value[1:-1]
    return value


def parse_frontmatter(path: Path) -> tuple[str, str]:
    text = path.read_text(encoding="utf-8-sig")
    match = re.match(r"\A---\s*\r?\n(.*?)\r?\n---(?:\r?\n|\Z)", text, re.DOTALL)
    if not match:
        raise ValueError("missing YAML frontmatter")

    lines = match.group(1).splitlines()
    name = ""
    description = ""
    index = 0
    while index < len(lines):
        line = lines[index]
        if line.startswith("name:"):
            name = strip_yaml_scalar(line.split(":", 1)[1])
        elif line.startswith("description:"):
            value = line.split(":", 1)[1].strip()
            if value in {">", "|", ">-", "|-"}:
                block: list[str] = []
                index += 1
                while index < len(lines) and (not lines[index].strip() or lines[index][0].isspace()):
                    block.append(lines[index].strip())
                    index += 1
                description = (" " if value.startswith(">") else "\n").join(
                    part for part in block if part
                )
                continue
            description = strip_yaml_scalar(value)
        index += 1

    if not name or not description:
        raise ValueError("frontmatter requires name and description")
    return name, description


def parse_invocation_policy(skill_file: Path) -> str:
    metadata = skill_file.parent / "agents" / "openai.yaml"
    if not metadata.is_file():
        return "default-implicit"
    text = metadata.read_text(encoding="utf-8-sig")
    match = re.search(r"(?m)^\s*allow_implicit_invocation\s*:\s*(true|false)\s*$", text)
    if match:
        return "implicit" if match.group(1) == "true" else "explicit"
    if re.search(r"(?m)^\s*allow_implicit_invocation\s*:", text):
        return "unknown"
    return "default-implicit"


def load_config_rules(config_path: Path) -> list[ConfigRule]:
    rules: list[ConfigRule] = []
    if not config_path.is_file():
        return rules
    with config_path.open("rb") as handle:
        data = tomllib.load(handle)
    configured = data.get("skills", {}).get("config", [])
    if not isinstance(configured, list):
        raise ValueError("skills.config must be an array of tables")
    for index, item in enumerate(configured):
        if not isinstance(item, dict):
            raise ValueError(f"skills.config[{index}] must be a table")
        raw_enabled = item.get("enabled", True)
        if not isinstance(raw_enabled, bool):
            raise ValueError(f"skills.config[{index}].enabled must be boolean")
        if name := item.get("name"):
            rules.append(ConfigRule("name", str(name).casefold(), raw_enabled, index))
        if path := item.get("path"):
            rules.append(
                ConfigRule(
                    "path",
                    os.path.normcase(os.path.abspath(os.path.expanduser(str(path)))),
                    raw_enabled,
                    index,
                )
            )
    return rules


def resolve_config(name: str, normalized_path: str, rules: list[ConfigRule]) -> tuple[bool, str, list[str], bool]:
    path_matches = [rule for rule in rules if rule.selector == "path" and rule.value == normalized_path]
    name_matches = [rule for rule in rules if rule.selector == "name" and rule.value == name.casefold()]
    chosen = path_matches or name_matches
    if not chosen:
        return True, "default-enabled", [], False
    enabled = chosen[-1].enabled
    all_matches = sorted(path_matches + name_matches, key=lambda rule: rule.index)
    conflict = len({rule.enabled for rule in all_matches}) > 1
    labels = [f"{rule.selector}[{rule.index}]={str(rule.enabled).lower()}" for rule in all_matches]
    return enabled, f"config-{chosen[-1].selector}", labels, conflict


def walk_skill_files(roots: Iterable[Path]) -> list[Path]:
    results: list[Path] = []
    visited: set[str] = set()
    for root in roots:
        if not root.is_dir():
            continue
        for directory, child_dirs, files in os.walk(root, followlinks=True):
            real_directory = os.path.normcase(os.path.realpath(directory))
            if real_directory in visited:
                child_dirs[:] = []
                continue
            visited.add(real_directory)
            child_dirs.sort(key=str.casefold)
            if "SKILL.md" in files:
                results.append(Path(directory) / "SKILL.md")
    return sorted(results, key=lambda item: os.path.normcase(str(item)))


def parse_session_index(session_path: Path) -> dict[str, str]:
    block = ""
    with session_path.open("r", encoding="utf-8") as handle:
        for line in handle:
            try:
                item = json.loads(line)
            except json.JSONDecodeError:
                continue
            content = item.get("payload", {}).get("content", [])
            text = "\n".join(
                part.get("text", "") for part in content if isinstance(part, dict)
            )
            start = text.find("<skills_instructions>")
            end = text.find("</skills_instructions>")
            if start >= 0 and end > start:
                block = text[start:end]
                break
    if not block:
        raise ValueError("skills_instructions block not found in session")

    roots = {
        match.group("alias"): match.group("path").replace("/", os.sep)
        for match in re.finditer(
            r"(?m)^- `(?P<alias>r\d+)` = `(?P<path>[^`]+)`$", block
        )
    }
    visible: dict[str, str] = {}
    pattern = re.compile(
        r"(?m)^- (?P<name>.+?): (?P<description>.*) "
        r"\(file: (?P<alias>r\d+)/(?P<relative>.*)\)$"
    )
    for match in pattern.finditer(block):
        root = roots.get(match.group("alias"))
        if not root:
            continue
        path = os.path.normcase(
            os.path.abspath(os.path.join(root, match.group("relative").replace("/", os.sep)))
        )
        visible[path] = match.group("description")
    return visible


def classify_source(path: Path) -> str:
    normalized = str(path).replace("\\", "/").casefold()
    if "/.codex/plugins/" in normalized:
        return "plugin"
    if "/.codex/skills/.system/" in normalized:
        return "system"
    if "/.agents/skills/" in normalized and not normalized.startswith(
        str(Path.home()).replace("\\", "/").casefold() + "/.agents/skills/"
    ):
        return "repository"
    return "user"


def recommend_tier(name: str, source: str, invocation: str, enabled: bool) -> tuple[str, str, str]:
    if not enabled:
        return "disabled", "high", "disabled by effective skills.config rule"
    if source == "repository":
        return "repo-local", "high", "discovered under a repository skill root"
    if invocation == "explicit" or STYLE_OR_META.search(name):
        return "global-explicit", "medium", "explicit policy or request-only style/meta heuristic"
    if REPO_SPECIFIC.search(name):
        return "repo-local", "medium", "project or stack-specific name heuristic"
    if source == "system":
        return "global-implicit", "medium", "system skill with implicit availability"
    return "global-implicit", "low", "fallback candidate; frequency and trigger uniqueness require human review"


def build_inventory(
    skill_files: Iterable[Path],
    visible_by_path: dict[str, str] | None,
    config_rules: list[ConfigRule],
    description_budget: int,
) -> list[SkillRecord]:
    records: list[SkillRecord] = []
    for skill_file in skill_files:
        normalized_path = os.path.normcase(os.path.abspath(skill_file))
        try:
            name, description = parse_frontmatter(skill_file)
        except (OSError, UnicodeError, ValueError) as error:
            records.append(
                SkillRecord(
                    name=skill_file.parent.name,
                    description="",
                    path=str(skill_file),
                    source=classify_source(skill_file),
                    scope="global",
                    invocation="unknown",
                    enabled=False,
                    config_decision_source="invalid-skill",
                    config_matches=[],
                    in_session_index=None if visible_by_path is None else normalized_path in visible_by_path,
                    visible_description=None,
                    description_chars=0,
                    description_words=0,
                    approximate_description_tokens=0,
                    truncated_in_session=False,
                    recommended_tier="disabled",
                    recommendation_confidence="high",
                    recommendation_reason="invalid skill metadata",
                    flags=[f"invalid-skill: {error}"],
                )
            )
            continue

        enabled, config_source, config_matches, config_conflict = resolve_config(
            name, normalized_path, config_rules
        )
        policy_error = None
        try:
            invocation = parse_invocation_policy(skill_file)
        except (OSError, UnicodeError) as error:
            invocation = "unknown"
            policy_error = str(error)
        source = classify_source(skill_file)
        scope = "repository" if source == "repository" else "global"
        tier, confidence, recommendation_reason = recommend_tier(name, source, invocation, enabled)
        visible = visible_by_path.get(normalized_path) if visible_by_path is not None else None
        truncated = bool(visible is not None and len(visible) < len(description) and description.startswith(visible))
        flags: list[str] = []
        if len(description) > description_budget:
            flags.append(f"description-over-budget:{len(description)}>{description_budget}")
        if truncated:
            flags.append("description-truncated-in-session")
        if visible_by_path is not None and normalized_path not in visible_by_path:
            flags.append("not-in-session-index")
        if invocation == "default-implicit":
            flags.append("implicit-policy-not-explicitly-declared")
        if invocation == "unknown":
            detail = f": {policy_error}" if policy_error else ""
            flags.append(f"invalid-invocation-policy{detail}")
        if config_conflict:
            flags.append("conflicting-config-rules")

        records.append(
            SkillRecord(
                name=name,
                description=description,
                path=str(skill_file),
                source=source,
                scope=scope,
                invocation=invocation,
                enabled=enabled,
                config_decision_source=config_source,
                config_matches=config_matches,
                in_session_index=None if visible_by_path is None else normalized_path in visible_by_path,
                visible_description=visible,
                description_chars=len(description),
                description_words=len(re.findall(r"\S+", description)),
                approximate_description_tokens=math.ceil(len(description) / 4),
                truncated_in_session=truncated,
                recommended_tier=tier,
                recommendation_confidence=confidence,
                recommendation_reason=recommendation_reason,
                flags=flags,
            )
        )

    counts: dict[str, int] = {}
    for record in records:
        counts[record.name.casefold()] = counts.get(record.name.casefold(), 0) + 1
    for record in records:
        record.duplicate_name_count = counts[record.name.casefold()]
        if record.duplicate_name_count > 1:
            record.flags.append(f"duplicate-name:{record.duplicate_name_count}")
    return sorted(records, key=lambda item: (item.name.casefold(), os.path.normcase(item.path)))


STOP_WORDS = {
    "a", "an", "and", "are", "as", "at", "be", "by", "codex", "for", "from",
    "in", "into", "is", "it", "of", "on", "or", "that", "the", "this", "to",
    "use", "when", "with", "work", "working", "user", "users",
}


def trigger_terms(record: SkillRecord) -> set[str]:
    words = re.findall(r"[a-z0-9][a-z0-9+.#-]{2,}", f"{record.name} {record.description}".casefold())
    return {word for word in words if word not in STOP_WORDS}


def workflow_family(name: str) -> str:
    lower = name.casefold()
    explicit_groups = {
        "ai-writing": {"avoid-ai-writing", "humanizer", "unslop"},
        "debugging": {"diagnose", "systematic-debugging"},
        "browser-control": {"playwright", "playwright-cli"},
        "practices": {"use-common-practices", "propose-best-practices"},
        "skill-discovery": {"find-skills", "skill-installer"},
        "response-style": {"caveman", "i-have-adhd", "talk-normal", "full-output-enforcement"},
        "react-best-practices": {"vercel-react-best-practices", "build-web-apps:react-best-practices"},
    }
    for family, names in explicit_groups.items():
        if lower in names:
            return family
    for prefix, family in (
        ("backlogmd-task-", "backlogmd-workflow"),
        ("openspec-", "openspec-workflow"),
        ("winui-", "winui-workflow"),
        ("google-calendar:", "google-calendar"),
        ("google-drive:", "google-drive"),
        ("product-design:", "product-design"),
    ):
        if lower.startswith(prefix):
            return family
    return ""


def detect_collisions(records: list[SkillRecord], threshold: float, limit: int) -> list[dict[str, object]]:
    collisions: list[dict[str, object]] = []
    terms = [trigger_terms(record) for record in records]
    descriptions: dict[str, list[int]] = defaultdict(list)
    for index, record in enumerate(records):
        descriptions[record.description.casefold()].append(index)

    for left in range(len(records)):
        for right in range(left + 1, len(records)):
            first = records[left]
            second = records[right]
            collision_type = ""
            score = 0.0
            evidence: list[str] = []
            if first.name.casefold() == second.name.casefold():
                collision_type = "exact-name"
                score = 1.0
                evidence = [first.name]
            elif first.description and first.description.casefold() == second.description.casefold():
                collision_type = "exact-description"
                score = 1.0
                evidence = ["identical description"]
            else:
                first_family = workflow_family(first.name)
                second_family = workflow_family(second.name)
                shared = sorted(terms[left] & terms[right])
                denominator = min(len(terms[left]), len(terms[right]))
                score = len(shared) / denominator if denominator else 0.0
                if first_family and first_family == second_family:
                    collision_type = "workflow-family"
                    evidence = [first_family, *shared[:8]]
                elif len(shared) >= 6 and score >= threshold:
                    collision_type = "semantic-trigger-overlap"
                    evidence = shared[:12]
            if not collision_type:
                continue
            collisions.append(
                {
                    "type": collision_type,
                    "left": first.name,
                    "left_path": first.path,
                    "right": second.name,
                    "right_path": second.path,
                    "score": round(score, 3),
                    "evidence": evidence,
                    "heuristic": collision_type in {"workflow-family", "semantic-trigger-overlap"},
                }
            )

    collisions.sort(key=lambda item: (-float(item["score"]), str(item["type"]), str(item["left"]), str(item["right"])))
    collisions = collisions[:limit]
    counts: dict[str, int] = defaultdict(int)
    for collision in collisions:
        counts[str(collision["left_path"])] += 1
        counts[str(collision["right_path"])] += 1
    for record in records:
        record.collision_count = counts[record.path]
        if record.collision_count:
            record.flags.append(f"collision-candidates:{record.collision_count}")
    return collisions


def portfolio_summary(records: list[SkillRecord]) -> dict[str, object]:
    enabled = [record for record in records if record.enabled]
    lengths = [record.description_chars for record in enabled]
    return {
        "skills_total": len(records),
        "skills_enabled": len(enabled),
        "implicit_enabled": sum(
            record.invocation != "explicit" for record in enabled
        ),
        "explicit_enabled": sum(
            record.invocation == "explicit" for record in enabled
        ),
        "descriptions_over_budget": sum(
            any(flag.startswith("description-over-budget:") for flag in record.flags)
            for record in enabled
        ),
        "descriptions_truncated_in_session": sum(
            record.truncated_in_session for record in records
        ),
        "not_in_session_index": sum(
            record.in_session_index is False for record in records
        ),
        "duplicate_names": sorted(
            {record.name for record in records if record.duplicate_name_count > 1},
            key=str.casefold,
        ),
        "description_statistics": {
            "characters_total": sum(lengths),
            "characters_median": statistics.median(lengths) if lengths else 0,
            "characters_max": max(lengths, default=0),
            "approximate_tokens_total": sum(record.approximate_description_tokens for record in enabled),
            "token_estimate_method": "ceil(description_characters / 4)",
        },
        "recommended_tiers": {
            tier: sum(record.recommended_tier == tier for record in records)
            for tier in ("global-implicit", "global-explicit", "repo-local", "disabled")
        },
    }


def markdown_cell(value: object) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ")


def render_markdown(report: dict[str, object]) -> str:
    summary = report["summary"]
    skills = report["skills"]
    collisions = report["collisions"]
    lines = [
        "# Skill portfolio audit",
        "",
        "> Read-only report. Tier recommendations are candidates for human review, not automatic changes.",
        "",
        "## Summary",
        "",
        f"- Skills: {summary['skills_total']} total, {summary['skills_enabled']} enabled",
        f"- Invocation: {summary['implicit_enabled']} implicit/default, {summary['explicit_enabled']} explicit",
        f"- Description budget: {summary['descriptions_over_budget']} over the configured limit",
        f"- Session evidence: {summary['descriptions_truncated_in_session']} truncated, {summary['not_in_session_index']} not indexed",
        f"- Collision candidates: {len(collisions)}",
        "",
        "## Inventory",
        "",
        "| Skill | Source | Invocation | Enabled | Tier candidate | Chars | Flags |",
        "|---|---|---|---:|---|---:|---|",
    ]
    for skill in skills:
        lines.append(
            "| " + " | ".join(
                markdown_cell(value)
                for value in (
                    skill["name"], skill["source"], skill["invocation"],
                    str(skill["enabled"]).lower(), skill["recommended_tier"],
                    skill["description_chars"], ", ".join(skill["flags"]),
                )
            ) + " |"
        )
    lines.extend(["", "## Collision candidates", ""])
    if not collisions:
        lines.append("No collision candidates met the configured rules.")
    else:
        lines.extend([
            "| Type | Skills | Score | Evidence | Heuristic |",
            "|---|---|---:|---|---:|",
        ])
        for collision in collisions:
            lines.append(
                "| " + " | ".join(
                    markdown_cell(value)
                    for value in (
                        collision["type"], f"{collision['left']} <-> {collision['right']}",
                        collision["score"], ", ".join(collision["evidence"]),
                        str(collision["heuristic"]).lower(),
                    )
                ) + " |"
            )
    lines.extend([
        "",
        "## Diagnostics",
        "",
    ])
    diagnostics = report.get("diagnostics", [])
    if diagnostics:
        lines.extend(f"- {diagnostic}" for diagnostic in diagnostics)
    else:
        lines.append("- No input diagnostics.")
    lines.extend([
        "",
        "## Method and limits",
        "",
        "- Exact-name and exact-description matches are deterministic findings.",
        "- Workflow-family and semantic-trigger-overlap matches are heuristic review candidates.",
        "- Frequency of real use, ownership, and business value cannot be inferred from files alone.",
        "- The script never changes skills, configuration, plugins, or repositories.",
        "",
    ])
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", action="append", type=Path, default=[])
    parser.add_argument("--session", type=Path)
    parser.add_argument("--config", type=Path, default=Path.home() / ".codex" / "config.toml")
    parser.add_argument("--description-budget", type=int, default=110)
    parser.add_argument("--overlap-threshold", type=float, default=0.55)
    parser.add_argument("--max-collisions", type=int, default=200)
    parser.add_argument("--format", choices=("markdown", "json"), default="markdown")
    parser.add_argument("--output", type=Path)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])
    roots = args.root or default_roots()
    diagnostics: list[str] = []
    visible = None
    if args.session:
        try:
            visible = parse_session_index(args.session)
        except (OSError, UnicodeError, ValueError) as error:
            diagnostics.append(f"session-index-unavailable: {error}")
    files = walk_skill_files(roots)
    if visible:
        known = {os.path.normcase(os.path.abspath(path)) for path in files}
        files.extend(
            Path(path) for path in visible if path not in known and Path(path).is_file()
        )
    try:
        config_rules = load_config_rules(args.config)
    except (OSError, ValueError, tomllib.TOMLDecodeError) as error:
        config_rules = []
        diagnostics.append(f"config-unavailable: {error}")
    records = build_inventory(
        files, visible, config_rules, args.description_budget
    )
    for record in records:
        for flag in record.flags:
            if flag.startswith(("invalid-skill:", "invalid-invocation-policy")):
                diagnostics.append(f"{flag} [{record.path}]")
    collisions = detect_collisions(records, args.overlap_threshold, args.max_collisions)
    report = {
        "schema_version": 1,
        "read_only": True,
        "roots": [str(root) for root in roots],
        "session": str(args.session) if args.session else None,
        "method": {
            "description_budget_characters": args.description_budget,
            "description_token_estimate": "ceil(description_characters / 4)",
            "collision_overlap_threshold": args.overlap_threshold,
            "session_index_used": visible is not None,
            "network_used": False,
            "symlink_policy": "follow links, scan each physical directory once, do not retain every alias",
        },
        "diagnostics": diagnostics,
        "summary": portfolio_summary(records),
        "collisions": collisions,
        "skills": [asdict(record) for record in records],
    }
    output = (
        json.dumps(report, ensure_ascii=False, indent=2) + "\n"
        if args.format == "json"
        else render_markdown(report)
    )
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(output, encoding="utf-8")
    else:
        if hasattr(sys.stdout, "reconfigure"):
            sys.stdout.reconfigure(encoding="utf-8")
        sys.stdout.write(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

