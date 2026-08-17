export const managedSectionStart = '<!-- arcantry:start -->';
export const managedSectionEnd = '<!-- arcantry:end -->';

export type ManagedSectionResult =
  | { status: 'unchanged'; content: string }
  | { status: 'changed'; content: string }
  | { status: 'conflict'; content: string; reason: string };

export const renderManagedSection = (body: string): string => `${managedSectionStart}\n${body.trim()}\n${managedSectionEnd}`;

export const upsertManagedSection = (existing: string, body: string): ManagedSectionResult => {
  const desired = renderManagedSection(body);
  const startIndexes = findAll(existing, managedSectionStart);
  const endIndexes = findAll(existing, managedSectionEnd);

  if (startIndexes.length === 0 && endIndexes.length === 0) {
    const prefix = existing.length === 0 ? '' : existing.endsWith('\n') ? '\n' : '\n\n';
    return { status: 'changed', content: `${existing}${prefix}${desired}\n` };
  }

  if (startIndexes.length !== 1 || endIndexes.length !== 1 || startIndexes[0] > endIndexes[0]) {
    return { status: 'conflict', content: existing, reason: 'Arcantry section markers are incomplete or duplicated.' };
  }

  const sectionEnd = endIndexes[0] + managedSectionEnd.length;
  const current = existing.slice(startIndexes[0], sectionEnd);
  if (current === desired) {
    return { status: 'unchanged', content: existing };
  }

  return {
    status: 'changed',
    content: `${existing.slice(0, startIndexes[0])}${desired}${existing.slice(sectionEnd)}`,
  };
};

export const removeManagedSection = (existing: string): ManagedSectionResult => {
  const startIndexes = findAll(existing, managedSectionStart);
  const endIndexes = findAll(existing, managedSectionEnd);

  if (startIndexes.length === 0 && endIndexes.length === 0) {
    return { status: 'unchanged', content: existing };
  }

  if (startIndexes.length !== 1 || endIndexes.length !== 1 || startIndexes[0] > endIndexes[0]) {
    return { status: 'conflict', content: existing, reason: 'Arcantry section markers are incomplete or duplicated.' };
  }

  const sectionEnd = endIndexes[0] + managedSectionEnd.length;
  const before = existing.slice(0, startIndexes[0]).replace(/\n+$/, '');
  const after = existing.slice(sectionEnd).replace(/^\n+/, '');
  const content = [before, after].filter((part) => part.length > 0).join('\n\n');

  return { status: 'changed', content: content.length > 0 ? `${content}\n` : '' };
};

export const containsManagedSection = (content: string): boolean =>
  content.includes(managedSectionStart) && content.includes(managedSectionEnd);

const findAll = (content: string, marker: string): number[] => {
  const indexes: number[] = [];
  let fromIndex = 0;
  while (fromIndex < content.length) {
    const index = content.indexOf(marker, fromIndex);
    if (index === -1) {
      break;
    }
    indexes.push(index);
    fromIndex = index + marker.length;
  }
  return indexes;
};
