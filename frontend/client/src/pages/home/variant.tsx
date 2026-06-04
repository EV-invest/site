import { useState, useEffect, Suspense, createElement } from "react";
import type { ComponentType, LazyExoticComponent, ReactNode } from "react";

// The structural shape SectionSlot consumes — every concrete Section instance
// (Header, Hero, ...) satisfies it. Kept local so no new public alias leaks.
// b()/c() are lazy-only (their chunks load on demand); a() is eager.
type SectionInstance = {
  name: string;
  a(): ComponentType;
  b(): LazyExoticComponent<ComponentType> | null;
  c(): LazyExoticComponent<ComponentType> | null;
};

type Variant = "a" | "b" | "c";

// Read the active variant suffix for a given section name from the URL hash.
// `#team`   -> "a" (default),   `#team.b` -> "b",   `#team.c` -> "c".
// Any other hash (different section, or none) leaves every section on "a".
function variantFor(name: string, hash: string): Variant {
  const raw = hash.replace(/^#/, "");
  const [section, suffix] = raw.split(".");
  if (section !== name) return "a";
  if (suffix === "b") return "b";
  if (suffix === "c") return "c";
  return "a";
}

// The variants a section actually implements, in cycle order. a() is always
// present; b()/c() return null when absent and are dropped here so cycling
// only ever lands on a real variant.
function availableVariants(section: SectionInstance): Variant[] {
  const present: Variant[] = ["a"];
  if (section.b()) present.push("b");
  if (section.c()) present.push("c");
  return present;
}

// Picks a section's variant from the URL hash and renders it.
// a() is a directly-imported component (in the main bundle, zero extra cost) —
// the only variant prod ever shows. b()/c() are React.lazy chunks, network-
// fetched only when dev cycles to them (Alt+l/Alt+h) — wrapped in <Suspense>
// for that load. Off the default path on purpose: visitors never pay for them.
export function SectionSlot({ section }: { section: SectionInstance }) {
  const [hash, setHash] = useState(window.location.hash);

  useEffect(() => {
    const onHashChange = () => setHash(window.location.hash);
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  }, []);

  const variant = variantFor(section.name, hash);
  const Component = variant === "b" ? section.b() : variant === "c" ? section.c() : section.a();

  // b()/c() default to null (no such variant) -> fall back to the default a().
  const Resolved = Component ?? section.a();

  const rendered =
    // Suspense is only meaningful for the lazy b()/c() chunks; harmless for a().
    <Suspense fallback={null}>{createElement(Resolved)}</Suspense>;

  // Production renders the bare slot; the dev-only A/B cycling wrapper adds
  // nothing to the shipped bundle.
  if (!import.meta.env.DEV) return rendered;
  return <DevVariantToggle section={section} variant={variant}>{rendered}</DevVariantToggle>;
}

// Dev-only: hover a section, then `l` / `h` cycles its variant by rewriting
// the URL hash (`#team.b`), reusing the same hash->variant pipeline the slot
// already reads. Cycling wraps around and only visits implemented variants.
function DevVariantToggle({ section, variant, children }: {
  section: SectionInstance;
  variant: Variant;
  children: ReactNode;
}) {
  const [focused, setFocused] = useState(false);

  useEffect(() => {
    if (!focused) return;
    const onKeyDown = (e: KeyboardEvent) => {
      // Bare h/l — but never steal keystrokes while typing into a field.
      const el = e.target as HTMLElement | null;
      if (el?.isContentEditable || el?.closest("input, textarea, select")) return;
      const step = e.key === "l" ? 1 : e.key === "h" ? -1 : 0;
      if (step === 0) return;
      e.preventDefault();

      const variants = availableVariants(section);
      const idx = variants.indexOf(variant);
      const next = variants[(idx + step + variants.length) % variants.length];
      // "a" is the bare anchor (`#team`); b/c carry the suffix (`#team.b`).
      window.location.hash = next === "a" ? section.name : `${section.name}.${next}`;
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [focused, section, variant]);

  return (
    <div
      onMouseEnter={() => setFocused(true)}
      onMouseLeave={() => setFocused(false)}
      className={focused ? "outline-dashed outline-1 outline-main-accent-t1/40" : undefined}
    >
      {children}
    </div>
  );
}
