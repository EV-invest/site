# FINAL navy-led scheme — built from the colors *actually used* in the
# `site_soup/` React build, not the full set its `index.css` defined. Usage was
# counted across `site_soup/client/src` (Tailwind classes, opacity modifiers,
# and `var(--color-*)` / `rgba()` refs in index.css); see
# tmp/final_colorscheme.md for the counts. OKLCH from hex via
# tmp/colorscheme_vietnam/hex_to_oklch.py.
#
# This is the navy logo-led resting palette (viet-black bg, viet-mist ivory
# text, viet-teal as the single connective accent), with the three rare jewels
# — gold/green/red — living in the state slots.
#
# Tokens that index.css defined but the markup NEVER referenced are dropped:
#   navy-soft, charcoal, brown, and the `viet-red` alias (status red is reached
#   through the shadcn `--destructive` value, kept here as `danger`).
#
# Mapping onto the fixed `Colorscheme` field set (src/config.rs):
#   - neutrals = the navy-black ramp: viet-black → viet-surface → viet-card,
#                with the shadcn navy-step + silver-navy hairline above it.
#   - brand    = viet-navy (#001E4E, the logo) ; brand_hi = viet-mist hover lift.
#   - info     = viet-teal — the 48-use workhorse accent, the real lead color.
#   - success  = viet-green (positive financial signals) ; warning = viet-gold
#                (headline return figures) ; danger = Flag Red (status only).
#   - viet-pink (#E58AAE, L=0.739 C=0.118 H=355.1) — the single 1-use lotus
#     punctuation jewel — has NO home in the 4-state set and is intentionally
#     not carried; gold/green/red are the meaningful accents here.
#
# Field set is fixed across all colorschemes — see `Colorscheme` in src/config.rs.
{
  # neutrals — the navy-black ramp actually used as bg / surface / card.
  bg_deep = { L = 0.159; C = 0.026; H = 261.1; }; # viet-black   #070d18 (base bg)
  bg = { L = 0.159; C = 0.026; H = 261.1; }; # viet-black   #070d18
  surface = { L = 0.175; C = 0.036; H = 262.8; }; # viet-surface #081020 (section-alt)
  elevated = { L = 0.200; C = 0.036; H = 259.5; }; # viet-card    #0c1626 (panels)
  border = { L = 0.276; C = 0.053; H = 265.0; }; # hairline     #1b2742
  muted = { L = 0.251; C = 0.050; H = 265.0; }; # muted step   #16213a
  subtle = { L = 0.722; C = 0.030; H = 258.4; }; # viet-silver  #9aa6b8 (muted-fg)
  text = { L = 0.910; C = 0.020; H = 90.6; }; # viet-mist    #E6E1D3 (ivory text)

  # brand — the logo navy; brand_hi is the Limestone Mist hover lift.
  brand = { L = 0.249; C = 0.096; H = 258.8; }; # viet-navy    #001E4E
  brand_fg = { L = 0.910; C = 0.020; H = 90.6; }; # viet-mist on navy
  brand_hi = { L = 0.910; C = 0.020; H = 90.6; }; # viet-mist    #E6E1D3 hover

  # state — the used accents. `info` carries the real lead (Ha Long Teal).
  danger = { L = 0.572; C = 0.215; H = 28.8; }; # Flag Red     #DA251D (status)
  warning = { L = 0.850; C = 0.146; H = 90.5; }; # viet-gold    #F2C94C (returns)
  success = { L = 0.621; C = 0.141; H = 152.7; }; # viet-green   #2E9E5B (growth)
  info = { L = 0.630; C = 0.101; H = 183.0; }; # viet-teal    #2A9D8F (accent)
}
