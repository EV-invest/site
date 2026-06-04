# `vietnam_site/` — original colorscheme snapshot

Captured verbatim from `vietnam_site/client/src/index.css` (the reinstated
original site) on 2026-05-30, before any navy-led rework. This is the
reference for "the vibrant-yet-cultured Vietnam feel" we're trying to recover:
a neutral charcoal base, warm ivory text, **Rice Gold as the lead brand color**,
and the rest of the Vietnam palette used as careful, sparse accents.

The OKLCH columns are computed by `tmp/colorscheme_vietnam/hex_to_oklch.py`.

## Vietnam palette (the `@theme inline` custom colors)

| name           | hex       |     L |     C |     H | role in the original site |
|----------------|-----------|------:|------:|------:|---------------------------|
| Ink Black      | `#171717` | 0.205 | 0.000 |  89.9 | page background           |
| Limestone Mist | `#E6E1D3` | 0.910 | 0.020 |  90.6 | body text (warm ivory)    |
| Rice Gold      | `#F2C94C` | 0.850 | 0.146 |  90.5 | **lead accent / `--primary`** — eyebrows, hero word, CTAs, links, most "look here" numbers |
| Ha Long Teal   | `#2A9D8F` | 0.630 | 0.101 | 183.0 | secondary workhorse accent — yields, FDI figures, research labels |
| Lotus Pink     | `#E58AAE` | 0.739 | 0.118 | 355.1 | **the rare jewel** — used exactly 3×: hero "100%" stat, team "Institutional Pioneers", one trust badge |
| Jungle Green   | `#1F6F43` | 0.482 | 0.104 | 154.8 | research-section "Insights" word (light theme) |
| Flag Red       | `#DA251D` | 0.572 | 0.215 |  28.8 | destructive / status only |
| Lacquer Brown  | `#6B3F2A` | 0.415 | 0.070 |  45.8 | **the misuse** — filled an entire research block; the one large structural use that broke trust |

## Neutrals (`:root`, dark theme)

| token            | hex       |     L |     C |     H |
|------------------|-----------|------:|------:|------:|
| `--background`   | `#171717` | 0.205 | 0.000 |  89.9 |
| `--card`         | `#1f1f1f` | 0.239 | 0.000 |  89.9 |
| `--muted`/border | `#2d2d2d` | 0.297 | 0.000 |  89.9 |
| `--muted-foreground` | `#a39f95` | 0.703 | 0.015 | 88.7 |
| `--foreground`   | `#E6E1D3` | 0.910 | 0.020 |  90.6 |

Note the neutral ramp is **pure grey** (C≈0); the H=90 is meaningless at zero
chroma. The warmth in the original comes entirely from the ivory text and the
gold/pink accents, not from a tinted background.

## Light "academic" theme (research section)

Was an ivory paper theme: `--background #E6E1D3`, `--foreground #171717`,
`--primary` Ha Long Teal, `--secondary` Jungle Green. (Distinct from the brown
block — that brown was an ad-hoc section fill, not part of this theme.)

## The lesson for the rework

- Gold and teal as **recurring accents** = good, on-brand, warm Vietnam feel.
- Pink as a **rare jewel** (≤1 per section) = the punctuation that broke monotony.
- Brown **filling a whole block** = the actual mistake.

The corresponding standard-format colorscheme is
`public/colorschemes/vietnam_colorful.nix`.

## The Lotus Pink jewel — every original usage (exactly 3)

Pink (`#E58AAE`, Lotus) was the rarest accent — one deliberate punctuation per
major section, always on a *single detail that should draw the eye*, never as
chrome. The three sites of use in `vietnam_site/client/src/pages/Home.tsx`:

1. **Hero stat ribbon** (`~L213`) — the `100%` under "Institutional Grade".
   The one stat in a 4-stat row that gets the jewel; the punchline figure.
2. **Team heading** (`~L582`) — the italic phrase *"Institutional Pioneers"*
   in "Led by Institutional Pioneers". One phrase, not the whole heading.
3. **Trust badges** (`~L655`) — the `ISO 9001 CERTIFIED` shield icon, one of
   four badges. The others are gold/teal; pink marks the single most "premium"
   credential.

In the current `site_soup`, all three were flattened to teal/white during the
navy rework:
- hero `100%` → `text-white` (`site_soup ~L216`)
- team heading → `text-viet-teal`, and the copy changed to "Our Team" (`~L586`)
- ISO badge → `text-viet-teal` (`~L722`)

To restore the punctuation we either reinstate `viet-pink` exactly, or map it to
whatever the chosen scheme parks in the `warning` slot (in `vietnam_colorful.nix`
that slot *is* Lotus Pink).
