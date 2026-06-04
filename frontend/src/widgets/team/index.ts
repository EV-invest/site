import { derive } from "@traits-ts/core";
import { lazy } from "react";
import { Section } from "@/shared/lib/section";
import { TeamA } from "./ui/team-a";

// Variant B ships as its own lazy chunk — never bundled for default-A visitors,
// only fetched when dev cycles to it (l/h on hover).
const TeamB = lazy(() => import("./ui/team-b"));

export class Team extends derive(Section) {
  get name() {
    return "team";
  }
  a() {
    return TeamA;
  }
  b() {
    return TeamB;
  }
}
