## ADDED Requirements

### Requirement: Strong dark-theme foregrounds use pure white

In the dark documentation theme, strong text, icons, and primary button fills MUST use pure white. Muted secondary text and decorative surfaces MAY retain softer neutral colors. Existing neutral values used only to compose gradients, glows, shadows, or other decorative effects MUST remain unchanged.

#### Scenario: A reader views an emphasized control in the dark theme

- **WHEN** strong text, an icon, or a primary button is rendered in the dark theme
- **THEN** its strong foreground token resolves to pure white
- **AND** decorative gradients and effects retain their established neutral mixing color
