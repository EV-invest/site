"""
Render the current SOTA colorscheme draft as:
  1. plotly horizontal swatches (top of HTML), one row per named color
  2. a hand-styled example hero page below, using those same colors

Run from repo root:
  nix-shell -p python3Packages.plotly --run "python3 examples/colorscheme/explore.py"

Writes tmp/colorscheme.html (gitignored) and opens it in the browser.

The palette lives in PALETTE below; edit the (L, C, H) triples and re-run to
iterate. Recipe behind the values is in log/colorscheme.md.
"""

import math
import webbrowser
from pathlib import Path

import plotly.graph_objects as go


# ---------- OKLCH -> sRGB hex (D65) ----------------------------------------

def _oklch_to_linear_srgb(L, C, H_deg):
    H = math.radians(H_deg)
    a = C * math.cos(H)
    b = C * math.sin(H)

    l_ = L + 0.3963377774 * a + 0.2158037573 * b
    m_ = L - 0.1055613458 * a - 0.0638541728 * b
    s_ = L - 0.0894841775 * a - 1.2914855480 * b

    l, m, s = l_ ** 3, m_ ** 3, s_ ** 3

    r =  4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s
    g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s
    b_ = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
    return r, g, b_


def _linear_to_srgb(c):
    c = max(0.0, min(1.0, c))
    return 12.92 * c if c <= 0.0031308 else 1.055 * (c ** (1 / 2.4)) - 0.055


def oklch_hex(L, C, H):
    """Return sRGB hex string for an OKLCH triple. Out-of-gamut is clamped."""
    r, g, b = _oklch_to_linear_srgb(L, C, H)
    rgb = [_linear_to_srgb(x) for x in (r, g, b)]
    return "#{:02X}{:02X}{:02X}".format(
        *(max(0, min(255, round(x * 255))) for x in rgb)
    )


def oklch_css(L, C, H):
    return f"oklch({L:.3f} {C:.3f} {H:.1f})"


# ---------- The palette ----------------------------------------------------
# All values derived from anchor oklch(0.256 0.10 260) per log/colorscheme.md

H_BRAND = 260
H_NEUTRAL = 280

PALETTE = [
    # group, name, (L, C, H)
    ("neutral", "bg_deep",   (0.16, 0.020, H_NEUTRAL)),
    ("neutral", "bg",        (0.20, 0.025, H_NEUTRAL)),
    ("neutral", "surface",   (0.24, 0.030, H_NEUTRAL)),
    ("neutral", "elevated",  (0.30, 0.034, H_NEUTRAL)),
    ("neutral", "border",    (0.40, 0.038, H_NEUTRAL)),
    ("neutral", "muted",     (0.55, 0.040, H_NEUTRAL)),
    ("neutral", "subtle",    (0.70, 0.042, H_NEUTRAL)),
    ("neutral", "text",      (0.90, 0.040, H_NEUTRAL)),

    ("brand",   "brand",     (0.256, 0.10, H_BRAND)),  # the anchor
    ("brand",   "brand_fg",  (0.78,  0.11, H_BRAND)),
    ("brand",   "brand_hi",  (0.88,  0.08, H_BRAND)),

    ("state",   "danger",    (0.70, 0.16,    8)),
    ("state",   "warning",   (0.82, 0.13,   75)),
    ("state",   "success",   (0.80, 0.13,  145)),
    ("state",   "info",      (0.78, 0.10,  230)),
]


# ---------- Swatches via plotly --------------------------------------------

def build_swatch_fig():
    names = [name for _, name, _ in PALETTE]
    hexes = [oklch_hex(*lch) for _, _, lch in PALETTE]
    labels = [
        f"{name:<10}  L={lch[0]:.3f}  C={lch[1]:.3f}  H={lch[2]:>5.1f}"
        for _, name, lch in PALETTE
    ]

    # one horizontal bar per color, all the same width
    fig = go.Figure()
    for i, (name, hex_, label) in enumerate(zip(names, hexes, labels)):
        y = len(PALETTE) - 1 - i  # top-to-bottom in declaration order
        fig.add_shape(
            type="rect",
            x0=0, x1=1, y0=y - 0.45, y1=y + 0.45,
            fillcolor=hex_, line=dict(width=0),
        )
        # label sits to the right of the swatch
        fig.add_annotation(
            x=1.02, y=y, xref="x", yref="y",
            text=f"<span style='font-family:monospace'>{label}  {hex_}</span>",
            showarrow=False, xanchor="left", yanchor="middle",
            font=dict(color="#e5e5e5", size=13),
        )

    fig.update_xaxes(visible=False, range=[0, 2.2])
    fig.update_yaxes(visible=False, range=[-0.6, len(PALETTE) - 0.4])
    fig.update_layout(
        height=44 * len(PALETTE) + 40,
        margin=dict(l=20, r=20, t=20, b=20),
        paper_bgcolor="#111",
        plot_bgcolor="#111",
        showlegend=False,
    )
    return fig


