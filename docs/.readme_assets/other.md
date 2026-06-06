### Design system (Figma)
The visual language is mirrored in a Figma file
([`Landing`](https://www.figma.com/design/e0V2P1cQpEFRuXTeNtEMh6/Landing)), kept
**code-first**: [`frontend/application/styles/globals.css`](../../frontend/application/styles/globals.css)
is the source of truth and the Figma side is conformed to it, never the reverse.

- **Tokens → Figma Variables.** Three collections mirror the CSS tokens 1:1, each with
  `var(--token)` code syntax so Dev Mode round-trips cleanly:
  - `ev/color` — brand primitives (`main-*`) + neutrals,
  - `ev/semantic` — shadcn roles (`background`, `foreground`, `primary`, `border`, …)
    aliased onto the primitives,
  - `ev/radius` — the `--radius` scale.
- **Components.** The shadcn `bricks` are rebuilt as Figma variant-sets bound to those
  Variables (Button, Badge, Input/Field, Checkbox, Switch, Card, Select, Tabs, Accordion,
  Tooltip), on a dark surface matching the app's navy theme. Fonts: Inter (sans) +
  Playfair (display).

> Code Connect (the design↔code component mapping) is intentionally **not** wired up: it
> requires a Figma Organization/Enterprise plan and the file is on Pro. Tokens and
> variant-sets work regardless.
