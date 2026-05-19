# Responsive breakpoints & cross-device color — design principles

How to reason about supporting many screen sizes without producing the typical "endless snake of unreadable oversized blocks" that amateur sites devolve into on mobile, and how to think about color consistency across devices.

## Breakpoints — the right mental model

**Mobile-first, additive.** Write base styles for the smallest screen, then `@media (min-width: ...)` rules only *add* — never override. Tailwind enforces this: `text-sm md:text-base lg:text-lg` reads "small by default, bigger when there's room." The base case is the most-constrained one. The discipline: you don't *grow* content as the screen shrinks, you *strip* the optional bits off the top.

**Priority tiers are the generalizable infrastructure.** Tag every element with a priority — P0 (must always show), P1 (show on tablet+), P2 (laptop+), P3 (chrome/decoration). One tiny set of utility classes implements the discarding once, then any new element just declares its tier:

```css
.p1-up { display: none } @media (min-width: 48rem) { .p1-up { display: revert } }
.p2-up { display: none } @media (min-width: 64rem) { .p2-up { display: revert } }
.p3-up { display: none } @media (min-width: 80rem) { .p3-up { display: revert } }
```

Now adding a new element is a one-decision question: what priority? The "snake of unreadable blocks" failure happens precisely when this question is never asked — designers ship the desktop content tree verbatim and just narrow the grid.

**Three layout transformations to plan for**, in order of importance:

1. **Multi-column → single column.** Card grids (`repeat(3, 1fr)` → `1fr`). Pills wrap naturally; form rows switch `flex-direction: row` → `column`.
2. **Nav collapse.** Inline link list drops to a hamburger or is hidden; logo stays.
3. **Typography fluidity.** Instead of stepped font-size breakpoints, use `clamp()`: `font-size: clamp(1.75rem, 4vw + 1rem, 2.75rem)`. This is the single best trick in responsive design and synergizes perfectly with rem units — one declaration replaces three breakpoint overrides.

**Container queries > media queries** for component-level decisions. `@container (min-width: 30rem)` lets a card decide its own layout based on its parent's width, not the viewport's. So if you drop the same card component into a sidebar later, it adapts without rewriting. This is the most compositional, reuse-friendly primitive — the "generalizable infrastructure" idea applied at the component level. Tailwind has `@container` support; modern browsers all support container queries now.

**Pick few breakpoints.** Two is usually enough: one "phone↔tablet" (~`48rem` / 768px) and one "tablet↔desktop" (~`64rem` / 1024px). Tailwind ships five (sm/md/lg/xl/2xl); use whatever subset you need, don't feel obliged to fill them all.

## Color across devices — what's actually true

**You cannot match exactly.** Different panels (OLED phone vs IPS laptop vs old TN monitor), different color profiles (Display P3 on Apple, sRGB elsewhere, varying Android), different ambient light, user-configured night modes — none of this is observable from JS/CSS and most isn't tweakable from your end. Professionals don't try to compensate per-device; they aim for *internal consistency on each device*, which is what users actually perceive. Users rarely A/B their phone vs laptop side-by-side; they want the page to hang together in one session.

**What OKLCH does for you.** It's perceptually uniform: `oklch(0.65 0.15 30)` (orange) and `oklch(0.65 0.15 240)` (blue) look about equally bright, unlike HSL where blue-at-50 looks far darker than yellow-at-50. This means:

- Pick all your semantic colors at the same L and they auto-balance visually.
- Generate variants algorithmically: `--brand` at `L=0.62`, `--brand-hi` at `L=0.72`, `--brand-lo` at `L=0.50`, all sharing C and H. No artist needed for hover/active states.
- Contrast math actually works: APCA / WCAG3 contrast calculators speak this language.

So OKLCH gives you *intra-palette* harmony — rely on it for that. It does **not** make different devices show the same color.

**The one knob worth building** if you want a single point of color-tuning: separate L/C/H into custom properties so chroma becomes adjustable.

```css
:root {
  --brand-l: 0.62; --brand-c: 0.18; --brand-h: 28;
  --chroma-scale: 1;
  --brand: oklch(var(--brand-l) calc(var(--brand-c) * var(--chroma-scale)) var(--brand-h));
}
@media (prefers-contrast: more) { :root { --chroma-scale: 0.85 } }
@media (color-gamut: p3)        { /* nothing to do — browser already uses wider gamut */ }
```

One scalar (`--chroma-scale`) and you can damp saturation globally for any condition you care about. Build this only if you actually observe a problem; for most well-designed sites it's overkill.

**Media features worth knowing** (real opt-in user signals, not guesses):

- `prefers-color-scheme: dark|light` — user's OS theme
- `prefers-contrast: more|less|no-preference` — accessibility setting
- `prefers-reduced-motion` — kill non-essential animation
- `color-gamut: p3|rec2020` — display is wider than sRGB; can drop in `color(display-p3 …)` swatches with graceful fallback
- `dynamic-range: high` — HDR display

These are the right hooks to react to; viewport width is a poor proxy for any of them.

**Net recommendation on color**: trust OKLCH for internal palette consistency, run contrast checks at design time (any OKLCH contrast tool — APCA recommended), don't try to "match across devices", and only add the `--chroma-scale` knob the day someone reports a real issue.
