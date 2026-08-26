## ADDED Requirements

### Requirement: Context discovery is complete and bounded

Repository context discovery MUST inspect the selected configuration, every supported shared and private standard source location, applicable adapter and management metadata, canonical methodology markers, and `.local` repository-policy health. It MUST explicitly report supported standard sources that are absent. Discovery MUST use bounded named locations and repository metadata and MUST NOT recursively enumerate arbitrary repository or private content.

#### Scenario: A standard source is absent

- **WHEN** context discovery finds no artifact at one supported standard location
- **THEN** it reports the source kind, scope and absent state
- **AND** does not create the source or treat absence as invalid adoption

#### Scenario: Private boundary health is inspected

- **WHEN** Git is available and `.local` is not tracked by the configured default remote branch
- **THEN** discovery reports whether Git's effective exclusion rules protect `.local`
- **AND** does not infer protection solely from one ignore file
