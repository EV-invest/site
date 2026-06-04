# Vietnam-themed palette for an investments / funds dashboard.
# Identity kept: navy / red / gold / teal. Tokens are OKLCH.
# Fixed: hover-lift bug, neutral text hue, separated surface ramp, state contrast.
{
  # neutrals — the navy-black ramp used as bg / surface / card.
  # Ramp now steps cleanly: bg < surface < elevated < muted < border (rising L),
  # so panels and hairlines are visually distinct instead of bunched together.
  bg_deep  = { L = 0.150; C = 0.026; H = 261.1; }; # deepest bg   #060b16
  bg       = { L = 0.159; C = 0.026; H = 261.1; }; # base bg      #070d18
  surface  = { L = 0.182; C = 0.036; H = 262.8; }; # section-alt  #091222
  elevated = { L = 0.215; C = 0.038; H = 260.0; }; # panels/cards #0d1828
  muted    = { L = 0.285; C = 0.050; H = 264.0; }; # fill step    #18243e
  border   = { L = 0.340; C = 0.052; H = 265.0; }; # hairline     #232f4c  (now clearly > fills)
  subtle   = { L = 0.722; C = 0.024; H = 260.0; }; # muted-fg     #99a4b6  (hue → cool neutral)
  text     = { L = 0.940; C = 0.010; H = 260.0; }; # primary text #e9ebf0  (cool ivory, no yellow cast)

  # brand — the logo navy. brand_hi is now a real hover LIFT on the navy fill,
  # not a copy of brand_fg (that bug made button text vanish on hover).
  brand    = { L = 0.249; C = 0.096; H = 258.8; }; # viet-navy    #001E4E
  brand_fg = { L = 0.940; C = 0.010; H = 260.0; }; # text on navy (matches `text`)
  brand_hi = { L = 0.320; C = 0.100; H = 258.8; }; # navy hover    lifted +0.07 L, fg stays readable

  # state — red reserved for errors/danger (kept loud), green = growth,
  # gold = returns/attention, teal = accent/lead. All lifted to pass small-text
  # contrast on the dark bg.
  danger   = { L = 0.605; C = 0.215; H = 28.8; }; # Flag Red      #E63A2E  (loud, error only)
  warning  = { L = 0.800; C = 0.150; H = 88.0; }; # viet-gold     #E8B92E  (lowered off ivory)
  success  = { L = 0.680; C = 0.150; H = 152.7; }; # viet-green    #34B26A  (growth, readable)
  info     = { L = 0.660; C = 0.105; H = 188.0; }; # viet-teal     #2FA89A  (accent / lead)
}
