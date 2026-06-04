import { derive } from "@traits-ts/core";
import { Section } from "@/shared/lib/section";
import { HeaderA } from "./ui/header";

// Header shares the "hero" anchor with the Hero section by design.
export class Header extends derive(Section) {
  get name() {
    return "hero";
  }
  a() {
    return HeaderA;
  }
}
