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
  FileText
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";

// Ссылки на сгенерированные высококачественные изображения
const ASSETS = {
  quynhon_future: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/quynhon_future-ExoshVjhhPWYbYR4Zf3xJn.webp",
  luxury_villa: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/luxury_villa-64wseo7dGJUQNbg7HMSNPo.webp",
  abstract_investment: "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/SPbgMPRFEXcrCSr7Bo27uM/abstract_investment-eYjGPMMXA3Hkv5fdUCnFfs.webp"
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
      }`}>
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
              <span className="text-[9px] font-mono-tech tracking-[0.3em] text-viet-gold uppercase">Quy Nhon Fund</span>
            </div>
          </div>

          <nav className="hidden md:flex items-center gap-8 font-mono-tech text-xs tracking-widest uppercase">
            <a href="#hero" className="hover:text-viet-gold transition-colors">Home</a>
            <a href="#portfolio" className="hover:text-viet-gold transition-colors">Portfolio</a>
            <a href="#calculator" className="hover:text-viet-gold transition-colors">Terminal</a>
            <a href="#research" className="hover:text-viet-gold transition-colors">Research</a>
            <a href="#team" className="hover:text-viet-gold transition-colors">Team</a>
          </nav>

          <div className="flex items-center gap-4">
            <Button 
              variant="outline" 
              className="hidden sm:inline-flex font-mono-tech text-xs tracking-wider border-viet-gold text-viet-gold hover:bg-viet-gold hover:text-viet-black transition-all duration-300"
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
            backgroundImage: `linear-gradient(to bottom, rgba(23, 23, 23, 0.8), rgba(23, 23, 23, 0.95)), url(${ASSETS.quynhon_future})`,
            backgroundSize: "cover",
            backgroundPosition: "center"
          }}
        />

        {/* Floating Abstract Financial Grid Overlay */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#8080800a_1px,transparent_1px),linear-gradient(to_bottom,#8080800a_1px,transparent_1px)] bg-[size:14px_24px] pointer-events-none" />

        <div className="container relative z-10 text-center flex flex-col items-center justify-center h-full max-w-4xl px-4">
          
          {/* Tagline / Subtitle */}
          <div className="inline-flex items-center gap-2 px-3 py-1.5 border border-viet-gold/30 bg-viet-gold/5 text-viet-gold font-mono-tech text-[10px] tracking-[0.25em] uppercase mb-8 animate-pulse">
            <Compass className="w-3.5 h-3.5" /> Premium Coastal Real Estate Fund
          </div>

          {/* Integrated Logo & Offering (Stronghold Metaphor) */}
          <div 
            className="transition-all duration-700 ease-out"
            style={{ 
              transform: `scale(${Math.max(0.8, 1 - (zoomLevel - 1) * 0.15)})`,
              opacity: Math.max(0.1, 1 - (zoomLevel - 1) * 0.5)
            }}
          >
            <h1 className="text-4xl sm:text-6xl md:text-8xl font-serif-display font-light text-white leading-tight mb-6">
              Invest in <span className="italic text-viet-gold">Quy Nhon</span><br />
              Through Institutional Vision.
            </h1>
            
            <p className="text-sm sm:text-base md:text-lg text-viet-mist/70 font-light max-w-2xl mx-auto mb-12 leading-relaxed">
              EV Investment bridges the gap between premium coastal real estate development and sophisticated investors. Experience high-yield real estate assets in Vietnam’s fastest-growing coastal hub.
            </p>
          </div>

          {/* Interactive Action / Zoom Indicator */}
          <div className="flex flex-col items-center gap-4">
            <Button 
              className="bg-viet-gold text-viet-black hover:bg-white hover:scale-105 active:scale-95 transition-all duration-300 font-mono-tech text-xs tracking-widest uppercase px-8 py-6 rounded-none"
              onClick={() => {
                const portfolioSec = document.getElementById("portfolio");
                portfolioSec?.scrollIntoView({ behavior: "smooth" });
              }}
            >
              Explore Assets <ArrowRight className="w-4 h-4 ml-2" />
            </Button>
            
            <div className="mt-8 flex flex-col items-center gap-1.5">
              <span className="text-[9px] font-mono-tech tracking-[0.3em] text-viet-mist/40 uppercase">Scroll to zoom in & discover</span>
              <div className="w-1 h-12 bg-gradient-to-b from-viet-gold to-transparent rounded-full animate-bounce" />
            </div>
          </div>
        </div>

        {/* Bottom stats ribbon */}
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
              <p className="text-2xl sm:text-3xl font-serif-display text-viet-pink font-bold">100%</p>
            </div>
          </div>
        </div>
      </section>

      {/* 3. PORTFOLIO SECTION (Bento Grid, DWF Labs & Binance Inspired) */}
      <section id="portfolio" className="py-24 relative border-t border-viet-mist/10">
        <div className="container">
          
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-16">
            <div>
              <span className="text-xs font-mono-tech text-viet-gold tracking-[0.3em] uppercase block mb-3">Investment Scope</span>
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
                    ? "bg-viet-gold text-viet-black font-bold" 
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
                style={{ backgroundImage: `linear-gradient(to top, rgba(23, 23, 23, 0.95) 10%, rgba(23, 23, 23, 0.2)), url(${ASSETS.luxury_villa})` }}
              />
              <div className="absolute top-4 right-4 bg-viet-gold text-viet-black font-mono-tech text-[10px] tracking-widest uppercase px-3 py-1.5 font-bold">
                Featured Deal
              </div>
              <div className="relative z-10 p-8">
                <div className="flex items-center gap-2 text-viet-gold font-mono-tech text-xs mb-3">
                  <MapPin className="w-3.5 h-3.5" /> Nhon Ly Beach, Quy Nhon
                </div>
                <h3 className="text-2xl sm:text-3xl font-serif-display text-white mb-4">The Horizon Premium Villas</h3>
                <p className="text-sm text-viet-mist/70 font-light max-w-xl mb-6">
                  Exclusive ultra-luxury oceanfront villas with private pools, nestled between pristine limestone cliffs and crystal-clear turquoise waters.
                </p>
                <div className="grid grid-cols-3 gap-4 border-t border-viet-mist/10 pt-6 max-w-md">
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Target Yield</span>
                    <span className="text-lg font-serif-display text-viet-teal font-bold">12.5% p.a.</span>
                  </div>
                  <div>
                    <span className="text-[10px] font-mono-tech text-viet-mist/40 uppercase block mb-1">Appreciation</span>
                    <span className="text-lg font-serif-display text-viet-gold font-bold">18% YoY</span>
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
                style={{ backgroundImage: `linear-gradient(to top, rgba(23, 23, 23, 0.95) 20%, rgba(23, 23, 23, 0.4)), url(${ASSETS.quynhon_future})` }}
              />
              <div className="relative z-10 p-8">
                <div className="flex items-center gap-2 text-viet-gold font-mono-tech text-xs mb-3">
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
                    className="p-0 text-viet-gold hover:text-white hover:bg-transparent font-mono-tech text-xs tracking-wider"
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
                  <span className="text-viet-gold font-bold">+28% YoY</span>
                </li>
                <li className="flex justify-between">
                  <span className="text-viet-mist/40">FDI Inflow (2025):</span>
                  <span className="text-viet-teal font-bold">$420M</span>
                </li>
              </ul>
            </div>

            {/* Asset 4 (Interactive Yield/Term Calculator - Binance Fintech Style) */}
            <div id="calculator" className="md:col-span-2 border border-viet-mist/10 bg-viet-card p-8 grid grid-cols-1 md:grid-cols-2 gap-8">
              <div className="flex flex-col justify-between">
                <div>
                  <span className="text-xs font-mono-tech text-viet-gold tracking-widest uppercase block mb-3">Yield Terminal</span>
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
                      className="w-full accent-viet-gold bg-viet-black/50"
                    />
                    <div className="flex justify-between text-viet-gold font-bold mt-1">
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
                      <span className="text-sm font-mono-tech text-viet-teal font-bold">${calculated.profit}</span>
                    </div>
                  </div>
                </div>

                <div className="mt-6">
                  <p className="text-[10px] text-viet-mist/40 font-light mb-4 leading-relaxed">
                    *Projections are based on historical performance and regional growth targets. Actual results may vary.
                  </p>
                  <Button 
                    className="w-full bg-viet-teal text-white hover:bg-white hover:text-viet-black rounded-none font-mono-tech text-xs tracking-wider uppercase py-5"
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

      {/* 4. RESEARCH SECTION (Bridgewater Inspired - Contrast Academic Light Theme) */}
      <section id="research" className="py-24 bg-viet-mist text-viet-black light-academic relative overflow-hidden">
        {/* Abstract watermark representing Bridgewater academic rigor */}
        <div className="absolute right-0 top-0 opacity-[0.03] text-[20rem] font-serif-display select-none pointer-events-none transform translate-x-1/4 -translate-y-1/4">
          R
        </div>

        <div className="container relative z-10">
          
          <div className="max-w-3xl mb-16">
            <span className="text-xs font-mono-tech text-viet-teal tracking-[0.3em] uppercase block mb-3">Academic Rigor</span>
            <h2 className="text-3xl sm:text-5xl font-serif-display text-viet-black font-light leading-tight">
              Bridgewater-Grade <span className="italic text-viet-green">Research & Insights</span>
            </h2>
            <p className="text-sm text-viet-black/60 font-light mt-4 leading-relaxed">
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
                  className={`p-6 border cursor-pointer transition-all duration-300 ${
                    activeResearch === idx 
                      ? "bg-white border-viet-teal shadow-md" 
                      : "border-viet-black/10 hover:border-viet-black/30 bg-transparent"
                  }`}
                >
                  <span className="text-[10px] font-mono-tech text-viet-teal uppercase tracking-widest block mb-2">{report.cat}</span>
                  <h4 className="font-serif-display text-lg text-viet-black font-bold mb-3">{report.title}</h4>
                  <div className="flex justify-between items-center font-mono-tech text-[10px] text-viet-black/40">
                    <span>{report.date}</span>
                    <span className="flex items-center gap-1">Read <ChevronRight className="w-3 h-3" /></span>
                  </div>
                </div>
              ))}
            </div>

            {/* Research Preview Content Pane */}
            <div className="lg:col-span-2 bg-white border border-viet-black/10 p-8 sm:p-12 flex flex-col justify-between shadow-lg">
              <div>
                <div className="flex flex-wrap justify-between items-start gap-4 border-b border-viet-black/10 pb-6 mb-8">
                  <div>
                    <span className="text-xs font-mono-tech text-viet-teal uppercase tracking-widest block mb-1">
                      {activeResearch === 0 ? "Macroeconomics" : activeResearch === 1 ? "Urban Planning" : "Market Analysis"}
                    </span>
                    <h3 className="text-2xl sm:text-3xl font-serif-display text-viet-black font-bold">
                      {activeResearch === 0 
                        ? "Vietnam Coastal Macro Report 2026" 
                        : activeResearch === 1 
                        ? "Quy Nhon Infrastructure Masterplan" 
                        : "Post-Pandemic Hospitality Yield Shifts"}
                    </h3>
                  </div>
                  <div className="flex items-center gap-2 font-mono-tech text-xs text-viet-black/50 bg-viet-mist/30 px-3 py-1.5 border border-viet-black/5">
                    <FileText className="w-4 h-4" /> PDF 4.2 MB
                  </div>
                </div>

                <div className="prose prose-sm text-viet-black/70 font-light max-w-none space-y-6 leading-relaxed">
                  {activeResearch === 0 ? (
                    <>
                      <p className="font-serif-display italic text-lg text-viet-black/80">
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
                      <p className="font-serif-display italic text-lg text-viet-black/80">
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
                      <p className="font-serif-display italic text-lg text-viet-black/80">
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

              <div className="mt-12 pt-6 border-t border-viet-black/10 flex flex-col sm:flex-row justify-between items-center gap-4">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-viet-black/10 flex items-center justify-center font-serif-display font-bold text-viet-black">
                    EV
                  </div>
                  <div>
                    <p className="text-xs font-bold text-viet-black">EV Research Department</p>
                    <p className="text-[10px] font-mono-tech text-viet-black/50">Lead Author: Dr. Nguyen An, Chief Economist</p>
                  </div>
                </div>
                <Button 
                  className="bg-viet-black text-white hover:bg-viet-teal transition-all duration-300 rounded-none font-mono-tech text-xs tracking-wider uppercase py-5 px-6"
                  onClick={() => handlePlaceholderClick("Download Full Report")}
                >
                  Download Full Report <ArrowUpRight className="w-4 h-4 ml-2" />
                </Button>
              </div>
            </div>

          </div>
        </div>
      </section>

      {/* 5. TEAM SECTION (Stronghold & SWF Institute Inspired) */}
      <section id="team" className="py-24 relative border-t border-viet-mist/10">
        <div className="container">
          
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-16">
            <div>
              <span className="text-xs font-mono-tech text-viet-gold tracking-[0.3em] uppercase block mb-3">Founding Vision</span>
              <h2 className="text-3xl sm:text-5xl font-serif-display text-white font-light">
                Led by <span className="italic text-viet-pink">Institutional Pioneers</span>
              </h2>
            </div>
            <p className="text-sm text-viet-mist/60 font-light max-w-md mt-4 md:mt-0 leading-relaxed">
              Our leadership team combines international investment banking discipline with deep on-the-ground Vietnamese real estate development expertise.
            </p>
          </div>

          {/* Team Grid */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {[
              {
                name: "Minh Hoang",
                role: "Managing Partner",
                bio: "Former VP of Real Estate Advisory at a leading Singaporean investment bank. 15+ years of capital markets experience across Southeast Asia.",
                init: "MH"
              },
              {
                name: "Dr. Nguyen An",
                role: "Chief Economist & Research",
                bio: "PhD in Urban Economics from London School of Economics. Leads our Bridgewater-grade research and macroeconomic forecasting models.",
                init: "NA"
              },
              {
                name: "Elena Vostrikova",
                role: "Head of International Capital",
                bio: "Specializes in structuring cross-border real estate investment vehicles for European and Middle Eastern family offices.",
                init: "EV"
              }
            ].map((member, idx) => (
              <div key={idx} className="border border-viet-mist/10 bg-viet-black/30 p-8 flex flex-col justify-between group hover:border-viet-gold transition-all duration-500">
                <div>
                  {/* Premium Abstract Portrait Placeholder */}
                  <div className="w-20 h-20 bg-viet-mist/10 border border-viet-mist/20 flex items-center justify-center font-serif-display text-2xl text-viet-gold mb-8 group-hover:bg-viet-gold group-hover:text-viet-black transition-all duration-500">
                    {member.init}
                  </div>
                  <h3 className="text-xl font-serif-display text-white font-bold mb-1">{member.name}</h3>
                  <p className="text-xs font-mono-tech text-viet-gold uppercase tracking-widest mb-6">{member.role}</p>
                  <p className="text-sm text-viet-mist/70 font-light leading-relaxed mb-8">
                    {member.bio}
                  </p>
                </div>
                
                <div className="border-t border-viet-mist/10 pt-6 flex justify-between items-center">
                  <span className="text-[10px] font-mono-tech text-viet-mist/40">Verified Advisory</span>
                  <div className="flex gap-3">
                    <button 
                      className="text-viet-mist/60 hover:text-viet-gold transition-colors"
                      onClick={() => handlePlaceholderClick(`Contact ${member.name} via LinkedIn`)}
                    >
                      <Globe className="w-4 h-4" />
                    </button>
                    <button 
                      className="text-viet-mist/60 hover:text-viet-gold transition-colors"
                      onClick={() => handlePlaceholderClick(`Contact ${member.name} via Email`)}
                    >
                      <Briefcase className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>

          {/* Institutional Trust Badges (SWF Institute Style) */}
          <div className="mt-24 pt-12 border-t border-viet-mist/10 grid grid-cols-2 md:grid-cols-4 gap-8 items-center justify-items-center opacity-40 grayscale hover:grayscale-0 transition-all duration-500">
            <div className="flex items-center gap-2 font-serif-display text-lg text-white font-bold">
              <ShieldCheck className="w-5 h-5 text-viet-gold" /> VIETNAM SEC COMPLIANT
            </div>
            <div className="flex items-center gap-2 font-serif-display text-lg text-white font-bold">
              <ShieldCheck className="w-5 h-5 text-viet-teal" /> APREA MEMBER
            </div>
            <div className="flex items-center gap-2 font-serif-display text-lg text-white font-bold">
              <ShieldCheck className="w-5 h-5 text-viet-pink" /> ISO 9001 CERTIFIED
            </div>
            <div className="flex items-center gap-2 font-serif-display text-lg text-white font-bold">
              <ShieldCheck className="w-5 h-5 text-viet-gold" /> ESG COMPLIANT FUND
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
                  <span className="text-[8px] font-mono-tech tracking-[0.3em] text-viet-gold uppercase">Quy Nhon Fund</span>
                </div>
              </div>
              <p className="text-xs text-viet-mist/50 font-light max-w-sm leading-relaxed mb-6">
                EV Investment is a registered real estate advisory and investment management fund specializing in premium coastal developments in Quy Nhon, Binh Dinh province, Vietnam.
              </p>
              <div className="flex gap-4 text-xs font-mono-tech text-viet-gold">
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
                  className="bg-viet-gold text-viet-black px-4 font-mono-tech text-xs uppercase font-bold hover:bg-white transition-colors"
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
