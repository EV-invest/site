import { SectionSlot } from "./variant";
import { Header } from "./Header";
import { Hero } from "./Hero";
import { Portfolio } from "./Portfolio";
import { Research } from "./Research";
import { Team } from "./Team";
import { Footer } from "./Footer";

// Sections compose the page top-to-bottom. Instances are built once at module
// load; SectionSlot wires each to URL-hash variant switching (a/b/c).
const SECTIONS = [new Header(), new Hero(), new Portfolio(), new Research(), new Team(), new Footer()];

export default function Home() {
  return (
    <div className="min-h-screen bg-main-black text-main-mist font-sans">
      {/* Header and Hero share the "hero" anchor, so key by index, not name. */}
      {SECTIONS.map((s, i) => <SectionSlot key={i} section={s} />)}
    </div>
  );
}
