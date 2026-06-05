"use client";

import { SectionSlot } from "@/features/ab-variant";
import { Hero } from "./hero";
import { Portfolio } from "./portfolio";
import { Research } from "./research";
import { Team } from "./team";

// Sections compose the page top-to-bottom. Instances are built once at module
// load; SectionSlot wires each to URL-hash variant switching (a/b/c). Section
// instances carry methods, so this stays a client module — they cross no
// server/client serialization boundary.
//
// Header and Footer are page chrome, not page sections: they live in the
// application layout (rendered around every view), so they're not listed here.
const SECTIONS = [new Hero(), new Portfolio(), new Research(), new Team()];

export function HomeView() {
  return (
    <div className="min-h-screen bg-main-black text-main-mist font-sans">
      {SECTIONS.map((s, i) => (
        <SectionSlot key={i} section={s} />
      ))}
    </div>
  );
}
