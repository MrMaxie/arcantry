## ADDED Requirements

### Requirement: Human output is terminal-safe

Human-readable CLI output MUST preserve printable text, line feeds and tabs while rendering all other control characters visibly. JSON and MCP values MUST preserve the original repository value.

#### Scenario: Repository text contains terminal controls

- **WHEN** next, explain or todo output includes ESC, OSC, BEL, DEL, C1 or bare carriage return characters
- **THEN** human stdout contains no raw terminal control sequence
- **AND** structured output retains the original value
