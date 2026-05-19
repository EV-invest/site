# current preferred colorscheme; derived from anchor oklch(0.256 0.10 260) per log/colorscheme.md.
# Format: flat attrset of OKLCH triples (parsed at compile time by build.rs).
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals (hue 280)
  bg_deep = { L = 0.160; C = 0.020; H = 280; };
  bg = { L = 0.200; C = 0.025; H = 280; };
  surface = { L = 0.240; C = 0.030; H = 280; };
  elevated = { L = 0.300; C = 0.034; H = 280; };
  border = { L = 0.400; C = 0.038; H = 280; };
  muted = { L = 0.550; C = 0.040; H = 280; };
  subtle = { L = 0.700; C = 0.042; H = 280; };
  text = { L = 0.900; C = 0.040; H = 280; };

  # brand (hue 260, anchor)
  brand = { L = 0.256; C = 0.100; H = 260; };
  brand_fg = { L = 0.780; C = 0.110; H = 260; };
  brand_hi = { L = 0.880; C = 0.080; H = 260; };

  # state
  danger = { L = 0.700; C = 0.160; H = 8; };
  warning = { L = 0.820; C = 0.130; H = 75; };
  success = { L = 0.800; C = 0.130; H = 145; };
  info = { L = 0.780; C = 0.100; H = 230; };
}
