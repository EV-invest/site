import { trait } from "@traits-ts/core";
import type { ComponentType } from "react";

// A Section knows its anchor name and can render up to three variants.
// a() is the default, always present. b()/c() are optional A/B alternates.
// traits-ts has no "abstract" keyword — required members throw in the base
// default and each concrete section overrides them (error out on tainted
// state rather than silently falling back).
export const Section = trait((base) => class extends base {
  get name(): string { throw new Error("Section.name must be overridden"); }
  a(): ComponentType { throw new Error("Section.a() must be overridden"); }
  b(): ComponentType | null { return null; }
  c(): ComponentType | null { return null; }
});
