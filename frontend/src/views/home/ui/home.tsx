"use client";

import { SectionSlot } from "@/features/ab-variant";
import { Header } from "@/widgets/header";
import { Hero } from "@/widgets/hero";
import { Portfolio } from "@/widgets/portfolio";
import { Research } from "@/widgets/research";
import { Team } from "@/widgets/team";
import { Footer } from "@/widgets/footer";

// Sections compose the page top-to-bottom. Instances are built once at module
// load; SectionSlot wires each to URL-hash variant switching (a/b/c). Section
// instances carry methods, so this stays a client module — they cross no
// server/client serialization boundary.
const SECTIONS = [
  new Header(),
  new Hero(),
  new Portfolio(),
  new Research(),
  new Team(),
  new Footer(),
];

export function HomeView() {
  return (
    <div className="min-h-screen bg-main-black text-main-mist font-sans">
      {/* Header and Hero share the "hero" anchor, so key by index, not name. */}
      {SECTIONS.map((s, i) => (
        <SectionSlot key={i} section={s} />
      ))}
    </div>
  );
}
