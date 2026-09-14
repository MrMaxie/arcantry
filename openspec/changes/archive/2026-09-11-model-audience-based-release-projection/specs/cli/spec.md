## ADDED Requirements

### Requirement: Release commands use one final adapter contract

Release planning, rendering, checking and cutting MUST use `openspec-release@1` for every supported topology. Commands MUST preserve manifest assignment and SemVer impact for changes classified with `changelog: omit`, and rendering MUST use group-owned prose for valid projection groups.

#### Scenario: Maintenance is omitted from prose

- **WHEN** an accepted maintenance change uses `changelog: omit`
- **THEN** release planning still assigns it and includes its SemVer impact
- **AND** release rendering emits no entry for it

#### Scenario: A group owns a release story

- **WHEN** all group members are assigned to one release
- **THEN** rendering emits the group's title and body once
- **AND** preserves source markers for every member
