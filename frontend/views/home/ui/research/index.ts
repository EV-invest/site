import { derive } from "@traits-ts/core";
import { Section } from "@/shared/lib/section";
import { ResearchA } from "./ui/research";

export class Research extends derive(Section) {
  get name() {
    return "research";
  }
  a() {
    return ResearchA;
  }
}
