# `dark.nix` with a subtle Vietnam-inspired flavor. Identical to `dark.nix`
# in every neutral and in the brand triad; only the four state-accent HUES
# are nudged toward Vietnamese palette anchors. L and C are unchanged from
# `dark.nix`, so contrast, gamut, and the overall investment-grade trust
# character (cool navy on near-purple neutrals, ivory text) are preserved.
#
# Hue anchors (sourced from tmp/colorscheme_vietnam/hex_to_oklch.py):
#   Flag Red       #DA251D  H ≈ 29
#   Rice Gold      #F2C94C  H ≈ 91
#   Jungle Green   #1F6F43  H ≈ 155
#   Ha Long Teal   #2A9D8F  H ≈ 183
#
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals (hue 280) — IDENTICAL to dark.nix
  bg_deep = { L = 0.160; C = 0.020; H = 280; };
  bg = { L = 0.200; C = 0.025; H = 280; };
  surface = { L = 0.240; C = 0.030; H = 280; };
  elevated = { L = 0.300; C = 0.034; H = 280; };
  border = { L = 0.400; C = 0.038; H = 280; };
  muted = { L = 0.550; C = 0.040; H = 280; };
  subtle = { L = 0.700; C = 0.042; H = 280; };
  text = { L = 0.900; C = 0.040; H = 280; };

  # brand (hue 260, anchor) — IDENTICAL to dark.nix (pinned to the logo)
  brand = { L = 0.256; C = 0.100; H = 260; };
  brand_fg = { L = 0.780; C = 0.110; H = 260; };
  brand_hi = { L = 0.880; C = 0.080; H = 260; };

  # state — same L+C bands as dark.nix, hues pulled toward Vietnam anchors.
  # danger and warning move to the exact Flag Red / Rice Gold hues (the
  # most iconic Vietnamese colors); success and info only partway, so they
  # still read as the familiar dark.nix green and cyan-blue rather than as
  # foreign-looking jungle-green / teal.
  danger = { L = 0.700; C = 0.160; H = 28; }; # → Flag Red       (was H=8)
  warning = { L = 0.820; C = 0.130; H = 90; }; # → Rice Gold      (was H=75)
  success = { L = 0.800; C = 0.130; H = 152; }; # → Jungle Green   (was H=145)
  info = { L = 0.780; C = 0.105; H = 210; }; # → Ha Long Teal   (was H=230, partial pull)
}
