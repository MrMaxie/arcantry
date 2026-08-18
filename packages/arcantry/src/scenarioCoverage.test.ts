import { readFile } from 'node:fs/promises';
import { describe, expect, it } from 'vitest';

const axes = {
  sources: ['none', 'single', 'mixed'],
  footprint: ['none', 'external-config', 'private-config', 'tracked-config'],
  vcs: ['none', 'clean-git', 'dirty-git'],
  management: ['ignore', 'observe', 'validate', 'manage'],
  adapter: ['previous', 'current', 'unsupported'],
  transition: ['preserve', 'adopt', 'rebind', 'cutover', 'migrate', 'relocate'],
  topology: ['project', 'monorepo'],
  visibility: ['shared', 'private'],
} as const;

type Axis = keyof typeof axes;
type ScenarioRow = { [Key in Axis]: (typeof axes)[Key][number] };

const names = Object.keys(axes) as Axis[];
const defaults = Object.fromEntries(names.map((name) => [name, axes[name][0]])) as ScenarioRow;
const pairwiseMatrix: ScenarioRow[] = [];

for (let leftIndex = 0; leftIndex < names.length; leftIndex += 1) {
  for (let rightIndex = leftIndex + 1; rightIndex < names.length; rightIndex += 1) {
    const left = names[leftIndex]!;
    const right = names[rightIndex]!;
    for (const leftValue of axes[left]) {
      for (const rightValue of axes[right]) {
        pairwiseMatrix.push({ ...defaults, [left]: leftValue, [right]: rightValue });
      }
    }
  }
}

const requiredEvidence = [
  ['knowledge.test.ts', 'accepts an empty non-Git project'],
  ['knowledge.test.ts', 'observes a changelog without inventing an OpenSpec authority'],
  ['knowledge.test.ts', 'observes OpenSpec when no changelog exists'],
  ['sourceTransition.test.ts', 'cuts over a configured changelog'],
  ['knowledge.test.ts', 'continues to support the previous changelog adapter family'],
  ['sourceTransition.test.ts', 'blocks ambiguous full changelog migration'],
  ['program.test.ts', 'uses one explicit external configuration'],
  ['program.test.ts', 'requires an explicit queue when both exist'],
  ['projectPlan.test.ts', 'rejects all writes when an input changes'],
  ['projectPlan.test.ts', 'preserves the relocation source when target verification fails'],
  ['sourceTransition.test.ts', 'leaves unrelated local changes untouched'],
  ['program.test.ts', 'lists, inspects, links, diagnoses, and unlinks skills'],
] as const;

describe('project knowledge scenario coverage', () => {
  it('covers every value pair across the normative project axes', () => {
    for (let leftIndex = 0; leftIndex < names.length; leftIndex += 1) {
      for (let rightIndex = leftIndex + 1; rightIndex < names.length; rightIndex += 1) {
        const left = names[leftIndex]!;
        const right = names[rightIndex]!;
        for (const leftValue of axes[left]) for (const rightValue of axes[right]) {
          expect(pairwiseMatrix.some((row) => row[left] === leftValue && row[right] === rightValue)).toBe(true);
        }
      }
    }
  });

  it('keeps executable evidence for every mandatory end-to-end scenario', async () => {
    for (const [file, phrase] of requiredEvidence) {
      expect(await readFile(new URL(file, import.meta.url), 'utf8')).toContain(phrase);
    }
  });
});
