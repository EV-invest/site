import { trait } from "@traits-ts/core";
import type { ComponentType, LazyExoticComponent } from "react";

// A Section knows its anchor name and can render up to three variants.
// a() is the default, always present, and ships eagerly in the main bundle.
// b()/c() are optional A/B alternates: their return type is narrowed to
// LazyExoticComponent so the only thing the compiler accepts is a lazy() — an
// eager `return TeamB` is a type error. This keeps every non-default variant
// out of the initial bundle by construction.
// traits-ts has no "abstract" keyword — required members throw in the base
// default and each concrete section overrides them (error out on tainted
// state rather than silently falling back).
export const Section = trait((base) => class extends base {
  get name(): string { throw new Error("Section.name must be overridden"); }
  a(): ComponentType { throw new Error("Section.a() must be overridden"); }
  b(): LazyExoticComponent<ComponentType> | null { return null; }
  c(): LazyExoticComponent<ComponentType> | null { return null; }
});
