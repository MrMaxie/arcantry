export interface CoverageFloor {
  line: number;
  branch: number | null;
}

export interface CoveragePolicy {
  schemaVersion: number;
  files: Record<string, CoverageFloor>;
  exclusions: Record<string, string>;
}

export interface CoverageSummary {
  path: string;
  line: { found: number; hit: number; percent: number };
  branch: { found: number; hit: number; percent: number | null };
}

interface MutableCoverage {
  lines: Map<number, number>;
  branches: Map<string, number>;
}

const normalize = (path: string) => path.replaceAll('\\', '/').replace(/^\.\//u, '');
const percent = (hit: number, found: number) => (found === 0 ? null : (hit / found) * 100);

export function verifyCoverage(report: string, policy: CoveragePolicy, productionFiles: string[]): CoverageSummary[] {
  if (policy.schemaVersion !== 1) throw new Error('Rust coverage policy schemaVersion must be 1.');
  const files = [...new Set(productionFiles.map(normalize))].sort();
  const policyPaths = Object.keys(policy.files).map(normalize);
  const exclusionPaths = Object.keys(policy.exclusions).map(normalize);
  const duplicate = policyPaths.find((path) => exclusionPaths.includes(path));
  if (duplicate) throw new Error(`Rust coverage policy lists ${duplicate} as both covered and excluded.`);

  const uninventoried = files.filter(
    (path) => !Object.hasOwn(policy.files, path) && !Object.hasOwn(policy.exclusions, path),
  );
  if (uninventoried.length > 0) {
    throw new Error(`Rust coverage policy is missing production files: ${uninventoried.join(', ')}`);
  }
  const stale = [...policyPaths, ...exclusionPaths].filter((path) => !files.includes(path));
  if (stale.length > 0) throw new Error(`Rust coverage policy contains stale files: ${stale.join(', ')}`);
  for (const [path, reason] of Object.entries(policy.exclusions)) {
    if (reason.trim().length === 0) throw new Error(`Rust coverage exclusion ${path} requires a reason.`);
  }

  const records = parseLcov(report);
  const summaries = [] as CoverageSummary[];
  const failures = [] as string[];
  for (const [path, floor] of Object.entries(policy.files)) {
    validateFloor(path, floor);
    const record = records.get(normalize(path));
    if (!record) {
      failures.push(`${path}: missing from coverage report`);
      continue;
    }
    const lineFound = record.lines.size;
    const lineHit = [...record.lines.values()].filter((count) => count > 0).length;
    const branchFound = record.branches.size;
    const branchHit = [...record.branches.values()].filter((count) => count > 0).length;
    const linePercent = percent(lineHit, lineFound) ?? 0;
    const branchPercent = percent(branchHit, branchFound);
    if (lineFound === 0 || lineHit === 0) failures.push(`${path}: no executed line evidence`);
    if (linePercent + Number.EPSILON < floor.line) {
      failures.push(`${path}: line coverage ${linePercent.toFixed(2)}% is below ${floor.line}%`);
    }
    if (branchFound > 0 && floor.branch === null) {
      failures.push(`${path}: branch records exist but the policy has no branch floor`);
    } else if (branchFound === 0 && floor.branch !== null) {
      failures.push(`${path}: branch floor requires branch records`);
    } else if (branchPercent !== null && floor.branch !== null && branchPercent + Number.EPSILON < floor.branch) {
      failures.push(`${path}: branch coverage ${branchPercent.toFixed(2)}% is below ${floor.branch}%`);
    }
    summaries.push({
      path,
      line: { found: lineFound, hit: lineHit, percent: linePercent },
      branch: { found: branchFound, hit: branchHit, percent: branchPercent },
    });
  }
  if (failures.length > 0) throw new Error(`Rust coverage policy failed:\n- ${failures.join('\n- ')}`);
  return summaries.sort((left, right) => left.path.localeCompare(right.path));
}

function validateFloor(path: string, floor: CoverageFloor) {
  for (const [kind, value] of [
    ['line', floor.line],
    ['branch', floor.branch],
  ] as const) {
    if (value !== null && (!Number.isFinite(value) || value < 0 || value > 100)) {
      throw new Error(`${path}: ${kind} floor must be between 0 and 100.`);
    }
  }
}

function parseLcov(report: string): Map<string, MutableCoverage> {
  const records = new Map<string, MutableCoverage>();
  let current: MutableCoverage | undefined;
  for (const line of report.split(/\r?\n/u)) {
    if (line.startsWith('SF:')) {
      const path = normalizeSourcePath(line.slice(3));
      current = records.get(path) ?? { lines: new Map(), branches: new Map() };
      records.set(path, current);
    } else if (current && line.startsWith('DA:')) {
      const [lineNumber, count] = line.slice(3).split(',').map(Number);
      current.lines.set(lineNumber, Math.max(current.lines.get(lineNumber) ?? 0, count));
    } else if (current && line.startsWith('BRDA:')) {
      const [lineNumber, block, branch, rawTaken] = line.slice(5).split(',');
      const taken = rawTaken === '-' ? 0 : Number(rawTaken);
      const key = `${lineNumber},${block},${branch}`;
      current.branches.set(key, Math.max(current.branches.get(key) ?? 0, taken));
    } else if (line === 'end_of_record') {
      current = undefined;
    }
  }
  return records;
}

function normalizeSourcePath(path: string): string {
  const normalized = normalize(path);
  for (const marker of ['/crates/', '/xtask/']) {
    const index = normalized.lastIndexOf(marker);
    if (index >= 0) return normalized.slice(index + 1);
  }
  return normalized;
}
