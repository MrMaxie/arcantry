import { describe, expect, it } from 'vitest';
import { addTodoTask, completeTodoTask, inspectTodoTasks, moveTodoTask, parseTodoDocument } from './todoTxt.js';

describe('todo.txt adapter', () => {
  it('preserves BOM, CRLF, arbitrary tags and metadata', () => {
    const content = '\uFEFF(A) 2026-08-18 Ship +Arcantry @digitalroom owner:maxie\r\nFree form @desk\r\n';

    const tasks = inspectTodoTasks(content);
    const updated = addTodoTask(content, 'Review +Arcantry @home custom:value');

    expect(tasks[0]).toMatchObject({
      priority: 'A',
      creationDate: '2026-08-18',
      projects: ['Arcantry'],
      contexts: ['digitalroom'],
      metadata: { owner: 'maxie' },
    });
    expect(parseTodoDocument(updated)).toMatchObject({ bom: true, newline: '\r\n', trailingNewline: true });
    expect(updated).toContain('Review +Arcantry @home custom:value\r\n');
  });

  it('completes a task using standard order and retains priority as metadata', () => {
    expect(completeTodoTask('(B) 2026-08-17 Verify +Arcantry @desk\n', 1, '2026-08-18')).toBe(
      'x 2026-08-18 2026-08-17 Verify +Arcantry @desk pri:B\n',
    );
  });

  it('moves one raw task without normalizing either document', () => {
    expect(moveTodoTask('First\r\nSecond @desk\r\n', '\uFEFFPrivate\r\n', 2)).toEqual({
      source: 'First\r\n',
      target: '\uFEFFPrivate\r\nSecond @desk\r\n',
    });
  });

  it('rejects multiline tasks and missing selectors', () => {
    expect(() => addTodoTask('', 'one\ntwo')).toThrow('one non-empty line');
    expect(() => completeTodoTask('one\n', 2, '2026-08-18')).toThrow('does not exist');
  });
});
