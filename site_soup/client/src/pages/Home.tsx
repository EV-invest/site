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
  Globe,
  ArrowRight,
  FileText
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";

// Ссылки на сгенерированные высококачественные изображения
const ASSETS = {
  quynhon_future: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/quynhon_future-ExoshVjhhPWYbYR4Zf3xJn.webp",
  luxury_villa: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/luxury_villa-64wseo7dGJUQNbg7HMSNPo.webp",
  abstract_investment: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/abstract_investment-eYjGPMMXA3Hkv5fdUCnFfs.webp",
  // Ported from manus_site_good_team_page — the team layout we liked.
  office_interior: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/office_interior-SxhsdYStJCspKeR4RjpDbW.webp",
  team_member_1: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/team_member_1-hJnPwkoXB2TsmzqEqY9yWb.webp",
  team_member_2: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/team_member_2-cdQ8SpsdHCexzaZSqvgtaE.webp"
};

export default function Home() {
  const [zoomLevel, setZoomLevel] = useState(1);
  const [hasScrolled, setHasScrolled] = useState(false);
  const [activeTab, setActiveTab] = useState("all");
  const [activeResearch, setActiveResearch] = useState(0);
  const [calculatorInputs, setCalculatorInputs] = useState({
    amount: 100000,
    term: 5,
    type: "residential"
  });

  const heroRef = useRef<HTMLDivElement>(null);

  // Обработка интерактивного зума (Stronghold Fund Metaphor)
  useEffect(() => {
    const handleScroll = () => {
      if (!heroRef.current) return;
      const scrollY = window.scrollY;
      const threshold = window.innerHeight;
      
      if (scrollY > 50) {
        setHasScrolled(true);
      } else {
        setHasScrolled(false);
      }

      if (scrollY < threshold) {
        // Рассчитываем зум от 1 до 4 в зависимости от прокрутки
        const factor = 1 + (scrollY / threshold) * 3;
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

  // Вычисление показателей для интерактивного калькулятора (Binance/Fintech Style)
  const calculateROI = () => {
    const rate = calculatorInputs.type === "residential" ? 0.085 : 0.12;
    const appreciation = calculatorInputs.type === "residential" ? 0.15 : 0.18;
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
    <div className="min-h-screen bg-viet-black text-viet-mist font-sans">
      
      {/* 1. HEADER (Minimalist, floating) */}
      <header className={`fixed top-0 left-0 w-full z-50 transition-all duration-500 border-b ${
        hasScrolled
          ? "bg-viet-black/90 backdrop-blur-md border-viet-mist/10 py-4"
          : "bg-transparent border-transparent py-6"
      }`}>{/* navy-led: gold chrome retired in favour of Ha Long Teal accent */}
        <div className="container flex justify-between items-center">
          <div className="flex items-center gap-3">
            {/* SVG Logo - Воспроизведение структуры вашего логотипа */}
            <svg className="w-10 h-10 text-white" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M20 50 L50 35 L80 50" stroke="white" strokeWidth="2" strokeLinecap="round" />
              <rect x="38" y="38" width="10" height="20" fill="white" />
              <rect x="52" y="30" width="12" height="28" fill="white" />
              <rect x="68" y="42" width="8" height="16" fill="white" />
              <path d="M15 55 L85 55" stroke="white" strokeWidth="2" />
              <text x="50" y="80" fill="white" fontSize="18" fontWeight="bold" textAnchor="middle" letterSpacing="2">EV</text>
            </svg>
            <div className="flex flex-col">
              <span className="font-serif-display font-bold text-lg tracking-wider text-white">EV INVESTMENT</span>
              <span className="text-[9px] font-mono-tech tracking-[0.3em] text-viet-teal uppercase">Quy Nhon Fund</span>
            </div>
          </div>

          <nav className="hidden md:flex items-center gap-8 font-mono-tech text-xs tracking-widest uppercase">
            <a href="#hero" className="hover:text-viet-teal transition-colors">Home</a>
            <a href="#portfolio" className="hover:text-viet-teal transition-colors">Portfolio</a>
            <a href="#calculator" className="hover:text-viet-teal transition-colors">Terminal</a>
            <a href="#research" className="hover:text-viet-teal transition-colors">Research</a>
            <a href="#team" className="hover:text-viet-teal transition-colors">Team</a>
          </nav>

          <div className="flex items-center gap-4">
            <Button
              variant="outline"
              className="hidden sm:inline-flex font-mono-tech text-xs tracking-wider border-viet-teal text-viet-teal hover:bg-viet-teal hover:text-viet-black transition-all duration-300 bg-transparent"
              onClick={() => handlePlaceholderClick("Investor Portal Login")}
            >
              Investor Portal
            </Button>
          </div>
        </div>
      </header>

      {/* 2. HERO PAGE WITH ZOOM METAPHOR (Stronghold Fund Inspired) */}
      <section 
        id="hero" 
        ref={heroRef} 
        className="relative h-screen flex flex-col justify-center items-center overflow-hidden z-10"
      >
        {/* Background Image with Zoom */}
        <div 
          className="absolute inset-0 z-0 transition-transform duration-100 ease-out"
          style={{ 
            transform: `scale(${zoomLevel})`,
            backgroundImage: `linear-gradient(to bottom, rgba(7, 13, 24, 0.78), rgba(7, 13, 24, 0.96)), url(${ASSETS.quynhon_future})`,
            backgroundSize: "cover",
            backgroundPosition: "center"
          }}
        />

        {/* Floating Abstract Financial Grid Overlay */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#8080800a_1px,transparent_1px),linear-gradient(to_bottom,#8080800a_1px,transparent_1px)] bg-[size:14px_24px] pointer-events-none" />

        <div className="container relative z-10 text-center flex flex-col items-center justify-center h-full max-w-4xl px-4">
          
          {/* Integrated Logo & Offering (Stronghold Metaphor) */}
          <div 
            className="transition-all duration-700 ease-out"
            style={{ 
              transform: `scale(${Math.max(0.8, 1 - (zoomLevel - 1) * 0.15)})`,
              opacity: Math.max(0.1, 1 - (zoomLevel - 1) * 0.5)
            }}
          >
            <h1 className="text-4xl sm:text-6xl md:text-8xl font-serif-display font-light text-white leading-tight mb-6">
              Invest in <span className="italic text-viet-teal">Quy Nhon</span><br />
              Through Institutional Vision.
            </h1>
            
            <p className="text-sm sm:text-base md:text-lg text-viet-mist/70 font-light max-w-2xl mx-auto mb-12 leading-relaxed">
              EV Investment bridges the gap between premium coastal real estate development and sophisticated investors. Experience high-yield real estate assets in Vietnam’s fastest-growing coastal hub.
            </p>
          </div>

          {/* Interactive Action / Zoom Indicator */}
          <div className="flex flex-col items-center gap-4">
            <Button
              className="bg-viet-mist text-viet-navy hover:bg-viet-teal hover:text-viet-black hover:scale-105 active:scale-95 transition-all duration-300 font-mono-tech text-xs tracking-widest uppercase px-8 py-6 rounded-none"
              onClick={() => {
                const portfolioSec = document.getElementById("portfolio");
                portfolioSec?.scrollIntoView({ behavior: "smooth" });
              }}
            >
              Explore Assets <ArrowRight className="w-4 h-4 ml-2" />
            </Button>

            <div className="mt-8 flex flex-col items-center gap-1.5">
              <span className="text-[9px] font-mono-tech tracking-[0.3em] text-viet-mist/40 uppercase">Scroll to zoom in & discover</span>
              <div className="w-1 h-12 bg-gradient-to-b from-viet-teal to-transparent rounded-full animate-bounce" />
            </div>
          </div>
        </div>

        {/* Bottom stats ribbon — teal-led, with ONE gold highlight (Target IRR). */}
        <div className="absolute bottom-0 left-0 w-full bg-viet-black/80 border-t border-viet-mist/10 py-6 backdrop-blur-sm z-20">
          <div className="container grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
            <div>
              <p className="text-xs font-mono-tech text-viet-mist/50 uppercase tracking-widest mb-1">Target IRR</p>
              <p className="text-2xl sm:text-3xl font-serif-display text-viet-gold font-bold">22.4% +</p>
            </div>
            <div>
              <p className="text-xs font-mono-tech text-viet-mist/50 uppercase tracking-widest mb-1">AUM Under Advisory</p>
              <p className="text-2xl sm:text-3xl font-serif-display text-white font-bold">$145M</p>
            </div>
            <div>
              <p className="text-xs font-mono-tech text-viet-mist/50 uppercase tracking-widest mb-1">Coastal Coastline</p>
              <p className="text-2xl sm:text-3xl font-serif-display text-viet-teal font-bold">72 km</p>
            </div>
            <div>
              <p className="text-xs font-mono-tech text-viet-mist/50 uppercase tracking-widest mb-1">Institutional Grade</p>
              <p className="text-2xl sm:text-3xl font-serif-display text-viet-pink font-bold">100%</p>{/* Lotus jewel #1 — the punchline stat */}
            </div>
          </div>
        </div>
      </section>

      {/* 3. PORTFOLIO SECTION (Bento Grid, DWF Labs & Binance Inspired) */}
      <section id="portfolio" className="py-24 relative border-t border-viet-mist/10">
        <div className="container">
          
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-16">
            <div>
              <span className="text-xs font-mono-tech text-viet-teal tracking-[0.3em] uppercase block mb-3">Investment Scope</span>
              <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light">
                Premium Asset <span className="italic text-viet-teal">Portfolio</span>
              </h2>
            </div>
            <p className="text-sm text-viet-mist/60 font-light max-w-md mt-4 md:mt-0 leading-relaxed">
              Curated, premium, high-yield developments across Quy Nhon city, focusing on high appreciation seaside villas and urban luxury residences.
            </p>
          </div>

          {/* Filter Tabs (Binance/DWF style) */}
          <div className="flex flex-wrap gap-2 mb-12 border-b border-viet-mist/10 pb-4 font-mono-tech text-xs tracking-wider">
            {["all", "villas", "commercial", "land"].map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`px-5 py-2.5 uppercase transition-all duration-300 ${
                  activeTab === tab
                    ? "bg-viet-teal text-viet-black font-bold"
                    : "text-viet-mist/60 hover:text-white hover:bg-viet-mist/5"
                }`}
              >
                {tab}
              </button>
            ))}
          </div>

          {/* Bento Grid Layout */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            
            {/* Main Featured Asset (Large Box) */}
            <div className="md:col-span-2 relative overflow-hidden group border border-viet-mist/10 bg-viet-black/40 flex flex-col justify-end min-h-[450px]">
              <div 
                className="absolute inset-0 z-0 bg-cover bg-center transition-transform duration-700 group-hover:scale-105"
                style={{ backgroundImage: `linear-gradient(to top, rgba(7, 13, 24, 0.96) 10%, rgba(7, 13, 24, 0.2)), url(${ASSETS.luxury_villa})` }}
              />
              <div className="absolute top-4 right-4 bg-viet-teal text-viet-black font-mono-tech text-[10px] tracking-widest uppercase px-3 py-1.5 font-bold">
                Featured Deal
              </div>
              <div className="relative z-10 p-8">
                <div className="flex items-center gap-2 text-viet-teal font-mono-tech text-xs mb-3">
                  <MapPin className="w-3.5 h-3.5" /> Nhon Ly Beach, Quy Nhon
                </div>
                <h3 className="text-2xl sm:text-3xl font-serif-display text-white mb-4">The Horizon Premium Villas</h3>
                <p className="text-sm text-viet-mist/70 font-light max-w-xl mb-6">
                  Exclusive ultra-luxury oceanfront villas with private pools, nestled between pristine limestone cliffs and crystal-clear turquoise waters.
                </p>
                <div className="grid grid-cols-3 gap-4 border-t border-viet-mist/10 pt-6 max-w-md">
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Target Yield</span>
                    <span className="text-lg font-serif-display text-viet-green font-bold">12.5% p.a.</span>
                  </div>
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Appreciation</span>
                    <span className="text-lg font-serif-display text-viet-gold font-bold">18% YoY</span>{/* rare gold highlight */}
                  </div>
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Status</span>
                    <span className="text-lg font-serif-display text-white font-bold">Pre-Launch</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Side Asset 1 */}
            <div className="relative overflow-hidden group border border-viet-mist/10 bg-viet-black/40 flex flex-col justify-end min-h-[450px]">
              <div 
                className="absolute inset-0 z-0 bg-cover bg-center transition-transform duration-700 group-hover:scale-105"
                style={{ backgroundImage: `linear-gradient(to top, rgba(7, 13, 24, 0.96) 20%, rgba(7, 13, 24, 0.4)), url(${ASSETS.quynhon_future})` }}
              />
              <div className="relative z-10 p-8">
                <div className="flex items-center gap-2 text-viet-teal font-mono-tech text-xs mb-3">
                  <MapPin className="w-3.5 h-3.5" /> Quy Nhon Center
                </div>
                <h3 className="text-xl sm:text-2xl font-serif-display text-white mb-4">Quy Nhon Bay Residences</h3>
                <p className="text-xs text-viet-mist/70 font-light mb-6">
                  Premium high-rise apartments with panoramic views of the bay, integrating luxury amenities and smart-home technology.
                </p>
                <div className="flex justify-between items-center border-t border-viet-mist/10 pt-6">
                  <div>
                    <span className="text-[9px] font-mono-tech text-viet-mist/40 uppercase block mb-0.5">LTV Ratio</span>
                    <span className="text-sm font-serif-display text-white font-bold">55% Max</span>
                  </div>
                  <Button
                    variant="ghost"
                    className="p-0 text-viet-teal hover:text-white hover:bg-transparent font-mono-tech text-xs tracking-wider"
                    onClick={() => handlePlaceholderClick("View Deal Sheet")}
                  >
                    Deal Sheet <ArrowUpRight className="w-3.5 h-3.5 ml-1" />
                  </Button>
                </div>
              </div>
            </div>

            {/* Asset 3 (Modular Bento Grid item - Info heavy like Binance) */}
            <div className="border border-viet-mist/10 bg-viet-card p-8 flex flex-col justify-between">
              <div>
                <div className="inline-flex items-center gap-1.5 px-2 py-1 bg-viet-teal/10 text-viet-teal border border-viet-teal/20 text-[9px] font-mono-tech uppercase tracking-wider mb-6">
                  <TrendingUp className="w-3 h-3" /> Market Growth
                </div>
                <h3 className="text-xl font-serif-display text-white mb-4">Why Quy Nhon?</h3>
                <p className="text-sm text-viet-mist/70 font-light leading-relaxed mb-6">
                  Positioned as the new gateway of Central Vietnam, Quy Nhon is undergoing a multi-billion dollar infrastructure upgrade, transforming into a global science and beach tourism destination.
                </p>
              </div>
              <ul className="space-y-3 border-t border-viet-mist/10 pt-6 font-mono-tech text-xs">
                <li className="flex justify-between">
                  <span className="text-viet-mist/40">Infrastructure Investment:</span>
                  <span className="text-white font-bold">$2.4 Billion</span>
                </li>
                <li className="flex justify-between">
                  <span className="text-viet-mist/40">Tourism Growth Rate:</span>
                  <span className="text-viet-green font-bold">+28% YoY</span>
                </li>
                <li className="flex justify-between">
                  <span className="text-viet-mist/40">FDI Inflow (2025):</span>
                  <span className="text-viet-green font-bold">$420M</span>
                </li>
              </ul>
            </div>

            {/* Asset 4 (Interactive Yield/Term Calculator - Binance Fintech Style) */}
            <div id="calculator" className="md:col-span-2 border border-viet-mist/10 bg-viet-card p-8 grid grid-cols-1 md:grid-cols-2 gap-8">
              <div className="flex flex-col justify-between">
                <div>
                  <span className="text-xs font-mono-tech text-viet-teal tracking-widest uppercase block mb-3">Yield Terminal</span>
                  <h3 className="text-2xl font-serif-display text-white mb-4">Investment Calculator</h3>
                  <p className="text-xs text-viet-mist/70 font-light leading-relaxed mb-6">
                    Project your returns across different asset classes in Quy Nhon based on our current fund advisory models.
                  </p>
                </div>

                <div className="space-y-4 font-mono-tech text-xs">
                  <div>
                    <label className="text-viet-mist/40 uppercase block mb-2">Principal Investment ($ USD)</label>
                    <input 
                      type="range" 
                      min="50000" 
                      max="1000000" 
                      step="10000"
                      value={calculatorInputs.amount}
                      onChange={(e) => setCalculatorInputs({...calculatorInputs, amount: Number(e.target.value)})}
                      className="w-full accent-viet-teal bg-viet-black/50"
                    />
                    <div className="flex justify-between text-viet-teal font-bold mt-1">
                      <span>$50k</span>
                      <span className="text-sm">${calculatorInputs.amount.toLocaleString()}</span>
                      <span>$1M</span>
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label className="text-viet-mist/40 uppercase block mb-2">Term (Years)</label>
                      <select 
                        value={calculatorInputs.term}
                        onChange={(e) => setCalculatorInputs({...calculatorInputs, term: Number(e.target.value)})}
                        className="w-full bg-viet-black border border-viet-mist/20 p-2 text-white"
                      >
                        {[3, 5, 7, 10].map(y => (
                          <option key={y} value={y}>{y} Years</option>
                        ))}
                      </select>
                    </div>
                    <div>
                      <label className="text-viet-mist/40 uppercase block mb-2">Asset Type</label>
                      <select 
                        value={calculatorInputs.type}
                        onChange={(e) => setCalculatorInputs({...calculatorInputs, type: e.target.value})}
                        className="w-full bg-viet-black border border-viet-mist/20 p-2 text-white"
                      >
                        <option value="residential">Luxury Villa</option>
                        <option value="commercial">Commercial Hub</option>
                      </select>
                    </div>
                  </div>
                </div>
              </div>

              {/* Calculator Output Display */}
              <div className="bg-viet-black/40 border border-viet-mist/10 p-6 flex flex-col justify-between">
                <div className="space-y-4">
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Estimated ROI</span>
                    <span className="text-4xl font-serif-display text-viet-gold font-bold">{calculated.roi}%</span>
                  </div>
                  <div className="grid grid-cols-2 gap-4 border-t border-viet-mist/10 pt-4">
                    <div>
                      <span className="text-[9px] font-mono-tech text-viet-mist/40 uppercase block mb-0.5">Total Payout</span>
                      <span className="text-sm font-mono-tech text-white font-bold">${calculated.total}</span>
                    </div>
                    <div>
                      <span className="text-[9px] font-mono-tech text-viet-mist/40 uppercase block mb-0.5">Net Profit</span>
                      <span className="text-sm font-mono-tech text-viet-green font-bold">${calculated.profit}</span>
                    </div>
                  </div>
                </div>

                <div className="mt-6">
                  <p className="text-[10px] text-viet-mist/40 font-light mb-4 leading-relaxed">
                    *Projections are based on historical performance and regional growth targets. Actual results may vary.
                  </p>
                  <Button 
                    className="w-full bg-viet-teal text-viet-black hover:bg-viet-mist hover:text-viet-navy rounded-none font-mono-tech text-xs tracking-wider uppercase py-5"
                    onClick={() => handlePlaceholderClick("Request Advisory Session")}
                  >
                    Request advisory
                  </Button>
                </div>
              </div>
            </div>

          </div>
        </div>
      </section>

      {/* 4. RESEARCH SECTION — quiet navy base (same family as the page) with a
          faint dot-grid texture so it reads as its own "document / library" zone
          via depth rather than a stark color clash. */}
      <section id="research" className="research-texture py-24 text-viet-mist relative overflow-hidden border-y border-viet-mist/10">
        <div className="container relative z-10">

          <div className="max-w-3xl mb-16">
            <span className="text-xs font-mono-tech text-viet-teal tracking-[0.3em] uppercase block mb-3">Academic Rigor</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light leading-tight">
              Bridgewater-Grade <span className="italic text-viet-teal">Research &amp; Insights</span>
            </h2>
            <p className="text-sm text-viet-mist/60 font-light mt-4 leading-relaxed">
              We believe in deep macroeconomic analysis. Our research team produces exhaustive monthly reports on Vietnam’s economic landscape, urbanization trends, and Quy Nhon’s real estate cycles.
            </p>
          </div>

          {/* Research Carousel / Interactive List */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">

            {/* Research Selection Menu */}
            <div className="lg:col-span-1 space-y-4">
              {[
                { title: "Vietnam Coastal Macro Report 2026", date: "May 2026", cat: "Macroeconomics" },
                { title: "Quy Nhon Infrastructure Masterplan & Land Valuation", date: "April 2026", cat: "Urban Planning" },
                { title: "Post-Pandemic Tourism & Hospitality Yield Shifts", date: "March 2026", cat: "Market Analysis" }
              ].map((report, idx) => (
                <div
                  key={idx}
                  onClick={() => setActiveResearch(idx)}
                  className={`p-6 border border-l-2 cursor-pointer transition-all duration-300 ${
                    activeResearch === idx
                      ? "research-panel border-viet-mist/10 border-l-viet-teal shadow-lg shadow-viet-black/50"
                      : "bg-viet-card/40 border-viet-mist/10 border-l-transparent hover:bg-viet-card/70 hover:border-l-viet-mist/30"
                  }`}
                >
                  <span className="text-[10px] font-mono-tech text-viet-teal uppercase tracking-widest block mb-2">{report.cat}</span>
                  <h4 className="font-serif-display text-lg text-white font-bold mb-3">{report.title}</h4>
                  <div className="flex justify-between items-center font-mono-tech text-[10px] text-viet-mist/40">
                    <span>{report.date}</span>
                    <span className="flex items-center gap-1">Read <ChevronRight className="w-3 h-3" /></span>
                  </div>
                </div>
              ))}
            </div>

            {/* Research Preview Content Pane — darker, opaque reading field so the
                dot texture stops at the edge and the dim body text is easy to focus on.
                `.research-panel` adds a soft warm "low sun" from the top-left. */}
            <div className="research-panel lg:col-span-2 border border-viet-mist/10 shadow-2xl shadow-viet-black/60 p-8 sm:p-12 flex flex-col justify-between">
              <div>
                <div className="flex flex-wrap justify-between items-start gap-4 border-b border-viet-mist/10 pb-6 mb-8">
                  <div>
                    <span className="text-xs font-mono-tech text-viet-teal uppercase tracking-widest block mb-1">
                      {activeResearch === 0 ? "Macroeconomics" : activeResearch === 1 ? "Urban Planning" : "Market Analysis"}
                    </span>
                    <h3 className="text-2xl sm:text-3xl font-serif-display text-white font-bold">
                      {activeResearch === 0
                        ? "Vietnam Coastal Macro Report 2026"
                        : activeResearch === 1
                        ? "Quy Nhon Infrastructure Masterplan"
                        : "Post-Pandemic Hospitality Yield Shifts"}
                    </h3>
                  </div>
                  <div className="flex items-center gap-2 font-mono-tech text-xs text-viet-mist/50 bg-viet-mist/[0.04] px-3 py-1.5">
                    <FileText className="w-4 h-4" /> PDF 4.2 MB
                  </div>
                </div>

                <div className="prose prose-sm text-viet-mist/80 font-light max-w-none space-y-6 leading-relaxed">
                  {activeResearch === 0 ? (
                    <>
                      <p className="font-serif-display italic text-lg text-viet-mist/90">
                        "Vietnam’s coastal secondary cities are outperforming saturated primary markets, driven by domestic wealth expansion and direct FDI."
                      </p>
                      <p>
                        Our May 2026 analysis indicates a major shift in capital deployment. As Ho Chi Minh City and Hanoi face regulatory and land availability constraints, institutional real estate capital is rapidly relocating to coastal hubs with active infrastructure development.
                      </p>
                      <p>
                        Quy Nhon stands out due to its unique combination of deep-water port access, high-speed rail connectivity, and a local government aggressively pushing for technological and tourism transformation.
                      </p>
                    </>
                  ) : activeResearch === 1 ? (
                    <>
                      <p className="font-serif-display italic text-lg text-viet-mist/90">
                        "The expansion of Phu Cat Airport and the Nhon Hoi Economic Zone are creating unprecedented land valuation uplifts in Quy Nhon."
                      </p>
                      <p>
                        This report maps the correlation between infrastructure milestones and land pricing. With the airport expansion nearing completion and the new coastal highway fully operational, travel times have decreased by 40%, directly impacting luxury resort occupancies.
                      </p>
                      <p>
                        We analyze specific sub-districts poised for the highest capital appreciation over the next 36 months, providing actionable entry points for institutional portfolios.
                      </p>
                    </>
                  ) : (
                    <>
                      <p className="font-serif-display italic text-lg text-viet-mist/90">
                        "Yield structures in hospitality assets are shifting from volume-driven models to exclusive, low-density, high-rate private estates."
                      </p>
                      <p>
                        Post-pandemic high-net-worth travelers demand privacy, wellness, and architectural uniqueness. Our research shows that low-density luxury villas in Quy Nhon command a 35% premium in ADR (Average Daily Rate) compared to traditional luxury hotel suites.
                      </p>
                      <p>
                        We dissect the operating metrics of leading coastal estates to demonstrate how smart design directly drives superior cash-on-cash yields.
                      </p>
                    </>
                  )}
                </div>
              </div>

              <div className="mt-12 pt-6 border-t border-viet-mist/10 flex flex-col sm:flex-row justify-between items-center gap-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-viet-teal/15 border border-viet-teal/30 flex items-center justify-center font-serif-display font-bold text-viet-teal">
                    EV
                  </div>
                  <div>
                    <p className="text-xs font-bold text-white">EV Research Department</p>
                    <p className="text-[10px] font-mono-tech text-viet-mist/50">Lead Author: Dr. Nguyen An, Chief Economist</p>
                  </div>
                </div>
                <Button
                  className="bg-viet-teal text-viet-black hover:bg-viet-mist hover:text-viet-navy transition-all duration-300 rounded-none font-mono-tech text-xs tracking-wider uppercase py-5 px-6"
                  onClick={() => handlePlaceholderClick("Download Full Report")}
                >
                  Download Full Report <ArrowUpRight className="w-4 h-4 ml-2" />
                </Button>
              </div>
            </div>

          </div>
        </div>
      </section>

      {/* 5. TEAM SECTION (ported from good_team page — photo cards + boardroom).
          Back on navy base so it separates cleanly from the charcoal research band. */}
      <section id="team" className="py-24 relative border-t border-viet-mist/10 bg-viet-black">
        <div className="container">

          {/* Intro + boardroom image, split layout */}
          <div className="grid lg:grid-cols-12 gap-12 items-center mb-16">
            <div className="lg:col-span-6 space-y-4">
              <span className="text-xs font-mono-tech text-viet-teal tracking-[0.3em] uppercase block">Management &amp; Expertise</span>
              <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light">
                Led by <span className="italic text-viet-teal">Institutional Pioneers</span>
              </h2>
              <p className="text-sm text-viet-mist/60 font-light leading-relaxed max-w-xl">
                The EV Investment team combines international experience in investment, risk management, and real estate development. Our goal is to deliver maximum transparency and returns for our partners.
              </p>
            </div>
            <div className="lg:col-span-6">
              {/* Premium fund-office image */}
              <div className="relative aspect-[16/9] rounded-xl overflow-hidden border border-viet-mist/10 shadow-2xl">
                <img
                  src={ASSETS.office_interior}
                  alt="EV Investment Boardroom"
                  className="w-full h-full object-cover opacity-70"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-viet-black/80 via-transparent to-transparent" />
                <div className="absolute bottom-4 left-4 right-4 flex justify-between items-end">
                  <div className="space-y-1">
                    <span className="text-[10px] font-mono-tech text-viet-teal uppercase tracking-wider">Head Office</span>
                    <h4 className="text-sm font-bold text-white">EV Boardroom • Quy Nhon</h4>
                  </div>
                  <span className="text-[10px] text-viet-mist/40 font-mono-tech">Q1 2026</span>
                </div>
              </div>
            </div>
          </div>

          {/* Team cards — Stronghold-style photo grid */}
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">

            {/* Member 1 */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-viet-mist/10 bg-viet-card">
                <img
                  src={ASSETS.team_member_1}
                  alt="Minh Nguyen"
                  className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-viet-black/85 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-6">
                  <p className="text-xs text-viet-mist/80 font-light leading-relaxed">
                    15+ years managing sovereign funds across Vietnam and Singapore. Former Investment Director at VinaCapital.
                  </p>
                </div>
              </div>
              <div>
                <h4 className="font-serif-display font-bold text-white text-base">Minh Nguyen</h4>
                <p className="text-xs text-viet-teal font-mono-tech mt-1">Managing Partner, Co-founder</p>
              </div>
            </div>

            {/* Member 2 */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-viet-mist/10 bg-viet-card">
                <img
                  src={ASSETS.team_member_2}
                  alt="Anh Pham"
                  className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-viet-black/85 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-6">
                  <p className="text-xs text-viet-mist/80 font-light leading-relaxed">
                    Specializes in risk assessment and financial modelling for coastal development. Previously at CBRE Vietnam.
                  </p>
                </div>
              </div>
              <div>
                <h4 className="font-serif-display font-bold text-white text-base">Anh Pham</h4>
                <p className="text-xs text-viet-teal font-mono-tech mt-1">Director of Research &amp; Risk</p>
              </div>
            </div>

            {/* Open position card */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-viet-mist/10 bg-viet-card flex items-center justify-center p-6 text-center">
                <div className="space-y-4">
                  <div className="w-12 h-12 rounded-full bg-viet-mist/5 border border-viet-mist/10 flex items-center justify-center mx-auto text-viet-teal">
                    <Users className="w-6 h-6" />
                  </div>
                  <div>
                    <h5 className="font-serif-display font-bold text-white text-sm">Join Us</h5>
                    <p className="text-xs text-viet-mist/50 mt-2 font-light">
                      We are always looking for talented analysts and asset managers in Quy Nhon.
                    </p>
                  </div>
                  <Button
                    onClick={() => handlePlaceholderClick("Careers")}
                    variant="outline"
                    className="border-viet-mist/15 text-viet-mist/80 hover:border-viet-teal hover:text-viet-teal text-xs py-1 h-auto bg-transparent"
                  >
                    Careers
                  </Button>
                </div>
              </div>
              <div>
                <h4 className="font-serif-display font-bold text-viet-mist/40 text-base">Open Position</h4>
                <p className="text-xs text-viet-mist/40 font-mono-tech mt-1">Investment Analyst</p>
              </div>
            </div>

            {/* LP network card */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-viet-mist/10 bg-viet-card flex items-center justify-center p-6 text-center">
                <div className="space-y-4">
                  <div className="w-12 h-12 rounded-full bg-viet-mist/5 border border-viet-mist/10 flex items-center justify-center mx-auto text-viet-gold">
                    <Globe className="w-6 h-6" />
                  </div>
                  <div>
                    <h5 className="font-serif-display font-bold text-white text-sm">LP Partner Network</h5>
                    <p className="text-xs text-viet-mist/50 mt-2 font-light">
                      Over 40 institutional investors across 12 countries trust us with their capital.
                    </p>
                  </div>
                  <Button
                    onClick={() => handlePlaceholderClick("IR Contacts")}
                    variant="outline"
                    className="border-viet-mist/15 text-viet-mist/80 hover:border-viet-teal hover:text-viet-teal text-xs py-1 h-auto bg-transparent"
                  >
                    IR Contacts
                  </Button>
                </div>
              </div>
              <div>
                <h4 className="font-serif-display font-bold text-viet-mist/40 text-base">Investor Relations</h4>
                <p className="text-xs text-viet-mist/40 font-mono-tech mt-1">Investor Relations (IR)</p>
              </div>
            </div>

          </div>

        </div>
      </section>

      {/* 6. FOOTER (Minimalist, structured) */}
      <footer className="bg-viet-black border-t border-viet-mist/10 py-16">
        <div className="container">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-12 mb-12">
            
            <div className="md:col-span-2">
              <div className="flex items-center gap-3 mb-6">
                <svg className="w-8 h-8 text-white" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M20 50 L50 35 L80 50" stroke="white" strokeWidth="2" strokeLinecap="round" />
                  <rect x="38" y="38" width="10" height="20" fill="white" />
                  <rect x="52" y="30" width="12" height="28" fill="white" />
                  <rect x="68" y="42" width="8" height="16" fill="white" />
                  <path d="M15 55 L85 55" stroke="white" strokeWidth="2" />
                </svg>
                <div className="flex flex-col">
                  <span className="font-serif-display font-bold text-base tracking-wider text-white">EV INVESTMENT</span>
                  <span className="text-[8px] font-mono-tech tracking-[0.3em] text-viet-teal uppercase">Quy Nhon Fund</span>
                </div>
              </div>
              <p className="text-xs text-viet-mist/50 font-light max-w-sm leading-relaxed mb-6">
                EV Investment is a registered real estate advisory and investment management fund specializing in premium coastal developments in Quy Nhon, Binh Dinh province, Vietnam.
              </p>
              <div className="flex gap-4 text-xs font-mono-tech text-viet-teal">
                <a href="#hero" className="hover:underline">Privacy Policy</a>
                <span className="text-viet-mist/20">|</span>
                <a href="#hero" className="hover:underline">Terms of Service</a>
              </div>
            </div>

            <div>
              <h4 className="font-mono-tech text-xs text-white uppercase tracking-widest mb-6">Offices</h4>
              <ul className="space-y-4 text-xs text-viet-mist/60 font-light leading-relaxed">
                <li>
                  <strong className="text-white block font-mono-tech text-[10px] uppercase tracking-wider mb-1">Quy Nhon Head Office</strong>
                  102 An Duong Vuong St, Nguyen Van Cu Ward, Quy Nhon City, Vietnam
                </li>
                <li>
                  <strong className="text-white block font-mono-tech text-[10px] uppercase tracking-wider mb-1">Ho Chi Minh Representative</strong>
                  Deutsches Haus, 33 Le Duan Blvd, District 1, Ho Chi Minh City, Vietnam
                </li>
              </ul>
            </div>

            <div>
              <h4 className="font-mono-tech text-xs text-white uppercase tracking-widest mb-6">Newsletter</h4>
              <p className="text-xs text-viet-mist/60 font-light leading-relaxed mb-4">
                Subscribe to our Bridgewater-grade monthly coastal macro reports.
              </p>
              <div className="flex border border-viet-mist/20">
                <input 
                  type="email" 
                  placeholder="Institutional Email" 
                  className="bg-transparent text-xs p-3 w-full focus:outline-none text-white"
                />
                <button
                  className="bg-viet-teal text-viet-black px-4 font-mono-tech text-xs uppercase font-bold hover:bg-viet-mist transition-colors"
                  onClick={() => handlePlaceholderClick("Newsletter Subscription")}
                >
                  Join
                </button>
              </div>
            </div>

          </div>

          <div className="border-t border-viet-mist/10 pt-8 flex flex-col sm:flex-row justify-between items-center gap-4 text-[10px] font-mono-tech text-viet-mist/40">
            <p>© 2026 EV Investment. All rights reserved.</p>
            <p>Designed for Bullmart Investment Fund.</p>
          </div>
        </div>
      </footer>

    </div>
  );
}
