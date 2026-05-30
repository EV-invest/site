# The ORIGINAL `vietnam_site/` palette, pinned faithfully as a standard
# colorscheme so it can be selected in the Rust color-testing harness next to
# dark.nix / dark_vietnam.nix / light.nix. Raw snapshot + provenance:
# examples/colorscheme/vietnam_site_original.md.
#
# IMPORTANT — this scheme does not map cleanly onto the `Colorscheme` field set.
# The original used SIX vibrant accents decoratively (Rice Gold, Ha Long Teal,
# Lotus Pink, Jungle Green, Flag Red, Lacquer Brown), but the struct exposes
# only `brand` + four state slots (danger/warning/success/info). So this is the
# best faithful fit, not a 1:1 copy:
#   - brand   = Rice Gold      (the original's lead `--primary`)
#   - danger  = Flag Red
#   - success = Jungle Green
#   - info    = Ha Long Teal
#   - warning = Lotus Pink     ← the "rare jewel" parked here so it has a home.
#                                (The original used Rice Gold for warnings, but
#                                 gold is already `brand`, so the slot is reused
#                                 to keep Pink representable.) Lacquer Brown does
#                                 NOT survive — it was the one structural misuse.
#
# This is deliberately the warm GOLD-led original, NOT the navy logo. Kept as a
# reference to A/B against the navy-led schemes and to steal accents from.
# OKLCH values from hex via tmp/colorscheme_vietnam/hex_to_oklch.py.
#
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals — the original's true pure-grey charcoal ramp (#171717 → #2d2d2d),
  # extended to a full ramp ending at Limestone Mist. The original was C=0 grey;
  # a whisper of warm chroma (H=90, C≤0.02) matches the ivory text and keeps
  # every step in sRGB gamut.
  bg_deep = { L = 0.175; C = 0.004; H = 90; }; # just under Ink Black #171717
  bg = { L = 0.205; C = 0.005; H = 90; }; # Ink Black #171717
  surface = { L = 0.239; C = 0.006; H = 90; }; # card #1f1f1f
  elevated = { L = 0.297; C = 0.008; H = 90; }; # muted #2d2d2d
  border = { L = 0.360; C = 0.010; H = 90; };
  muted = { L = 0.550; C = 0.014; H = 90; };
  subtle = { L = 0.703; C = 0.015; H = 90; }; # muted-fg #a39f95
  text = { L = 0.910; C = 0.020; H = 90; }; # Limestone Mist #E6E1D3

  # brand — Rice Gold, the original's lead `--primary`.
  brand = { L = 0.850; C = 0.146; H = 90.5; }; # Rice Gold #F2C94C
  brand_fg = { L = 0.205; C = 0.005; H = 90; }; # Ink Black — text on gold
  brand_hi = { L = 0.910; C = 0.020; H = 90; }; # Limestone Mist hover lift

  # state — the Vietnam accents (decorative in the original, semantic here).
  danger = { L = 0.572; C = 0.215; H = 28.8; }; # Flag Red     #DA251D
  warning = { L = 0.739; C = 0.118; H = 355.1; }; # Lotus Pink   #E58AAE ← the jewel
  success = { L = 0.482; C = 0.104; H = 154.8; }; # Jungle Green #1F6F43
  info = { L = 0.630; C = 0.101; H = 183; }; # Ha Long Teal #2A9D8F
}
