# Purpose

Define how the Arcantry brand is expressed structurally in the documentation interface while Starlight remains the underlying documentation engine.

# Requirements

## Documentation shell matches the approved Arcantry concept

The desktop documentation shell MUST present a full-height left navigation rail, an independent top content bar, and an editorial main content area matching the approved Arcantry documentation concept rather than Starlight's default shell.

## Brand assets are structural UI

The Arcantry mark and wordmark MUST be used as primary navigation branding rather than decorative content inside the page body.

## Overview follows the approved composition

The overview MUST use the approved composition: serif-led hero, restrained primary action, six compact navigation cards, and a two-column lifecycle explanation.

## Starlight remains infrastructure

Routing, content collections, search, accessibility behavior, and mobile navigation MAY remain powered by Starlight, but default Starlight visual conventions MUST NOT dominate the rendered interface.

## Responsive behavior preserves hierarchy

On narrow screens the desktop rail MAY collapse, but brand, search, primary navigation, and content hierarchy MUST remain accessible without horizontal overflow.

## Theme state is functional

The documentation MUST provide distinct light and dark semantic palettes. Changing the Starlight theme state MUST visibly update background, text, border, control and surface colors without relying on page reloads.

## Components consume semantic tokens

Documentation components MUST use semantic theme variables for colors that differ by theme. Hardcoded light-only foreground and background colors MUST NOT be used in reusable chrome or overview components.

## Brand remains legible in both themes

The canonical Arcantry mark and wordmark MUST remain visually legible in light and dark themes without substituting alternate brand geometry.

## Visual language remains restrained

The documentation MUST prefer flat surfaces, hairline separators and square-to-subtle corner treatment. Glass blur, glow, decorative gradients, oversized pill controls, gratuitous shadows and bento-style filler containers MUST NOT be introduced unless an approved concept explicitly calls for them.