# ---------- Hero example HTML ----------------------------------------------

def build_hero_html():
    """Return a <section> using the palette via CSS custom properties.

    Native CSS oklch() — every recent browser supports it. We bind names
    to values from PALETTE so this stays in sync with the swatch chart.
    """
    css_vars = "\n      ".join(
        f"--{name}: {oklch_css(*lch)};" for _, name, lch in PALETTE
    )
    return f"""
    <style>
      .demo-root {{
        {css_vars}
        --radius: 8px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        color: var(--text);
        background: var(--bg);
        padding: 0;
        margin: 0 auto;
        max-width: 1100px;
        border: 1px solid var(--border);
        border-radius: 12px;
        overflow: hidden;
      }}
      .demo-root * {{ box-sizing: border-box; }}
      .nav {{
        display: flex; align-items: center; justify-content: space-between;
        padding: 16px 28px; background: var(--bg_deep);
        border-bottom: 1px solid var(--border);
      }}
      .nav .logo {{
        display: inline-flex; align-items: center; gap: 10px;
        font-weight: 700; letter-spacing: 0.04em; color: var(--text);
      }}
      .nav .logo .mark {{
        width: 28px; height: 28px; border-radius: 6px;
        background: var(--brand);
        display: inline-flex; align-items: center; justify-content: center;
        color: var(--text); font-size: 13px; font-weight: 800;
      }}
      .nav a {{ color: var(--subtle); margin-left: 22px; text-decoration: none; font-size: 14px; }}
      .nav a:hover {{ color: var(--brand_fg); }}

      .hero {{ padding: 64px 48px 48px; background: var(--brand); }}
      .hero h1 {{
        margin: 0 0 16px; font-size: 44px; line-height: 1.1; color: var(--text);
        max-width: 640px;
      }}
      .hero p.lead {{
        margin: 0 0 32px; font-size: 18px; color: var(--brand_hi);
        max-width: 560px; opacity: 0.9;
      }}
      .btn {{
        display: inline-flex; align-items: center; gap: 8px;
        padding: 12px 22px; border-radius: var(--radius);
        font-size: 15px; font-weight: 600; text-decoration: none;
        border: 1px solid transparent; cursor: pointer;
      }}
      .btn-primary {{ background: var(--brand_fg); color: var(--bg_deep); }}
      .btn-primary:hover {{ background: var(--brand_hi); }}
      .btn-ghost {{
        background: transparent; color: var(--text);
        border-color: var(--border);
      }}
      .btn-ghost:hover {{ border-color: var(--brand_fg); color: var(--brand_fg); }}

      .body {{ padding: 48px; background: var(--bg); }}
      .body h2 {{ margin: 0 0 12px; font-size: 22px; color: var(--text); }}
      .body p {{ color: var(--subtle); line-height: 1.6; }}
      .body p .muted {{ color: var(--muted); }}

      .cards {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-top: 28px; }}
      .card {{
        background: var(--surface); border: 1px solid var(--border);
        border-radius: var(--radius); padding: 20px;
      }}
      .card h3 {{ margin: 0 0 8px; font-size: 15px; color: var(--text); }}
      .card p  {{ margin: 0; font-size: 13px; color: var(--muted); }}
      .card .num {{ font-size: 28px; font-weight: 700; color: var(--brand_fg); }}
      .card.elevated {{ background: var(--elevated); }}

      .states {{
        display: flex; flex-wrap: wrap; gap: 10px; margin-top: 28px;
      }}
      .pill {{
        display: inline-flex; align-items: center; gap: 8px;
        padding: 6px 12px; border-radius: 999px; font-size: 13px;
        background: var(--surface); border: 1px solid var(--border);
        color: var(--text);
      }}
      .pill .dot {{ width: 8px; height: 8px; border-radius: 50%; }}
      .pill.success .dot {{ background: var(--success); }}
      .pill.success {{ color: var(--success); border-color: color-mix(in oklch, var(--success) 40%, var(--border)); }}
      .pill.warning .dot {{ background: var(--warning); }}
      .pill.warning {{ color: var(--warning); border-color: color-mix(in oklch, var(--warning) 40%, var(--border)); }}
      .pill.danger  .dot {{ background: var(--danger);  }}
      .pill.danger  {{ color: var(--danger); border-color: color-mix(in oklch, var(--danger) 40%, var(--border)); }}
      .pill.info    .dot {{ background: var(--info);    }}
      .pill.info    {{ color: var(--info); border-color: color-mix(in oklch, var(--info) 40%, var(--border)); }}

      .formrow {{ display: flex; gap: 12px; margin-top: 24px; align-items: center; }}
      .formrow input {{
        flex: 1; padding: 11px 14px; border-radius: var(--radius);
        background: var(--bg_deep); border: 1px solid var(--border);
        color: var(--text); font-size: 14px; outline: none;
      }}
      .formrow input::placeholder {{ color: var(--muted); }}
      .formrow input:focus {{ border-color: var(--brand_fg); box-shadow: 0 0 0 3px color-mix(in oklch, var(--brand_fg) 25%, transparent); }}

      .footer {{
        padding: 22px 48px; border-top: 1px solid var(--border);
        background: var(--bg_deep); color: var(--muted); font-size: 13px;
        display: flex; justify-content: space-between;
      }}
      .footer a {{ color: var(--subtle); text-decoration: none; }}
      .footer a:hover {{ color: var(--brand_fg); }}
      a.link {{ color: var(--brand_fg); }}
    </style>

    <div class="demo-root">
      <div class="nav">
        <span class="logo"><span class="mark">EV</span> EV INVESTMENT</span>
        <div>
          <a href="#">Strategy</a>
          <a href="#">Portfolio</a>
          <a href="#">Insights</a>
          <a href="#">Contact</a>
        </div>
      </div>

      <div class="hero">
        <h1>Capital, deployed with conviction.</h1>
        <p class="lead">
          Long-horizon investing in companies building the electric future.
          Disciplined research. Concentrated positions. No nonsense.
        </p>
        <a class="btn btn-primary" href="#">Read our thesis &rarr;</a>
        &nbsp;
        <a class="btn btn-ghost" href="#">Portfolio &rarr;</a>
      </div>

      <div class="body">
        <h2>Recent performance</h2>
        <p>
          Numbers below are illustrative. <span class="muted">Past returns are not a
          reliable indicator of future performance.</span> See our
          <a class="link" href="#">methodology &rarr;</a>.
        </p>

        <div class="cards">
          <div class="card">
            <h3>YTD return</h3>
            <div class="num">+18.4%</div>
            <p>vs benchmark +9.1%</p>
          </div>
          <div class="card elevated">
            <h3>AUM</h3>
            <div class="num">$240M</div>
            <p>across 11 positions</p>
          </div>
          <div class="card">
            <h3>Hold time</h3>
            <div class="num">4.2y</div>
            <p>median, conviction-weighted</p>
          </div>
        </div>

        <div class="states">
          <span class="pill success"><span class="dot"></span> Allocation healthy</span>
          <span class="pill info"><span class="dot"></span> Re-balance scheduled</span>
          <span class="pill warning"><span class="dot"></span> Position near cap</span>
          <span class="pill danger"><span class="dot"></span> Drawdown alert</span>
        </div>

        <div class="formrow">
          <input placeholder="your@email — quarterly memos" />
          <a class="btn btn-primary" href="#">Subscribe</a>
        </div>
      </div>

      <div class="footer">
        <span>&copy; EV Investment 2026</span>
        <span><a href="#">Privacy</a> &middot; <a href="#">Terms</a> &middot; <a href="#">Disclosures</a></span>
      </div>
    </div>
    """


