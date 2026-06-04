import { useState, useEffect, Suspense, createElement } from "react";
import type { ComponentType } from "react";

// The structural shape SectionSlot consumes — every concrete Section instance
// (Header, Hero, ...) satisfies it. Kept local so no new public alias leaks.
type SectionInstance = {
  name: string;
  a(): ComponentType;
  b(): ComponentType | null;
  c(): ComponentType | null;
};

// Read the active variant suffix for a given section name from the URL hash.
// `#team`   -> "a" (default),   `#team.b` -> "b",   `#team.c` -> "c".
// Any other hash (different section, or none) leaves every section on "a".
function variantFor(name: string, hash: string): "a" | "b" | "c" {
  const raw = hash.replace(/^#/, "");
  const [section, suffix] = raw.split(".");
  if (section !== name) return "a";
  if (suffix === "b") return "b";
  if (suffix === "c") return "c";
  return "a";
}

// Picks a section's variant from the URL hash and renders it.
// a() is a directly-imported component (in the main bundle, zero extra cost);
// b()/c() are React.lazy components emitted as their own chunks, fetched only
// when their variant is actually selected — wrapped in <Suspense> for that load.
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

  // Suspense is only meaningful for the lazy b()/c() chunks; harmless for a().
  return <Suspense fallback={null}>{createElement(Resolved)}</Suspense>;
}
