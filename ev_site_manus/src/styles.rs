//! Self-contained global stylesheet for the manus dioxus port.
//!
//! The original site relies on Tailwind. Rather than pulling in a build step we
//! ship a hand-written subset that covers exactly the utilities used by the
//! ported `Home`/`NotFound` pages, plus the bespoke `index.css` rules
//! (`.btn-primary`, `.card-minimal`, `.text-gradient`, `.container`,
//! `.fade-in-up`, …) verbatim where practical.

pub const GLOBAL_CSS: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap');

:root {
  --primary: #00D084;
  --primary-foreground: #0F1419;
  --accent: #00B4D8;
  --accent-foreground: #0F1419;
  --secondary: #6B7280;
  --secondary-foreground: #F5F7FA;
  --radius: 0.5rem;
  --background: #F5F7FA;
  --foreground: #0F1419;
  --card: #FFFFFF;
  --card-foreground: #0F1419;
  --muted: #D0D5DD;
  --muted-foreground: #6B7280;
  --destructive: #EF4444;
  --destructive-foreground: #FFFFFF;
  --border: #D0D5DD;
  --input: #FFFFFF;
  --ring: #00D084;
  --ease-out: cubic-bezier(0.23, 1, 0.32, 1);
  --ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);
}

*, *::before, *::after { box-sizing: border-box; border-color: var(--border); }
html { scroll-behavior: smooth; }
body {
  margin: 0;
  background: var(--background);
  color: var(--foreground);
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
}
h1, h2, h3, h4, h5, h6 {
  font-family: 'Space Grotesk', system-ui, -apple-system, sans-serif;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin: 0;
}
p { margin: 0; }
ul { margin: 0; padding: 0; list-style: none; }
a { color: inherit; text-decoration: none; }
button { font: inherit; color: inherit; background: none; border: 0; padding: 0; cursor: pointer; }
button:not(:disabled), a[href] { cursor: pointer; }
@media (prefers-reduced-motion: no-preference) {
  * { transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-duration: 150ms; }
}

/* ---------- Bespoke components from index.css -------------------------- */

.container {
  width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding-left: 1rem;
  padding-right: 1rem;
}
@media (min-width: 640px) { .container { padding-left: 1.5rem; padding-right: 1.5rem; } }
@media (min-width: 1024px) { .container { padding-left: 2rem; padding-right: 2rem; max-width: 1280px; } }

.btn-primary {
  display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: var(--primary); color: var(--primary-foreground);
  border-radius: var(--radius); font-weight: 600;
  transition: all 200ms var(--ease-out);
}
.btn-primary:hover { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); transform: scale(1.05); }
.btn-primary:active { transform: scale(0.95); }

.btn-secondary {
  display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--border); color: var(--foreground);
  border-radius: var(--radius); font-weight: 600;
  transition: all 200ms var(--ease-out);
}
.btn-secondary:hover { background: var(--muted); }
.btn-secondary:active { transform: scale(0.95); }

.card-minimal {
  background: var(--card); color: var(--card-foreground);
  border: 1px solid var(--border); border-radius: var(--radius);
  padding: 1.5rem;
  transition: all 300ms var(--ease-out);
}
.card-minimal:hover { box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }

.text-gradient {
  background: linear-gradient(to right, #00D084, #00B4D8, #00D084);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  color: transparent;
}

.fade-in-up { animation: fadeInUp 0.6s var(--ease-out) forwards; }
@keyframes fadeInUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: .5; } }
.animate-pulse { animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }

/* ---------- Tailwind utility subset used by ported pages -------------- */

.min-h-screen { min-height: 100vh; }
.h-full { height: 100%; }
.w-full { width: 100%; }
.h-px { height: 1px; }
.h-2 { height: 0.5rem; }
.h-4 { height: 1rem; } .w-4 { width: 1rem; }
.h-5 { height: 1.25rem; } .w-5 { width: 1.25rem; }
.h-6 { height: 1.5rem; } .w-6 { width: 1.5rem; }
.h-8 { height: 2rem; }   .w-8 { width: 2rem; }
.h-12 { height: 3rem; }  .w-12 { width: 3rem; }
.h-16 { height: 4rem; }  .w-16 { width: 4rem; }
.w-1\.5 { width: 0.375rem; } .h-1\.5 { height: 0.375rem; }

