import { derive } from "@traits-ts/core";
import type { ReactNode } from "react";
import { Users, Globe, type LucideIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { toast } from "sonner";
import { Section } from "./section";
import { ASSETS } from "./assets";

// Named team members — name lives once and feeds both the heading and the image alt text.
const TEAM = [
  {
    photo: ASSETS.team_member_1,
    name: "Elisey TODO",
    role: "Managing Partner, Co-founder",
    bio: "15+ years managing sovereign funds across Vietnam and Singapore. Former Investment Director at VinaCapital." //TODO: .
  },
  {
    photo: ASSETS.team_member_2,
    name: "Valeriy Sakharov",
    role: "Director of Research & Risk",
    bio: "Specializes in risk assessment and financial modelling of Emerging Markets investing. Previously at QuantM Alpha."
  }
];

// A grid cell: a 3:4 framed visual on top, a name/sub label below. The frame's
// contents vary per card (a photo, an icon prompt); the shell — aspect, border,
// hover group, and the heading/sub label block — is constant across all cards.
function Card({ children, heading, headingClassName, sub }: {
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
function PlaceholderCard({ icon: Icon, iconClassName, title, body, cta, onCtaClick, heading, sub }: {
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

export function TeamA() {
  const handlePlaceholderClick = (featureName: string) => {
    toast.info(`${featureName} — TODO`, {
      description: "TODO",
      duration: 3000
    });
  };

  return (
    <section id="team" className="py-24 relative border-t border-main-mist/10 bg-main-black">
      <div className="container">

        {/* Intro + boardroom image, split layout */}
        <div className="grid lg:grid-cols-12 gap-12 items-center mb-16">
          <div className="lg:col-span-6 space-y-4">
            <span className="text-xs font-mono-tech text-main-accent-t1 tracking-[0.3em] uppercase block">Management &amp; Expertise</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light">
              Led by <span className="italic text-main-accent-t1">Institutional Pioneers</span>
            </h2>
            <Text className="max-w-xl">
              The EV Investment team combines international experience in investment, risk management, and real estate development. Our goal is to deliver maximum transparency and returns for our partners.
            </Text>
          </div>
          <div className="lg:col-span-6">
            {/* Premium fund-office image */}
            <div className="relative aspect-[16/9] rounded-xl overflow-hidden border border-main-mist/10 shadow-2xl">
              <img
                src={ASSETS.office_interior}
                alt="EV Investment Boardroom"
                className="w-full h-full object-cover opacity-70"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-main-black/80 via-transparent to-transparent" />
              <div className="absolute bottom-4 left-4 right-4 flex justify-between items-end">
                <div className="space-y-1">
                  <span className="text-[10px] font-mono-tech text-main-accent-t1 uppercase tracking-wider">Head Office</span>
                  <h4 className="text-sm font-bold text-white">EV Boardroom • Quy Nhon</h4>
                </div>
                <Text asChild variant="secondary"><span className="text-[10px] font-mono-tech">Q1 2026</span></Text>
              </div>
            </div>
          </div>
        </div>

        {/* Team cards — Stronghold-style photo grid */}
        <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">

          {TEAM.map((member) => (
            <Card
              key={member.name}
              heading={member.name}
              headingClassName="text-white"
              sub={<p className="text-xs text-main-accent-t1 font-mono-tech mt-1">{member.role}</p>}
            >
              <img
                src={member.photo}
                alt={member.name}
                className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-main-black/85 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-6">
                <Text className="text-xs">
                  {member.bio}
                </Text>
              </div>
            </Card>
          ))}

          <PlaceholderCard
            icon={Users}
            iconClassName="text-main-accent-t1"
            title="Join Us"
            body="We are always looking for talented analysts and asset managers in Quy Nhon."
            cta="Careers"
            onCtaClick={() => handlePlaceholderClick("Careers")}
            heading="Open Position"
            sub="Investment Analyst"
          />

          <PlaceholderCard
            icon={Globe}
            iconClassName="text-main-accent-t3"
            title="LP Partner Network"
            body="Over 40 institutional investors across 12 countries trust us with their capital."
            cta="IR Contacts"
            onCtaClick={() => handlePlaceholderClick("IR Contacts")}
            heading="Investor Relations"
            sub="Investor Relations (IR)"
          />

        </div>

      </div>
    </section>
  );
}

export class Team extends derive(Section) {
  get name() { return "team"; }
  a() { return TeamA; }
}
