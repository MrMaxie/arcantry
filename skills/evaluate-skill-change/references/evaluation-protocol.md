# Evaluation Protocol

## Case schema

Each JSONL record contains:

- `id`: unique stable identifier;
- `kind`: `direct`, `indirect`, `negative`, or `behavior`;
- `prompt`: synthetic raw user request;
- `expected_route`: expected skill name or `none`;
- `rubric`: non-empty list of observable requirements;
- `holdout`: boolean;
- `scope`: `global`, `project`, or `local`.

Do not place raw transcripts, secrets, private URLs, credentials, personal data, or unrelated local identifiers in cases.

## Minimum set

- 2 direct cases;
- 2 indirect cases;
- 3 negative or confusable cases;
- 2 behavior or edge cases;
- at least 3 held-out cases overall.

## Metrics

- **Routing precision:** selected positive cases divided by all cases where the skill was selected.
- **Routing recall:** selected positive cases divided by all direct and indirect positive cases.
- **Behavior pass rate:** cases satisfying every applicable rubric item.
- **Critical regressions:** new privacy, authorization, destructive-action, scope, or secret-handling failures.
- **Context delta:** candidate body words and lines relative to baseline.

## Gate

- `accept`: fixes at least one baseline failure, has no critical regression, and does not reduce held-out behavior.
- `reject`: no demonstrated improvement, any critical regression, or any held-out regression.
- `needs-review`: execution evidence is incomplete, a material tradeoff is not scoreable, or unexplained body growth exceeds 10 percent.

Use failed and rejected cases as negative evidence in later revisions. Do not append them automatically to a live skill.

