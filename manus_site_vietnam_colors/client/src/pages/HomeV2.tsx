import React, { useState, useEffect, useRef } from "react";
import { 
  Building2, 
  TrendingUp, 
  Users, 
  BookOpen, 
  ArrowUpRight, 
  MapPin, 
  Layers, 
  Percent, 
  Compass, 
  ExternalLink, 
  ChevronRight, 
  ChevronLeft,
  Briefcase,
  Globe,
  ArrowRight,
  ShieldCheck,
  FileText,
  LineChart
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import Logo from "@/components/Logo";

const ASSETS = {
  quynhon_future: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/quynhon_future-ExoshVjhhPWYbYR4Zf3xJn.webp",
  luxury_villa: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/luxury_villa-64wseo7dGJUQNbg7HMSNPo.webp"
};

export default function HomeV2() {
  const [zoomLevel, setZoomLevel] = useState(1);
  const [hasScrolled, setHasScrolled] = useState(false);
  const [activeResearch, setActiveResearch] = useState(0);
  const [calculatorInputs, setCalculatorInputs] = useState({
    amount: 500000,
    term: 7,
    type: "commercial"
  });

  const heroRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleScroll = () => {
      if (!heroRef.current) return;
      const scrollY = window.scrollY;
      const threshold = window.innerHeight;
      
      setHasScrolled(scrollY > 50);

      if (scrollY < threshold) {
        const factor = 1 + (scrollY / threshold) * 1.8;
        setZoomLevel(factor);
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const handlePlaceholderClick = (featureName: string) => {
    toast.info(`${featureName} — Концепт-интерфейс`, {
      description: "Данный элемент является частью интерактивного дизайн-макета.",
      duration: 3000
    });
  };

  const calculateROI = () => {
    const rate = calculatorInputs.type === "residential" ? 0.092 : 0.135;
    const appreciation = calculatorInputs.type === "residential" ? 0.14 : 0.19;
    const totalReturn = calculatorInputs.amount * Math.pow(1 + (rate + appreciation), calculatorInputs.term);
    const profit = totalReturn - calculatorInputs.amount;
    const roi = ((profit / calculatorInputs.amount) * 100).toFixed(1);
    return {
      total: totalReturn.toLocaleString("en-US", { maximumFractionDigits: 0 }),
      profit: profit.toLocaleString("en-US", { maximumFractionDigits: 0 }),
      roi
    };
  };

  const calculated = calculateROI();

  return (
    <div className="min-h-screen bg-bg-deep text-text-custom font-sans selection:bg-brand selection:text-white">
      
      {/* FLOATING VERSION SWITCHER */}
      <div className="fixed bottom-6 right-6 z-50 bg-surface/90 backdrop-blur-md border border-border-custom p-1.5 flex gap-1 shadow-2xl">
        <a href="/" className="px-3 py-1.5 text-[10px] font-mono-tech uppercase tracking-widest text-muted-custom hover:text-white transition-colors">V1: Horizon</a>
        <a href="/v2" className="px-3 py-1.5 text-[10px] font-mono-tech uppercase tracking-widest bg-brand text-white font-bold shadow-md">V2: Alpha</a>
        <a href="/v3" className="px-3 py-1.5 text-[10px] font-mono-tech uppercase tracking-widest text-muted-custom hover:text-white transition-colors">V3: Sovereign</a>
      </div>

      {/* HEADER */}
      <header className={`fixed top-0 left-0 w-full z-50 transition-all duration-500 border-b ${
        hasScrolled 
          ? "bg-bg-deep/95 backdrop-blur-md border-border-custom py-3" 
          : "bg-transparent border-transparent py-5"
      }`}>
        <div className="container flex justify-between items-center">
          <div className="flex items-center gap-3">
            <Logo className="w-14 h-14 text-white hover:scale-105 transition-transform duration-300" color="#8EB8FE" />
            <div className="flex flex-col">
              <span className="font-serif-display font-bold text-lg tracking-wider text-white">EV INVESTMENT</span>
              <span className="text-[9px] font-mono-tech tracking-[0.3em] text-brand uppercase">Institutional Alpha</span>
            </div>
          </div>

          <nav className="hidden lg:flex items-center gap-8 font-mono-tech text-xs tracking-widest uppercase">
            <a href="#hero" className="hover:text-brand transition-colors text-subtle-custom">Overview</a>
            <a href="#holdings" className="hover:text-brand transition-colors text-subtle-custom">Holdings</a>
            <a href="#terminal" className="hover:text-brand transition-colors text-subtle-custom">Yield Terminal</a>
            <a href="#research" className="hover:text-brand transition-colors text-subtle-custom">Research</a>
            <a href="#team" className="hover:text-brand transition-colors text-subtle-custom">Advisory</a>
          </nav>

          <Button 
            className="font-mono-tech text-xs tracking-wider bg-brand text-white hover:bg-white hover:text-bg-deep transition-all duration-300"
            onClick={() => handlePlaceholderClick("Institutional Portal Login")}
          >
            Client Portal
          </Button>
        </div>
      </header>

      {/* HERO SECTION WITH ZOOM METAPHOR */}
      <section 
        id="hero" 
        ref={heroRef} 
        className="relative h-screen flex flex-col justify-center items-start overflow-hidden z-10 pt-24"
      >
        <div 
          className="absolute inset-0 z-0 transition-transform duration-150 ease-out"
          style={{ 
            transform: `scale(${zoomLevel})`,
            backgroundImage: `linear-gradient(to right, rgba(9, 11, 15, 0.95) 30%, rgba(9, 11, 15, 0.6)), url(${ASSETS.quynhon_future})`,
            backgroundSize: "cover",
            backgroundPosition: "center"
          }}
        />

        <div className="container relative z-10 max-w-5xl">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
            <div className="lg:col-span-8 space-y-8">
              <span className="text-xs font-mono-tech text-brand tracking-[0.3em] uppercase block">
                // INSTITUTIONAL ASSET MANAGEMENT
              </span>
              
              <h1 className="text-4xl sm:text-6xl font-serif-display font-light text-white leading-tight">
                Capital Preservation.<br />
                <span className="italic text-brand">Coastal Alpha.</span>
              </h1>
              
              <div className="h-0.5 w-24 bg-brand" />
              
              <p className="text-sm sm:text-base text-subtle-custom font-light max-w-xl leading-relaxed">
                We identify and secure institutional-grade coastal developments in Quy Nhon, Vietnam. Our mathematical model targets capital appreciation through infrastructure inflection points.
              </p>

              <div className="flex flex-wrap gap-4 pt-4">
                <Button 
                  className="bg-brand text-white hover:bg-white hover:text-bg-deep transition-all duration-300 font-mono-tech text-xs tracking-widest uppercase px-8 py-6 rounded-none"
                  onClick={() => {
                    const portfolioSec = document.getElementById("holdings");
                    portfolioSec?.scrollIntoView({ behavior: "smooth" });
                  }}
                >
                  View Holdings
                </Button>
                <Button 
                  variant="outline" 
                  className="border-white/20 text-white hover:bg-white/10 font-mono-tech text-xs tracking-widest uppercase px-8 py-6 rounded-none"
                  onClick={() => handlePlaceholderClick("Download Institutional Prospectus")}
                >
                  Fund Prospectus <FileText className="w-4 h-4 ml-2" />
                </Button>
              </div>
            </div>

            <div className="lg:col-span-4 hidden lg:flex flex-col items-center justify-center p-8 bg-surface/60 border border-border-custom backdrop-blur-sm">
              <Logo className="w-44 h-44 text-white mb-6" color="#8EB8FE" />
              <p className="text-[10px] font-mono-tech text-muted-custom tracking-widest uppercase text-center leading-relaxed">
                ESTABLISHED 2026<br />QUY NHON, VIETNAM
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* PORTFOLIO SECTION (Holdings Grid) */}
      <section id="holdings" className="py-24 relative border-t border-border-custom bg-bg-deep">
        <div className="container max-w-5xl">
          <div className="text-center max-w-2xl mx-auto mb-20">
            <span className="text-xs font-mono-tech text-brand tracking-[0.3em] uppercase block mb-3">Investment Strategy</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light mb-6">
              Strategic <span className="italic text-brand">Holdings</span>
            </h2>
            <p className="text-sm text-subtle-custom font-light leading-relaxed">
              Our portfolio consists of limited, high-conviction coastal developments structured to deliver superior risk-adjusted returns.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
            <div className="border border-border-custom bg-surface p-8 space-y-6">
              <div className="aspect-[16/10] bg-cover bg-center" style={{ backgroundImage: `url(${ASSETS.luxury_villa})` }} />
              <div className="flex justify-between items-start">
                <div>
                  <span className="text-[10px] font-mono-tech text-brand uppercase tracking-widest block mb-1">Residential Pool</span>
                  <h3 className="text-2xl font-serif-display text-white">The Horizon Premium Villas</h3>
                </div>
                <span className="text-sm font-mono-tech text-success-custom font-bold">13.5% IRR</span>
              </div>
              <p className="text-xs text-subtle-custom font-light leading-relaxed">
                Ultra-luxury oceanfront villas with private beach access, targeting high-net-worth domestic and international investors.
              </p>
              <Button 
                variant="outline" 
                className="w-full border-border-custom text-white hover:bg-white hover:text-bg-deep rounded-none py-5"
                onClick={() => handlePlaceholderClick("View Villa Deal Sheet")}
              >
                Request Access
              </Button>
            </div>

            <div className="border border-border-custom bg-surface p-8 space-y-6 flex flex-col justify-between">
              <div className="space-y-6">
                <div className="aspect-[16/10] bg-cover bg-center" style={{ backgroundImage: `url(${ASSETS.quynhon_future})` }} />
                <div className="flex justify-between items-start">
                  <div>
                    <span className="text-[10px] font-mono-tech text-brand uppercase tracking-widest block mb-1">Land Development</span>
                    <h3 className="text-2xl font-serif-display text-white">Nhon Hoi Coastal Expansion</h3>
                  </div>
                  <span className="text-sm font-mono-tech text-success-custom font-bold">24.6% IRR</span>
                </div>
                <p className="text-xs text-subtle-custom font-light leading-relaxed">
                  Strategic land banking adjacent to major infrastructure nodes, structured for maximum capital appreciation.
                </p>
              </div>
              <Button 
                variant="outline" 
                className="w-full border-border-custom text-white hover:bg-white hover:text-bg-deep rounded-none py-5"
                onClick={() => handlePlaceholderClick("View Land Deal Sheet")}
              >
                Request Access
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* YIELD TERMINAL (Interactive Calculator) */}
      <section id="terminal" className="py-24 border-t border-border-custom bg-surface">
        <div className="container max-w-5xl">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
            <div className="lg:col-span-5 space-y-6">
              <span className="text-xs font-mono-tech text-brand tracking-[0.3em] uppercase block">Yield Terminal</span>
              <h2 className="text-3xl sm:text-4xl font-serif-display text-white font-light">
                Run Your <span className="italic text-brand">Simulation</span>
              </h2>
              <p className="text-sm text-subtle-custom font-light leading-relaxed">
                Interact with our financial projection model. Adjust your principal and term to simulate capital appreciation in the Quy Nhon economic zone.
              </p>

              <div className="space-y-6 pt-4 font-mono-tech text-xs">
                <div>
                  <label className="text-muted-custom uppercase block mb-2">Principal Investment ($ USD)</label>
                  <input 
                    type="range" 
                    min="100000" 
                    max="2000000" 
                    step="50000"
                    value={calculatorInputs.amount}
                    onChange={(e) => setCalculatorInputs({...calculatorInputs, amount: Number(e.target.value)})}
                    className="w-full accent-brand bg-bg-deep"
                  />
                  <div className="flex justify-between text-brand font-bold mt-1">
                    <span>$100k</span>
                    <span className="text-sm">${calculatorInputs.amount.toLocaleString()}</span>
                    <span>$2M</span>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="text-muted-custom uppercase block mb-2">Term (Years)</label>
                    <select 
                      value={calculatorInputs.term}
                      onChange={(e) => setCalculatorInputs({...calculatorInputs, term: Number(e.target.value)})}
                      className="w-full bg-bg-deep border border-border-custom p-2 text-white"
                    >
                      {[3, 5, 7, 10].map(y => (
                        <option key={y} value={y}>{y} Years</option>
                      ))}
                    </select>
                  </div>
                  <div>
                    <label className="text-muted-custom uppercase block mb-2">Asset Type</label>
                    <select 
                      value={calculatorInputs.type}
                      onChange={(e) => setCalculatorInputs({...calculatorInputs, type: e.target.value})}
                      className="w-full bg-bg-deep border border-border-custom p-2 text-white"
                    >
                      <option value="residential">Luxury Villa</option>
                      <option value="commercial">Commercial Hub</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div className="lg:col-span-7 bg-bg-deep border border-border-custom p-8 sm:p-12 flex flex-col justify-between h-full">
              <div className="space-y-8">
                <div className="flex items-center gap-3 text-brand font-mono-tech text-xs uppercase tracking-wider">
                  <LineChart className="w-4 h-4" /> Live Yield Simulation
                </div>
                <div>
                  <span className="text-[10px] font-mono-tech text-muted-custom uppercase block mb-1">Estimated ROI</span>
                  <span className="text-5xl sm:text-6xl font-serif-display text-brand font-bold">{calculated.roi}%</span>
                </div>
                <div className="grid grid-cols-2 gap-6 border-t border-border-custom pt-6">
                  <div>
                    <span className="text-[9px] font-mono-tech text-muted-custom uppercase block mb-0.5">Total Payout</span>
                    <span className="text-xl font-mono-tech text-white font-bold">${calculated.total}</span>
                  </div>
                  <div>
                    <span className="text-[9px] font-mono-tech text-muted-custom uppercase block mb-0.5">Net Profit</span>
                    <span className="text-xl font-mono-tech text-success-custom font-bold">${calculated.profit}</span>
                  </div>
                </div>
              </div>

              <div className="mt-8 border-t border-border-custom pt-6">
                <Button 
                  className="w-full bg-brand text-white hover:bg-white hover:text-bg-deep rounded-none font-mono-tech text-xs tracking-wider uppercase py-6"
                  onClick={() => handlePlaceholderClick("Request Advisory Session")}
                >
                  Request Advisory Session
                </Button>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* RESEARCH SECTION (Bridgewater Focus) */}
      <section id="research" className="py-24 bg-brand-hi text-bg-deep relative overflow-hidden">
        <div className="container max-w-5xl">
          <div className="max-w-2xl mb-16">
            <span className="text-xs font-mono-tech text-brand-blue tracking-[0.3em] uppercase block mb-3">Academic Rigor</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-bg-deep font-light leading-tight">
              Bridgewater-Grade <span className="italic text-brand-blue">Research & Insights</span>
            </h2>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div className="lg:col-span-1 space-y-4">
              {[
                { title: "Vietnam Coastal Macro Report 2026", cat: "Macroeconomics" },
                { title: "Quy Nhon Infrastructure Masterplan", cat: "Urban Planning" }
              ].map((report, idx) => (
                <div 
                  key={idx}
                  onClick={() => setActiveResearch(idx)}
                  className={`p-6 border cursor-pointer transition-all duration-300 ${
                    activeResearch === idx 
                      ? "bg-white border-brand-blue shadow-md" 
                      : "border-bg-deep/10 hover:border-bg-deep/30 bg-transparent"
                  }`}
                >
                  <span className="text-[10px] font-mono-tech text-brand-blue uppercase tracking-widest block mb-2">{report.cat}</span>
                  <h4 className="font-serif-display text-lg text-bg-deep font-bold">{report.title}</h4>
                </div>
              ))}
            </div>

            <div className="lg:col-span-2 bg-white border border-bg-deep/10 p-8 sm:p-12 flex flex-col justify-between shadow-lg">
              <div>
                <h3 className="text-2xl sm:text-3xl font-serif-display text-bg-deep font-bold mb-6">
                  {activeResearch === 0 ? "Vietnam Coastal Macro Report 2026" : "Quy Nhon Infrastructure Masterplan"}
                </h3>
                <p className="text-sm text-bg-deep/80 font-light leading-relaxed mb-6">
                  Our comprehensive macroeconomic analysis highlights the shift of institutional capital toward coastal secondary cities, with Quy Nhon showing unprecedented growth vectors.
                </p>
              </div>
              <Button 
                className="bg-bg-deep text-white hover:bg-brand-blue transition-all duration-300 rounded-none font-mono-tech text-xs tracking-wider uppercase py-5"
                onClick={() => handlePlaceholderClick("Download Report")}
              >
                Download Full Report <ArrowUpRight className="w-4 h-4 ml-2" />
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* ADVISORY TEAM SECTION */}
      <section id="team" className="py-24 relative border-t border-border-custom">
        <div className="container max-w-5xl">
          <div className="text-center max-w-2xl mx-auto mb-20">
            <span className="text-xs font-mono-tech text-brand tracking-[0.3em] uppercase block mb-3">Trust & Transparency</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light">
              Advisory <span className="italic text-brand">Partners</span>
            </h2>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {[
              { name: "Minh Hoang", role: "Managing Partner", init: "MH" },
              { name: "Dr. Nguyen An", role: "Chief Economist", init: "NA" },
              { name: "Elena Vostrikova", role: "Head of International Capital", init: "EV" }
            ].map((member, idx) => (
              <div key={idx} className="border border-border-custom bg-surface p-8 text-center space-y-6">
                <div className="w-20 h-20 bg-bg-deep border border-border-custom flex items-center justify-center font-serif-display text-2xl text-brand mx-auto">
                  {member.init}
                </div>
                <div>
                  <h3 className="text-xl font-serif-display text-white font-bold mb-1">{member.name}</h3>
                  <p className="text-xs font-mono-tech text-brand uppercase tracking-widest">{member.role}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* FOOTER */}
      <footer className="bg-bg-deep border-t border-border-custom py-16 text-center text-[10px] font-mono-tech text-muted-custom">
        <div className="container">
          <Logo className="w-12 h-12 text-white mx-auto mb-6" color="#8EB8FE" />
          <p>© 2026 EV Investment. All rights reserved.</p>
          <p className="mt-2">Designed for Bullmart Investment Fund.</p>
        </div>
      </footer>

    </div>
  );
}