.max-w-lg { max-width: 32rem; }
.max-w-xl { max-width: 36rem; }
.max-w-2xl { max-width: 42rem; }
.max-w-3xl { max-width: 48rem; }
.max-w-4xl { max-width: 56rem; }
.max-w-5xl { max-width: 64rem; }
.max-w-6xl { max-width: 72rem; }

.mx-auto { margin-left: auto; margin-right: auto; }
.mx-4 { margin-left: 1rem; margin-right: 1rem; }
.mr-2 { margin-right: 0.5rem; }
.mt-4 { margin-top: 1rem; }
.mt-12 { margin-top: 3rem; }
.mb-1 { margin-bottom: 0.25rem; }
.mb-2 { margin-bottom: 0.5rem; }
.mb-3 { margin-bottom: 0.75rem; }
.mb-4 { margin-bottom: 1rem; }
.mb-6 { margin-bottom: 1.5rem; }
.mb-8 { margin-bottom: 2rem; }
.mb-12 { margin-bottom: 3rem; }
.mb-16 { margin-bottom: 4rem; }

.px-4 { padding-left: 1rem; padding-right: 1rem; }
.px-6 { padding-left: 1.5rem; padding-right: 1.5rem; }
.py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
.py-2\.5 { padding-top: 0.625rem; padding-bottom: 0.625rem; }
.py-4 { padding-top: 1rem; padding-bottom: 1rem; }
.py-12 { padding-top: 3rem; padding-bottom: 3rem; }
.py-24 { padding-top: 6rem; padding-bottom: 6rem; }
.pt-6 { padding-top: 1.5rem; }
.pt-8 { padding-top: 2rem; }
.pt-12 { padding-top: 3rem; }
.pb-8 { padding-bottom: 2rem; }

.flex { display: flex; }
.hidden { display: none; }
.inline-block { display: inline-block; }
.inline-flex { display: inline-flex; }
.grid { display: grid; }
.flex-1 { flex: 1 1 0%; }
.flex-col { flex-direction: column; }
.flex-wrap { flex-wrap: wrap; }
.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.justify-center { justify-content: center; }
.justify-between { justify-content: space-between; }

.gap-2 { gap: 0.5rem; }
.gap-3 { gap: 0.75rem; }
.gap-4 { gap: 1rem; }
.gap-6 { gap: 1.5rem; }
.gap-8 { gap: 2rem; }

.space-y-2 > * + * { margin-top: 0.5rem; }
.space-y-6 > * + * { margin-top: 1.5rem; }

.relative { position: relative; }
.absolute { position: absolute; }
.sticky { position: sticky; }
.inset-0 { top: 0; right: 0; bottom: 0; left: 0; }
.top-0 { top: 0; }
.z-0 { z-index: 0; }
.z-10 { z-index: 10; }
.z-20 { z-index: 20; }
.z-50 { z-index: 50; }
.overflow-hidden { overflow: hidden; }

.rounded-md { border-radius: calc(var(--radius) - 2px); }
.rounded-lg { border-radius: var(--radius); }
.rounded-xl { border-radius: calc(var(--radius) + 4px); }
.rounded-full { border-radius: 9999px; }

.border { border: 1px solid var(--border); }
.border-0 { border-width: 0; }
.border-2 { border-width: 2px; }
.border-b { border-bottom: 1px solid var(--border); }
.border-t { border-top: 1px solid var(--border); }
.border-border { border-color: var(--border); }
.border-transparent { border-color: transparent; }
.border-primary\/30 { border-color: color-mix(in srgb, var(--primary) 30%, transparent); }
.border-primary\/50 { border-color: color-mix(in srgb, var(--primary) 50%, transparent); }
.outline-none { outline: none; }