# ---------- Compose final HTML ---------------------------------------------

def main():
    fig = build_swatch_fig()
    swatch_html = fig.to_html(include_plotlyjs="cdn", full_html=False)

    page = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>EV colorscheme exploration</title>
<style>
  body {{ margin: 0; background: #0a0a0a; color: #e5e5e5;
         font-family: -apple-system, BlinkMacSystemFont, sans-serif; }}
  h1.page {{ margin: 0; padding: 28px 48px 8px; font-size: 20px; font-weight: 600; }}
  .section-title {{ padding: 8px 48px 4px; color: #888; font-size: 13px;
                    letter-spacing: 0.08em; text-transform: uppercase; }}
  .wrap {{ padding: 12px 28px 48px; }}
</style>
</head>
<body>
  <h1 class="page">EV colorscheme — exploration</h1>
  <div class="section-title">Palette (OKLCH)</div>
  <div class="wrap">{swatch_html}</div>
  <div class="section-title">Hero example using the same colors</div>
  <div class="wrap">{build_hero_html()}</div>
</body>
</html>
"""
    repo_root = Path(__file__).resolve().parent.parent.parent
    out = repo_root / "tmp" / "colorscheme.html"
    out.parent.mkdir(exist_ok=True)
    out.write_text(page)
    print(f"wrote {out}")
    webbrowser.open(out.as_uri())


if __name__ == "__main__":
    main()
