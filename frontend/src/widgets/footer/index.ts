import { derive } from "@traits-ts/core";
import { Section } from "@/shared/lib/section";
import { FooterA } from "./ui/footer";

export class Footer extends derive(Section) {
  get name() {
    return "footer";
  }
  a() {
    return FooterA;
  }
}
