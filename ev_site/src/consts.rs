//! Placeholder copy (will be replaced by real data).

pub(crate) const COMPANY: &str = "EV INVESTMENT";
pub(crate) const LOGO_MARK: &str = "EV";

pub(crate) const HERO_TITLE: &str = "Capital, deployed with conviction.";
pub(crate) const HERO_LEAD: &str = "Long-horizon investing in companies building the electric future. \
                        Disciplined research. Concentrated positions. No nonsense.";

pub(crate) const PERF_HEADING: &str = "Recent performance";
pub(crate) const PERF_INTRO: &str = "Numbers below are illustrative.";
pub(crate) const PERF_DISCLAIMER: &str = "Past returns are not a reliable indicator of future performance.";

pub(crate) const YTD_LABEL: &str = "YTD return";
pub(crate) const YTD_VALUE: &str = "+18.4%";
pub(crate) const YTD_SUB: &str = "vs benchmark +9.1%";

pub(crate) const AUM_LABEL: &str = "AUM";
pub(crate) const AUM_VALUE: &str = "$240M";
pub(crate) const AUM_SUB: &str = "across 11 positions";

pub(crate) const HOLD_LABEL: &str = "Hold time";
pub(crate) const HOLD_VALUE: &str = "4.2y";
pub(crate) const HOLD_SUB: &str = "median, conviction-weighted";

pub(crate) const PILL_SUCCESS: &str = "Allocation healthy";
pub(crate) const PILL_INFO: &str = "Re-balance scheduled";
pub(crate) const PILL_WARNING: &str = "Position near cap";
pub(crate) const PILL_DANGER: &str = "Drawdown alert";

pub(crate) const SUBSCRIBE_PLACEHOLDER: &str = "your@email — quarterly memos";
pub(crate) const SUBSCRIBE_CTA: &str = "Subscribe";

pub(crate) const COPYRIGHT: &str = "\u{00A9} EV Investment 2026";

// Reused inline-style fragments. Kept here purely to avoid retyping the same long
// string across multiple call-sites; each is a single CSS declaration block. To move
// or delete a button/card, copy or remove its callsite — these consts can stay if
// unused (they're plain `&str`s) or be inlined back.
pub(crate) const STYLE_BTN_BASE: &str = "display: inline-flex; align-items: center; gap: 0.5rem; \
                              padding: 0.75rem 1.375rem; border-radius: var(--radius); \
                              font-size: 0.9375rem; font-weight: 600; text-decoration: none; \
                              border: 0.0625rem solid transparent; cursor: pointer;";
pub(crate) const STYLE_BTN_PRIMARY: &str = "background: var(--brand_fg); color: var(--bg_deep);";
pub(crate) const STYLE_BTN_GHOST: &str = "background: transparent; color: var(--text); \
                               border-color: var(--border);";

pub(crate) const STYLE_CARD: &str = "border: 0.0625rem solid var(--border); border-radius: var(--radius); \
                          padding: 1.25rem;";
pub(crate) const STYLE_CARD_H3: &str = "margin: 0 0 0.5rem; font-size: 0.9375rem; color: var(--text);";
pub(crate) const STYLE_CARD_NUM: &str = "font-size: 1.75rem; font-weight: 700; color: var(--brand_fg);";
pub(crate) const STYLE_CARD_P: &str = "margin: 0; font-size: 0.8125rem; color: var(--muted);";

pub(crate) const STYLE_PILL_DOT: &str = "width: 0.5rem; height: 0.5rem; border-radius: 50%;";
