import { describe, expect, it } from 'vitest';
import { type CoveragePolicy, verifyCoverage } from './rust-coverage-policy';

const policy: CoveragePolicy = {
  schemaVersion: 1,
  files: {
    'crates/example/src/main.rs': { line: 50, branch: 50 },
  },
  exclusions: {
    'crates/example/src/lib.rs': 'Module exports only.',
  },
};

const report = `SF:/workspace/crates/example/src/main.rs
DA:1,1
DA:2,0
BRDA:1,0,0,1
BRDA:1,0,1,0
end_of_record
`;

describe('Rust coverage policy', () => {
  it('accepts per-file line and branch evidence at the reviewed floors', () => {
    expect(verifyCoverage(report, policy, ['crates/example/src/main.rs', 'crates/example/src/lib.rs'])).toEqual([
      {
        path: 'crates/example/src/main.rs',
        line: { found: 2, hit: 1, percent: 50 },
        branch: { found: 2, hit: 1, percent: 50 },
      },
    ]);
  });

  it('rejects a new production file that is absent from the policy', () => {
    expect(() =>
      verifyCoverage(report, policy, [
        'crates/example/src/main.rs',
        'crates/example/src/lib.rs',
        'crates/example/src/new.rs',
      ]),
    ).toThrow('missing production files: crates/example/src/new.rs');
  });

  it('rejects line coverage below a file floor', () => {
    const strict = structuredClone(policy);
    strict.files['crates/example/src/main.rs'] = { line: 51, branch: 51 };

    expect(() => verifyCoverage(report, strict, ['crates/example/src/main.rs', 'crates/example/src/lib.rs'])).toThrow(
      'line coverage 50.00% is below 51%',
    );
  });

  it('rejects branch coverage below a file floor', () => {
    const strict = structuredClone(policy);
    strict.files['crates/example/src/main.rs'] = { line: 50, branch: 51 };

    expect(() => verifyCoverage(report, strict, ['crates/example/src/main.rs', 'crates/example/src/lib.rs'])).toThrow(
      'branch coverage 50.00% is below 51%',
    );
  });
});
