import type { ReactNode } from "react";
import { type LucideIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { ASSETS } from "./assets";

// Named team members — name lives once and feeds both the heading and the image alt text.
export const TEAM = [
  {
    photo: ASSETS.team_member_1,
    name: "Elisey TODO",
    role: "Managing Partner, Co-founder",
    bio: "15+ years managing sovereign funds across Vietnam and Singapore. Former Investment Director at VinaCapital." //TODO: .
  },
  {
    photo: ASSETS.team_member_2,
    name: "Valeriy Sakharov",
    role: "Director of Research & Risk, CTO",
    bio: "Specializes in algorithmically-driven risk assessment and financial modelling. Previously at QuantM Alpha."
  }
];

// A grid cell: a 3:4 framed visual on top, a name/sub label below. The frame's
// contents vary per card (a photo, an icon prompt); the shell — aspect, border,
// hover group, and the heading/sub label block — is constant across all cards.
export function Card({ children, heading, headingClassName, sub }: {
  children: ReactNode;
  heading: string;
  headingClassName: string;
  sub: ReactNode;
}) {
  return (
    <div className="group space-y-4">
      <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-main-mist/10 bg-main-card">
        {children}
      </div>
      <div>
        <h4 className={`font-serif-display font-bold text-base ${headingClassName}`}>{heading}</h4>
        {sub}
      </div>
    </div>
  );
}

// The two non-person cards (open role, LP network) share one layout: a centred
// icon-disc + title + blurb + outline CTA, under a muted "Open Position"-style
// label. Only the icon, its accent colour, and the copy differ.
export function PlaceholderCard({ icon: Icon, iconClassName, title, body, cta, onCtaClick, heading, sub }: {
  icon: LucideIcon;
  iconClassName: string;
  title: string;
  body: string;
  cta: string;
  onCtaClick: () => void;
  heading: string;
  sub: string;
}) {
  return (
    <Card
      heading={heading}
      headingClassName="text-main-mist/40"
      sub={<Text variant="secondary" className="text-xs font-mono-tech mt-1">{sub}</Text>}
    >
      <div className="absolute inset-0 flex items-center justify-center p-6 text-center">
        <div className="space-y-4">
          <div className={`w-12 h-12 rounded-full bg-main-mist/5 border border-main-mist/10 flex items-center justify-center mx-auto ${iconClassName}`}>
            <Icon className="w-6 h-6" />
          </div>
          <div>
            <h5 className="font-serif-display font-bold text-white text-sm">{title}</h5>
            <Text variant="secondary" className="text-xs mt-2 font-light">
              {body}
            </Text>
          </div>
          <Button
            onClick={onCtaClick}
            variant="outline"
            className="border-main-mist/15 text-main-mist/80 hover:border-main-accent-t1 hover:text-main-accent-t1 text-xs py-1 h-auto bg-transparent"
          >
            {cta}
          </Button>
        </div>
      </div>
    </Card>
  );
}
