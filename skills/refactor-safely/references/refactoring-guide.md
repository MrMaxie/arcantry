# Refactoring guide

[Refactoring.Guru's refactoring workflow](https://refactoring.guru/refactoring/how-to) defines refactoring as small, controlled structural change that preserves behavior and relies on tests. Apply that discipline through the target project's own language and tools.

Choose transformations that have one observable purpose, such as moving one responsibility, replacing duplicated knowledge with one owner, shortening a dependency path or making a side effect explicit. After each transformation:

1. compile or type-check the affected boundary;
2. run the closest behavioral tests;
3. inspect public signatures and serialized forms;
4. review the diff for feature work or unrelated cleanup;
5. continue only from a passing state.

If a test reveals an existing product defect, record it separately. A refactor does not silently decide its intended replacement behavior.
