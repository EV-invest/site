# Light-mode counterpart of dark.nix. `brand` is pinned to the logo value (must
# match dark.nix), so the light scheme is built around it rather than derived
# from it.
#
# Neutrals are an ivory ramp (H=85, warm yellow-tan) rather than pure white —
# evokes paper / vellum and conveys editorial trust. The cool-navy brand on
# warm-ivory bg is the classic financial-press contrast (FT, WSJ).
#
# State colors are pulled in a touch on chroma vs. a generic palette, so they
# read as informational rather than alarming; still picked to stay in sRGB
# gamut (esp. yellow/green at H=75/145 which clamp at low L).
#
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals (hue 85 — paper). Chroma is intentionally tiny: real paper reads
  # as warm-white, not yellow. L is compressed at the top so surfaces feel
  # like one sheet, not stacked frames.
  bg_deep = { L = 0.965; C = 0.008; H = 85; };
  bg = { L = 0.985; C = 0.005; H = 85; };
  surface = { L = 0.975; C = 0.006; H = 85; };
  elevated = { L = 0.995; C = 0.003; H = 85; };
  border = { L = 0.900; C = 0.010; H = 85; };
  muted = { L = 0.550; C = 0.012; H = 85; };
  subtle = { L = 0.400; C = 0.012; H = 85; };
  text = { L = 0.220; C = 0.010; H = 85; };

  # brand (hue 260) — `brand` MUST equal dark.nix (pinned to logo).
  brand = { L = 0.256; C = 0.100; H = 260; };
  brand_fg = { L = 0.400; C = 0.160; H = 260; };
  brand_hi = { L = 0.300; C = 0.140; H = 260; };

  # state — restrained chroma; informational, not alarming.
  danger = { L = 0.520; C = 0.170; H = 10; };
  warning = { L = 0.680; C = 0.120; H = 70; };
  success = { L = 0.520; C = 0.130; H = 145; };
  info = { L = 0.520; C = 0.120; H = 240; };
}