.text-xs { font-size: 0.75rem; line-height: 1rem; }
.text-sm { font-size: 0.875rem; line-height: 1.25rem; }
.text-base { font-size: 1rem; line-height: 1.5rem; }
.text-lg { font-size: 1.125rem; line-height: 1.75rem; }
.text-xl { font-size: 1.25rem; line-height: 1.75rem; }
.text-2xl { font-size: 1.5rem; line-height: 2rem; }
.text-3xl { font-size: 1.875rem; line-height: 2.25rem; }
.text-4xl { font-size: 2.25rem; line-height: 2.5rem; }
.text-5xl { font-size: 3rem; line-height: 1; }
.text-7xl { font-size: 4.5rem; line-height: 1; }
.font-medium { font-weight: 500; }
.font-semibold { font-weight: 600; }
.font-bold { font-weight: 700; }
.uppercase { text-transform: uppercase; }
.tracking-tight { letter-spacing: -0.025em; }
.tracking-wider { letter-spacing: 0.05em; }
.leading-tight { line-height: 1.25; }
.leading-relaxed { line-height: 1.625; }
.text-center { text-align: center; }
.whitespace-nowrap { white-space: nowrap; }

.text-white { color: #ffffff; }
.text-foreground { color: var(--foreground); }
.text-primary { color: var(--primary); }
.text-muted-foreground { color: var(--muted-foreground); }
.text-primary\/20 { color: color-mix(in srgb, var(--primary) 20%, transparent); }
.text-slate-600 { color: #475569; }
.text-slate-700 { color: #334155; }
.text-slate-900 { color: #0f172a; }
.text-red-500 { color: #ef4444; }

.bg-background { background: var(--background); }
.bg-background\/50 { background: color-mix(in srgb, var(--background) 50%, transparent); }
.bg-background\/80 { background: color-mix(in srgb, var(--background) 80%, transparent); }
.bg-background\/60 { background: color-mix(in srgb, var(--background) 60%, transparent); }
.bg-card { background: var(--card); }
.bg-muted { background: var(--muted); }
.bg-primary { background: var(--primary); }
.bg-primary\/5 { background: color-mix(in srgb, var(--primary) 5%, transparent); }
.bg-blue-600 { background: #2563eb; }
.bg-blue-600:hover, .hover\:bg-blue-700:hover { background: #1d4ed8; }
.bg-white\/80 { background: rgba(255,255,255,0.8); }
.bg-red-100 { background: #fee2e2; }

.bg-gradient-to-r { background-image: linear-gradient(to right, var(--tw-gradient-from, transparent), var(--tw-gradient-via, transparent), var(--tw-gradient-to, transparent)); }
.bg-gradient-to-b { background-image: linear-gradient(to bottom, var(--tw-gradient-from, transparent), var(--tw-gradient-via, transparent), var(--tw-gradient-to, transparent)); }
.bg-gradient-to-br { background-image: linear-gradient(to bottom right, var(--tw-gradient-from, transparent), var(--tw-gradient-via, transparent), var(--tw-gradient-to, transparent)); }

.from-primary { --tw-gradient-from: var(--primary); --tw-gradient-via: var(--primary); --tw-gradient-to: var(--primary); }
.to-accent { --tw-gradient-to: var(--accent); }
.via-accent { --tw-gradient-via: var(--accent); }
.via-background { --tw-gradient-via: var(--background); }
.from-background { --tw-gradient-from: var(--background); --tw-gradient-via: var(--background); }
.from-background\/40 { --tw-gradient-from: color-mix(in srgb, var(--background) 40%, transparent); --tw-gradient-via: color-mix(in srgb, var(--background) 40%, transparent); }
.via-background\/20 { --tw-gradient-via: color-mix(in srgb, var(--background) 20%, transparent); }
.to-background\/80 { --tw-gradient-to: color-mix(in srgb, var(--background) 80%, transparent); }
.to-primary\/5 { --tw-gradient-to: color-mix(in srgb, var(--primary) 5%, transparent); }
.from-primary\/5 { --tw-gradient-from: color-mix(in srgb, var(--primary) 5%, transparent); }
.via-accent\/5 { --tw-gradient-via: color-mix(in srgb, var(--accent) 5%, transparent); }
.to-primary\/5\, { --tw-gradient-to: color-mix(in srgb, var(--primary) 5%, transparent); }
.from-primary\/20 { --tw-gradient-from: color-mix(in srgb, var(--primary) 20%, transparent); --tw-gradient-via: color-mix(in srgb, var(--primary) 20%, transparent); }
.to-accent\/20 { --tw-gradient-to: color-mix(in srgb, var(--accent) 20%, transparent); }
.from-primary\/10 { --tw-gradient-from: color-mix(in srgb, var(--primary) 10%, transparent); --tw-gradient-via: color-mix(in srgb, var(--primary) 10%, transparent); }
.to-accent\/10 { --tw-gradient-to: color-mix(in srgb, var(--accent) 10%, transparent); }
.from-primary\/60 { --tw-gradient-from: color-mix(in srgb, var(--primary) 60%, transparent); --tw-gradient-via: color-mix(in srgb, var(--primary) 60%, transparent); }
.from-accent { --tw-gradient-from: var(--accent); --tw-gradient-via: var(--accent); }
.from-accent\/60 { --tw-gradient-from: color-mix(in srgb, var(--accent) 60%, transparent); --tw-gradient-via: color-mix(in srgb, var(--accent) 60%, transparent); }
.from-slate-50 { --tw-gradient-from: #f8fafc; --tw-gradient-via: #f8fafc; }
.to-slate-100 { --tw-gradient-to: #f1f5f9; }

.opacity-0 { opacity: 0; }
.opacity-30 { opacity: 0.3; }
.opacity-40 { opacity: 0.4; }
.opacity-50 { opacity: 0.5; }
.backdrop-blur-sm { backdrop-filter: blur(4px); }
.backdrop-blur-md { backdrop-filter: blur(12px); }
.blur-xl { filter: blur(24px); }
.shadow-md { box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }
.shadow-lg { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }
.shadow-sm { box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); }

.cursor-pointer { cursor: pointer; }
.transform { transform: var(--tw-transform); }
.transition-all { transition-property: all; transition-duration: 150ms; }
.transition-colors { transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-duration: 150ms; }
.duration-200 { transition-duration: 200ms; }
.duration-300 { transition-duration: 300ms; }

/* Hover utilities */
.hover\:text-primary:hover { color: var(--primary); }
.hover\:border-primary\/50:hover { border-color: color-mix(in srgb, var(--primary) 50%, transparent); }
.hover\:border-primary\/30:hover { border-color: color-mix(in srgb, var(--primary) 30%, transparent); }
.hover\:shadow-lg:hover { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }

/* Group hover utilities */
.group:hover .group-hover\:opacity-100 { opacity: 1; }
.group:hover .group-hover\:translate-x-1 { transform: translateX(0.25rem); }
.group:hover .group-hover\:text-primary { color: var(--primary); }
.group:hover .group-hover\:text-primary\/40 { color: color-mix(in srgb, var(--primary) 40%, transparent); }
.group:hover .group-hover\:border-primary\/30 { border-color: color-mix(in srgb, var(--primary) 30%, transparent); }
.group:hover .group-hover\:border-primary\/50 { border-color: color-mix(in srgb, var(--primary) 50%, transparent); }
.group:hover .group-hover\:from-primary\/40 { --tw-gradient-from: color-mix(in srgb, var(--primary) 40%, transparent); --tw-gradient-via: color-mix(in srgb, var(--primary) 40%, transparent); }
.group:hover .group-hover\:to-accent\/40 { --tw-gradient-to: color-mix(in srgb, var(--accent) 40%, transparent); }

/* Responsive utilities (mobile-first) */
@media (min-width: 640px) {
  .sm\:inline-flex { display: inline-flex; }
  .sm\:flex-row { flex-direction: row; }
}
@media (min-width: 768px) {
  .md\:flex { display: flex; }
  .md\:flex-row { flex-direction: row; }
  .md\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .md\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
  .md\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
  .md\:text-2xl { font-size: 1.5rem; line-height: 2rem; }
  .md\:text-5xl { font-size: 3rem; line-height: 1; }
  .md\:text-7xl { font-size: 4.5rem; line-height: 1; }
  .md\:py-32 { padding-top: 8rem; padding-bottom: 8rem; }
  .md\:px-8 { padding-left: 2rem; padding-right: 2rem; }
  .md\:mt-0 { margin-top: 0; }
}

@media (max-width: 640px) {
  h1 { font-size: 1.875rem; line-height: 2.25rem; }
  h2 { font-size: 1.5rem; line-height: 2rem; }
  h3 { font-size: 1.25rem; line-height: 1.75rem; }
}

/* Focus visible */
button:focus-visible, a:focus-visible { outline: 2px solid var(--primary); outline-offset: 2px; }
"#;
