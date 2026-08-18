export type TodoTask = {
  line: number;
  raw: string;
  completed: boolean;
  priority?: string;
  creationDate?: string;
  completionDate?: string;
  projects: string[];
  contexts: string[];
  metadata: Record<string, string>;
};

export type TodoDocument = {
  bom: boolean;
  newline: '\n' | '\r\n';
  trailingNewline: boolean;
  lines: string[];
};

const datePattern = /^\d{4}-\d{2}-\d{2}$/;

export const parseTodoDocument = (content: string): TodoDocument => {
  const bom = content.startsWith('\uFEFF');
  const plain = bom ? content.slice(1) : content;
  const newline = plain.includes('\r\n') ? '\r\n' : '\n';
  const trailingNewline = plain.endsWith('\n');
  const lines = plain.length === 0 ? [] : plain.split(/\r?\n/);
  if (trailingNewline) lines.pop();
  return { bom, newline, trailingNewline, lines };
};

export const renderTodoDocument = (document: TodoDocument): string => {
  const body = document.lines.join(document.newline) + (document.trailingNewline && document.lines.length > 0 ? document.newline : '');
  return `${document.bom ? '\uFEFF' : ''}${body}`;
};

export const inspectTodoTasks = (content: string): TodoTask[] => {
  const document = parseTodoDocument(content);
  return document.lines.flatMap((line, index) => (line.trim().length === 0 ? [] : [parseTask(line, index + 1)]));
};

export const addTodoTask = (content: string, task: string): string => {
  const normalized = task.trim();
  if (normalized.length === 0 || /[\r\n]/.test(normalized)) throw new Error('A todo.txt task must be one non-empty line.');
  const document = parseTodoDocument(content);
  document.lines.push(normalized);
  if (document.lines.length === 1 && content.replace(/^\uFEFF/, '').length === 0) document.trailingNewline = true;
  return renderTodoDocument(document);
};

export const completeTodoTask = (content: string, line: number, completionDate: string): string => {
  if (!datePattern.test(completionDate)) throw new Error('Completion date must use YYYY-MM-DD.');
  const document = parseTodoDocument(content);
  const index = line - 1;
  const current = document.lines[index];
  if (current === undefined) throw new Error(`todo.txt line ${line} does not exist.`);
  if (/^x\s/.test(current)) return content;

  const priority = current.match(/^\(([A-Z])\)\s+/)?.[1];
  const withoutPriority = priority === undefined ? current : current.replace(/^\([A-Z]\)\s+/, '');
  document.lines[index] = `x ${completionDate} ${withoutPriority}${priority === undefined ? '' : ` pri:${priority}`}`;
  return renderTodoDocument(document);
};

export const moveTodoTask = (sourceContent: string, targetContent: string, line: number): { source: string; target: string } => {
  const source = parseTodoDocument(sourceContent);
  const target = parseTodoDocument(targetContent);
  const index = line - 1;
  const [task] = source.lines.splice(index, 1);
  if (task === undefined) throw new Error(`todo.txt line ${line} does not exist.`);
  target.lines.push(task);
  if (target.lines.length === 1 && targetContent.replace(/^\uFEFF/, '').length === 0) target.trailingNewline = true;
  return { source: renderTodoDocument(source), target: renderTodoDocument(target) };
};

const parseTask = (raw: string, line: number): TodoTask => {
  let remainder = raw;
  let completed = false;
  let completionDate: string | undefined;
  let priority: string | undefined;
  let creationDate: string | undefined;

  const completedMatch = remainder.match(/^x\s+(\d{4}-\d{2}-\d{2})\s+/);
  if (completedMatch !== null) {
    completed = true;
    completionDate = completedMatch[1];
    remainder = remainder.slice(completedMatch[0].length);
  } else {
    const priorityMatch = remainder.match(/^\(([A-Z])\)\s+/);
    if (priorityMatch !== null) {
      priority = priorityMatch[1];
      remainder = remainder.slice(priorityMatch[0].length);
    }
  }

  const creationMatch = remainder.match(/^(\d{4}-\d{2}-\d{2})\s+/);
  if (creationMatch !== null) creationDate = creationMatch[1];

  const tokens = raw.split(/\s+/);
  const projects = tokens.filter((token) => /^\+\S+$/.test(token)).map((token) => token.slice(1));
  const contexts = tokens.filter((token) => /^@\S+$/.test(token)).map((token) => token.slice(1));
  const metadata = Object.fromEntries(
    tokens.flatMap((token) => {
      const match = token.match(/^([^:\s]+):([^:\s]+)$/);
      return match === null ? [] : [[match[1]!, match[2]!] as const];
    }),
  );

  return {
    line,
    raw,
    completed,
    ...(priority === undefined ? {} : { priority }),
    ...(creationDate === undefined ? {} : { creationDate }),
    ...(completionDate === undefined ? {} : { completionDate }),
    projects,
    contexts,
    metadata,
  };
};
