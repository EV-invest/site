# Dark theme, subtly inspired by the Vietnam palette. Same charcoal+navy
# institutional bones as `dark.nix`, with two intentional Vietnam moments
# that don't compromise the trust-and-confidence read:
#
#   1. Neutrals collapsed to the brand hue (H=260) at near-zero chroma —
#      true charcoal aligned to the navy logo. No competing tints in the
#      bg/surface/border ramp; the resting page is institutional and quiet.
#
#   2. `brand_hi` (hover lift on brand elements) set to literal Limestone
#      Mist from the Vietnam palette: #E6E1D3, L=0.910 C=0.020 H=90.
#      A warm-paper / ivory halo, not a colored accent. Only appears on
#      hover, so it never competes with the resting palette — but when it
#      does appear, it reads as editorial / institutional rather than "fun".
#
# State accents (danger/warning/success/info) keep the Vietnam-derived
# hues already established in `dark.nix` — Flag Red, Rice Gold, Jungle
# Green, Ha Long Teal — used only for status, never for chrome.
#
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals — true charcoal aligned to brand hue (H=260) at near-zero chroma.
  bg_deep = { L = 0.150; C = 0.008; H = 260; };
  bg = { L = 0.190; C = 0.010; H = 260; };
  surface = { L = 0.230; C = 0.012; H = 260; };
  elevated = { L = 0.290; C = 0.014; H = 260; };
  border = { L = 0.380; C = 0.018; H = 260; };
  muted = { L = 0.550; C = 0.020; H = 260; };
  subtle = { L = 0.700; C = 0.022; H = 260; };
  text = { L = 0.910; C = 0.020; H = 260; };

  # brand — `brand` and `brand_fg` are the exact logo navy (pinned).
  # `brand_hi` is Limestone Mist (#E6E1D3) — the single Vietnam moment
  # in the chrome, surfaced only on hover.
  brand = { L = 0.256; C = 0.100; H = 260; };
  brand_fg = { L = 0.780; C = 0.110; H = 260; };
  brand_hi = { L = 0.910; C = 0.020; H = 90; };

  # state — Vietnam-anchored hues, unchanged from dark.nix.
  danger = { L = 0.700; C = 0.160; H = 28; }; # → Flag Red
  warning = { L = 0.820; C = 0.130; H = 90; }; # → Rice Gold
  success = { L = 0.800; C = 0.130; H = 152; }; # → Jungle Green (partial)
  info = { L = 0.780; C = 0.105; H = 210; }; # → Ha Long Teal (partial)
}
