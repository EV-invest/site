# Colorscheme

## Goal
Figure out a procedural way to generate the **exact full colorscheme for the entire
site and branding** from a single anchor color. The recipe must be reproducible:
deterministic transformations in OKLCH, not eyeballed picks.

## Anchor
**`oklch(25.6% 0.1 260)`** — the navy from the logo background.
- L = 25.6% (dark)
- C = 0.1 (moderate chroma — well within sRGB gamut at this L/H)
- H = 260° (blue, slight lean toward purple at 270°)

Everything downstream is derived from this triple.

---

## The procedural pattern (observed across Catppuccin, Tokyo Night, Rosé Pine)

All three top-tier dark themes follow the same construction. This is the recipe.

### 1. Two distinct rings in OKLCH

| Ring        | L                | C            | H                                    |
|-------------|------------------|--------------|--------------------------------------|
| **Neutrals** (bg → text) | 0.18 → 0.90 ramp | 0.02 – 0.05  | **single locked hue**, slight purple lean |
| **Accents**  | locked band 0.72 – 0.85 | locked band 0.08 – 0.14 | **distributed around the wheel** |

Neutrals share a hue and vary L. Accents share L+C and vary H. That is the whole idea.

### 2. Neutral-hue choice
Pick the neutral hue close to but slightly off the brand hue (typically +15° to +30° toward purple — keeps surfaces feeling "of the same family" without competing with the brand).

Observed neutral hues:
- Catppuccin Mocha: ~282° (brand blue at 260°, +22°)
- Tokyo Night Storm: ~274° (blue accent at 264°, +10°)
- Rosé Pine: ~290° (no single brand, picks the warm-purple side)

For our anchor at H=260, neutrals at **H ≈ 275–285** fit the same pattern.

### 3. Chroma curve along the neutral ramp
Chroma is **not** constant — it's a low-amplitude bell: low at the very dark and very light extremes, slight peak in the mid-tones. Catppuccin Mocha example:
- crust L=0.18 → C=0.020
- base  L=0.24 → C=0.030
- surface0 L=0.32 → C=0.032
- overlay0 L=0.55 → C=0.034
- subtext1 L=0.82 → C=0.040
- text  L=0.88 → C=0.043

So chroma rises ~2× from black to text. Simplest reproducible curve: `C(L) = C_min + (C_max - C_min) * sin(π * L)`.

### 4. Accent ring
Accents sit at a tight L/C band so no single hue dominates. Concrete observed values:

| Theme            | accent L | accent C |
|------------------|----------|----------|
| Catppuccin Mocha | 0.76 – 0.92 | 0.07 – 0.13 |
| Tokyo Night Storm| 0.65 – 0.82 | 0.10 – 0.16 |
| Rosé Pine        | 0.70 – 0.85 | 0.05 – 0.15 |

Hues distributed at semantic positions:
- red ≈ 0–10° · orange ≈ 50° · yellow ≈ 85° · green ≈ 140°
- teal ≈ 180° · cyan ≈ 210° · blue ≈ 260° · purple ≈ 300° · pink ≈ 335°

Catppuccin uses 14 hues, Tokyo Night ~8, Rosé Pine 7. **More than ~8 starts feeling
indistinguishable** — pick the minimum for our use case.

### 5. The brand anchor's special role
Our anchor `oklch(25.6% 0.1 260)` is **darker and more saturated** than the accent
ring — it sits in the *neutral-L* range but with full *accent-C*. This means:

- **Brand-as-surface**: the anchor itself is the signature dark surface (logo bg, hero, primary brand panels). Bg ramp is built *around* it: slightly darker for body bg, slightly lighter for elevated surfaces, all sharing H≈260–280.
- **Brand-as-accent**: lift to the accent ring → `oklch(0.78 0.11 260)`. Used wherever the brand needs to read on a dark surface (buttons, links, focus rings).

This dual role is exactly how Catppuccin handles `blue` (accent at L=0.77) vs `base`
(L=0.24, very low C). We collapse both onto our single anchor hue.

---

## Recipe for our site

Derive **everything** from `(L₀, C₀, H₀) = (0.256, 0.10, 260)`:

```
H_brand    = 260
H_neutral  = 280            # +20° purple lean

# Neutrals (single hue, lightness ramp, bell chroma)
bg_deep    = oklch(0.16  0.020  280)
bg         = oklch(0.20  0.025  280)
surface    = oklch(0.24  0.030  280)   # ~ anchor L with neutral C
elevated   = oklch(0.30  0.034  280)
border     = oklch(0.40  0.038  280)
muted      = oklch(0.55  0.040  280)
subtle     = oklch(0.70  0.042  280)
text       = oklch(0.90  0.040  280)

# Brand
brand      = oklch(0.256 0.10   260)   # the anchor — signature surface
brand_fg   = oklch(0.78  0.11   260)   # accent-ring lift for use on dark
brand_hi   = oklch(0.88  0.08   260)   # hover / emphasis

# Semantic accents (locked L=0.78, C=0.11, hue distributed)
danger     = oklch(0.70  0.16     8)   # red — bump C, hue is loud
warning    = oklch(0.82  0.13    75)   # yellow-orange
success    = oklch(0.80  0.13   145)   # green
info       = oklch(0.78  0.10   230)   # cyan-blue (offset from brand)
```

Light mode (if/when needed): mirror by `L_light = 1 - L_dark` per stop, drop accent
chroma ~25% (light backgrounds make saturated accents shout).

### Constraints to enforce
- Every color stays inside sRGB gamut → if `C > C_max(L, H)`, clamp C, never L.
- Adjacent neutrals must differ by ≥ 0.04 in L (visible step).
- Text on `bg`: target APCA |Lc| ≥ 75 (≈ WCAG AAA body). At our values text 0.90
  on bg 0.20 sits well above that.
- Brand-as-accent on `bg`: target APCA |Lc| ≥ 60 (UI element).

---

## Sources worth reading again
- evilmartians.com/chronicles/oklch-in-css-why-quit-rgb-hsl — why OKLCH at all, P3 expansion pattern
- utilitybend.com — sine easing on chroma via `oklch(from …)` relative syntax
- catppuccin/palette JSON — empirical 26-color reference, all four flavors in OKLCH
- huetone.ardov.me — interactive palette builder with APCA contrast overlay
