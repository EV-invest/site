import { derive } from "@traits-ts/core";
import { Users, Globe } from "lucide-react";
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
    bio: "15+ years managing sovereign funds across Vietnam and Singapore. Former Investment Director at VinaCapital."
  },
  {
    photo: ASSETS.team_member_2,
    name: "Valeriy Sakharov",
    role: "Director of Research & Risk",
    bio: "Specializes in risk assessment and financial modelling for coastal development. Previously at CBRE Vietnam."
  }
];

export function TeamA() {
  const handlePlaceholderClick = (featureName: string) => {
    toast.info(`${featureName} — Концепт-интерфейс`, {
      description: "Данный элемент является частью интерактивного дизайн-макета.",
      duration: 3000
    });
  };

  // 5. TEAM SECTION (ported from good_team page — photo cards + boardroom).
  //    Back on navy base so it separates cleanly from the charcoal research band.
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
            <div key={member.name} className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-main-mist/10 bg-main-card">
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
              </div>
              <div>
                <h4 className="font-serif-display font-bold text-white text-base">{member.name}</h4>
                <p className="text-xs text-main-accent-t1 font-mono-tech mt-1">{member.role}</p>
              </div>
            </div>
          ))}

          {/* Open position card */}
          <div className="group space-y-4">
            <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-main-mist/10 bg-main-card flex items-center justify-center p-6 text-center">
              <div className="space-y-4">
                <div className="w-12 h-12 rounded-full bg-main-mist/5 border border-main-mist/10 flex items-center justify-center mx-auto text-main-accent-t1">
                  <Users className="w-6 h-6" />
                </div>
                <div>
                  <h5 className="font-serif-display font-bold text-white text-sm">Join Us</h5>
                  <Text variant="secondary" className="text-xs mt-2 font-light">
                    We are always looking for talented analysts and asset managers in Quy Nhon.
                  </Text>
                </div>
                <Button
                  onClick={() => handlePlaceholderClick("Careers")}
                  variant="outline"
                  className="border-main-mist/15 text-main-mist/80 hover:border-main-accent-t1 hover:text-main-accent-t1 text-xs py-1 h-auto bg-transparent"
                >
                  Careers
                </Button>
              </div>
            </div>
            <div>
              <h4 className="font-serif-display font-bold text-main-mist/40 text-base">Open Position</h4>
              <Text variant="secondary" className="text-xs font-mono-tech mt-1">Investment Analyst</Text>
            </div>
          </div>

          {/* LP network card */}
          <div className="group space-y-4">
            <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-main-mist/10 bg-main-card flex items-center justify-center p-6 text-center">
              <div className="space-y-4">
                <div className="w-12 h-12 rounded-full bg-main-mist/5 border border-main-mist/10 flex items-center justify-center mx-auto text-main-accent-t3">
                  <Globe className="w-6 h-6" />
                </div>
                <div>
                  <h5 className="font-serif-display font-bold text-white text-sm">LP Partner Network</h5>
                  <Text variant="secondary" className="text-xs mt-2 font-light">
                    Over 40 institutional investors across 12 countries trust us with their capital.
                  </Text>
                </div>
                <Button
                  onClick={() => handlePlaceholderClick("IR Contacts")}
                  variant="outline"
                  className="border-main-mist/15 text-main-mist/80 hover:border-main-accent-t1 hover:text-main-accent-t1 text-xs py-1 h-auto bg-transparent"
                >
                  IR Contacts
                </Button>
              </div>
            </div>
            <div>
              <h4 className="font-serif-display font-bold text-main-mist/40 text-base">Investor Relations</h4>
              <Text variant="secondary" className="text-xs font-mono-tech mt-1">Investor Relations (IR)</Text>
            </div>
          </div>

        </div>

      </div>
    </section>
  );
}

export class Team extends derive(Section) {
  get name() { return "team"; }
  a() { return TeamA; }
}
