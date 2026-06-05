import { derive } from "@traits-ts/core";
import { Section } from "@/shared/lib/section";
import { HeroA } from "./ui/hero";

export class Hero extends derive(Section) {
  get name() {
    return "hero";
  }
  a() {
    return HeroA;
  }
}
