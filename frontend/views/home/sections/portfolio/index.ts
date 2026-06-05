import { derive } from "@traits-ts/core";
import { Section } from "@/shared/lib/section";
import { PortfolioA } from "./ui/portfolio";

export class Portfolio extends derive(Section) {
  get name() {
    return "portfolio";
  }
  a() {
    return PortfolioA;
  }
}
