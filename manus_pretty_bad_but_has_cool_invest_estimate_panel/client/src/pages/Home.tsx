import React, { useState, useEffect, useRef } from "react";
import { motion, useScroll, useTransform, AnimatePresence } from "framer-motion";
import { 
  TrendingUp, 
  ArrowUpRight, 
  ChevronRight, 
  FileText, 
  Download, 
  MapPin, 
  Users, 
  Shield, 
  LineChart, 
  Globe, 
  Layers, 
  Search, 
  Filter, 
  Layers3,
  Sliders,
  ChevronDown
} from "lucide-react";
import Logo from "@/components/Logo";

// Generated assets
const HERO_BG = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/AiD4jWiKvxpfHDiPT7YcwL/hero_bg-hzXrbi4y8n9uCi9Ds7edCw.webp";
const RESEARCH_COVER = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/AiD4jWiKvxpfHDiPT7YcwL/research_cover-A8aoCQSHSdZfekSh8RnFrg.webp";
const TEAM_PORTRAIT = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/AiD4jWiKvxpfHDiPT7YcwL/team_placeholder-NzYVFhVTUAf8tQZhyDHDZv.webp";

export default function Home() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [activeTab, setActiveTab] = useState<"all" | "residential" | "hospitality" | "land">("all");
  const [selectedResearch, setSelectedResearch] = useState<number | null>(null);
  const [calculatorAmount, setCalculatorAmount] = useState<number>(100000);
  const [calculatorPeriod, setCalculatorPeriod] = useState<number>(5);

  // Framer Motion scroll hook for Hero Zoom effect (inspired by Stronghold Fund)
  const { scrollYProgress } = useScroll({
    target: containerRef,
    offset: ["start start", "end end"]
  });

  // Zoom and opacity animations for hero page
  const scale = useTransform(scrollYProgress, [0, 0.25], [1, 1.4]);
  const bgOpacity = useTransform(scrollYProgress, [0, 0.25], [0.4, 0.15]);
  const heroContentY = useTransform(scrollYProgress, [0, 0.2], [0, -80]);
  const heroContentOpacity = useTransform(scrollYProgress, [0, 0.15], [1, 0]);

  // Quy Nhon real estate project data
  const projects = [
    {
      id: 1,
      title: "The Horizon Heights",
      category: "residential",
      location: "Quy Nhon Bay",
      irr: "18.4%",
      multiplier: "1.65x",
      status: "Under Construction",
      minInvestment: "$50,000",
      description: "Premium residential tower with panoramic views of the Quy Nhon ocean skyline."
    },
    {
      id: 2,
      title: "Nhơn Hội Coastal Resort",
      category: "hospitality",
      location: "Nhon Hoi Economic Zone",
      irr: "22.1%",
      multiplier: "1.90x",
      status: "Permitting",
      minInvestment: "$100,000",
      description: "Eco-luxury beachfront resort catering to high-end international tourism."
    },
    {
      id: 3,
      title: "Phu Cat Logistic Hub",
      category: "land",
      location: "Phu Cat",
      irr: "15.8%",
      multiplier: "1.45x",
      status: "Acquisition Phase",
      minInvestment: "$250,000",
      description: "Strategic industrial land parcels adjacent to Phu Cat Airport."
    },
    {
      id: 4,
      title: "Quy Nhon Marina Villas",
      category: "residential",
      location: "Thi Nai Lagoon",
      irr: "19.5%",
      multiplier: "1.75x",
      status: "Pre-sale",
      minInvestment: "$75,000",
      description: "Exclusive waterfront residential community with private yacht berths."
    }
  ];

  // Research reports data (inspired by Bridgewater)
  const researchReports = [
    {
      id: 1,
      title: "Quy Nhon Real Estate Market Outlook Q2 2026",
      date: "May 2026",
      author: "EV Research Team",
      summary: "An in-depth analysis of supply-demand dynamics in Binh Dinh province, highlighting the infrastructure acceleration of Nhon Hoi.",
      readTime: "12 min read",
      category: "Macro Outlook"
    },
    {
      id: 2,
      title: "Coastal Tourism & Hospitality Yield Analysis",
      date: "April 2026",
      author: "Investment Strategy Group",
      summary: "Comparative yield analysis of Quy Nhon versus Da Nang and Nha Trang, exploring underserved premium segments.",
      readTime: "18 min read",
      category: "Sector Report"
    },
    {
      id: 3,
      title: "Binh Dinh Infrastructure Masterplan & Land Values",
      date: "March 2026",
      author: "Urban Planning Analytics",
      summary: "How the expansion of Phu Cat Airport and Highway 19 will reshape logistics and industrial land valuation over the next decade.",
      readTime: "15 min read",
      category: "Infrastructure"
    }
  ];

  // Team data (inspired by Stronghold Fund & SWF)
  const team = [
    {
      name: "Alex Nguyen",
      role: "Managing Partner",
      bio: "15+ years in Southeast Asia real estate private equity. Former Director at VinaCapital.",
      focus: "Fund Strategy & Investor Relations"
    },
    {
      name: "Minh Tran",
      role: "Head of Acquisitions",
      bio: "Local Quy Nhon development expert with deep regulatory network and transaction history.",
      focus: "Deal Sourcing & Permitting"
    },
    {
      name: "Elena Rostova",
      role: "Chief Investment Officer",
      bio: "Ex-Wall Street analyst. Specializes in structured real estate finance and risk underwriting.",
      focus: "Underwriting & Asset Management"
    }
  ];

  const filteredProjects = activeTab === "all" ? projects : projects.filter(p => p.category === activeTab);

  // Financial Calculator logic (inspired by Binance utility)
  const estimatedReturn = calculatorAmount * Math.pow(1 + 0.184, calculatorPeriod);
  const estimatedProfit = estimatedReturn - calculatorAmount;

  return (
    <div ref={containerRef} className="bg-[#111418] text-[#DAE2EF] min-h-screen relative overflow-x-hidden selection:bg-[#8EB8FE] selection:text-[#090B0F]">
      
      {/* Dynamic Header */}
      <header className="fixed top-0 left-0 right-0 z-50 bg-[#111418]/80 backdrop-blur-md border-b border-[#3D434C]/50 px-6 py-4 transition-all duration-300">
        <div className="max-w-7xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Logo className="h-8" />
          </div>
          
          <nav className="hidden md:flex items-center gap-8 text-sm font-medium tracking-wider text-[#979FAC]">
            <a href="#hero" className="hover:text-[#8EB8FE] transition-colors">OVERVIEW</a>
            <a href="#portfolio" className="hover:text-[#8EB8FE] transition-colors">PORTFOLIO</a>
            <a href="#research" className="hover:text-[#8EB8FE] transition-colors">RESEARCH</a>
            <a href="#calculator" className="hover:text-[#8EB8FE] transition-colors">METRICS</a>
            <a href="#team" className="hover:text-[#8EB8FE] transition-colors">TEAM</a>
          </nav>

          <div className="flex items-center gap-4">
            <button className="hidden sm:inline-flex px-4 py-2 border border-[#3D434C] text-xs font-semibold tracking-widest text-[#E6E1D3] hover:bg-[#8EB8FE] hover:text-[#090B0F] hover:border-[#8EB8FE] transition-all duration-300">
              INVESTOR PORTAL
            </button>
            <button className="px-4 py-2 bg-[#8EB8FE] text-[#090B0F] text-xs font-semibold tracking-widest hover:bg-white transition-all duration-300">
              CONTACT FUND
            </button>
          </div>
        </div>
      </header>

      {/* SECTION 1: HERO ZOOM PAGE (Stronghold Fund Metaphor) */}
      <section id="hero" className="relative h-screen flex items-center justify-center overflow-hidden">
        {/* Cinematic Zoom Background */}
        <motion.div 
          style={{ scale, opacity: bgOpacity }}
          className="absolute inset-0 z-0 transition-transform duration-100 ease-out"
        >
          <div className="absolute inset-0 bg-gradient-to-b from-[#090B0F] via-transparent to-[#111418] z-10" />
          <img 
            src={HERO_BG} 
            alt="Quy Nhon Skyline" 
            className="w-full h-full object-cover filter brightness-75 contrast-110"
          />
        </motion.div>

        {/* Grid Overlay for Architectural feel */}
        <div className="absolute inset-0 grid-overlay z-10 opacity-30 pointer-events-none" />

        {/* Integrated Logo & Offering (Stronghold style) */}
        <motion.div 
          style={{ y: heroContentY, opacity: heroContentOpacity }}
          className="relative z-20 text-center px-4 max-w-4xl mx-auto flex flex-col items-center"
        >
          <div className="mb-6 opacity-80">
            <Logo className="h-16" />
          </div>
          
          <span className="text-[#8EB8FE] text-xs font-bold tracking-[0.3em] mb-4 block">
            VIETNAM REAL ESTATE PRIVATE EQUITY
          </span>
          
          <h1 className="text-4xl md:text-7xl font-light tracking-tight text-[#E6E1D3] mb-6 leading-tight">
            Institutional Real Estate <br />
            <span className="font-bold text-white">Investments in Quy Nhon</span>
          </h1>
          
          <p className="text-[#979FAC] text-lg md:text-xl font-light max-w-2xl mb-8 leading-relaxed">
            Unlocking high-yield coastal developments through meticulous research, local structural advantages, and global risk management.
          </p>

          <div className="flex flex-col sm:flex-row gap-4">
            <a 
              href="#portfolio" 
              className="px-8 py-4 bg-[#8EB8FE] text-[#090B0F] font-bold text-xs tracking-widest hover:bg-[#E6E1D3] transition-all duration-300 flex items-center gap-2"
            >
              EXPLORE OPPORTUNITIES <ArrowUpRight className="h-4 w-4" />
            </a>
            <a 
              href="#research" 
              className="px-8 py-4 border border-[#3D434C] text-[#E6E1D3] font-bold text-xs tracking-widest hover:bg-white/5 transition-all duration-300"
            >
              VIEW RESEARCH & INSIGHTS
            </a>
          </div>
        </motion.div>

        {/* Scroll Indicator */}
        <div className="absolute bottom-8 left-1/2 -translate-x-1/2 z-20 flex flex-col items-center gap-2 opacity-50">
          <span className="text-[10px] tracking-[0.2em] font-medium">SCROLL TO ZOOM</span>
          <ChevronDown className="h-4 w-4 animate-bounce text-[#8EB8FE]" />
        </div>
      </section>

      {/* SECTION 2: THE PORTFOLIO & SECTOR FOCUS */}
      <section id="portfolio" className="py-24 px-6 relative z-20 bg-[#111418] border-t border-[#3D434C]/50">
        <div className="max-w-7xl mx-auto">
          
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-16 gap-6">
            <div>
              <span className="text-[#8EB8FE] text-xs font-bold tracking-[0.2em] mb-2 block">
                STRATEGIC ALLOCATION
              </span>
              <h2 className="text-3xl md:text-5xl font-bold text-white tracking-tight">
                Quy Nhon Asset Class Focus
              </h2>
            </div>
            
            {/* Filter Tabs (Binance style) */}
            <div className="flex flex-wrap gap-2 border-b border-[#3D434C] pb-2 md:pb-0 md:border-none">
              {(["all", "residential", "hospitality", "land"] as const).map((tab) => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-4 py-2 text-xs font-semibold tracking-widest uppercase transition-all duration-300 ${
                    activeTab === tab 
                      ? "text-[#8EB8FE] border-b-2 border-[#8EB8FE] bg-[#8EB8FE]/5" 
                      : "text-[#979FAC] hover:text-white"
                  }`}
                >
                  {tab}
                </button>
              ))}
            </div>
          </div>

          {/* Grid Layout (Asymmetric modern structure) */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <AnimatePresence mode="popLayout">
              {filteredProjects.map((project, index) => (
                <motion.div
                  layout
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  transition={{ duration: 0.3, delay: index * 0.05 }}
                  key={project.id}
                  className="bg-[#1A1D23] border border-[#3D434C] p-8 hover:border-[#8EB8FE] transition-all duration-500 flex flex-col justify-between group glow-effect"
                >
                  <div>
                    <div className="flex justify-between items-start mb-6">
                      <span className="text-xs font-mono tracking-widest text-[#8EB8FE] uppercase bg-[#8EB8FE]/10 px-2.5 py-1">
                        {project.category}
                      </span>
                      <span className="text-xs font-mono text-[#979FAC] flex items-center gap-1">
                        <MapPin className="h-3 w-3" /> {project.location}
                      </span>
                    </div>

                    <h3 className="text-2xl font-bold text-white mb-3 group-hover:text-[#8EB8FE] transition-colors duration-300">
                      {project.title}
                    </h3>
                    
                    <p className="text-[#979FAC] text-sm font-light leading-relaxed mb-8">
                      {project.description}
                    </p>
                  </div>

                  <div className="border-t border-[#3D434C]/50 pt-6 mt-auto">
                    <div className="grid grid-cols-3 gap-4">
                      <div>
                        <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">TARGET IRR</span>
                        <span className="text-xl font-bold text-[#78D694] font-mono flex items-center gap-1">
                          {project.irr} <TrendingUp className="h-4 w-4" />
                        </span>
                      </div>
                      <div>
                        <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">MULTIPLIER</span>
                        <span className="text-xl font-bold text-[#E6E1D3] font-mono">
                          {project.multiplier}
                        </span>
                      </div>
                      <div>
                        <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">MIN. COMMIT</span>
                        <span className="text-sm font-bold text-white font-mono mt-1 block">
                          {project.minInvestment}
                        </span>
                      </div>
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>

        </div>
      </section>

      {/* SECTION 3: DEEP RESEARCH & ANALYSIS (Bridgewater Style) */}
      <section id="research" className="py-24 px-6 relative z-20 bg-[#090B0F] border-t border-[#3D434C]/50">
        <div className="max-w-7xl mx-auto">
          
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-12 items-start">
            
            {/* Left Column: Context & Branding */}
            <div className="lg:col-span-1">
              <span className="text-[#8EB8FE] text-xs font-bold tracking-[0.2em] mb-2 block">
                INTELLECTUAL CAPITAL
              </span>
              <h2 className="text-3xl md:text-5xl font-bold text-white tracking-tight mb-6">
                Bridgewater-Grade Market Intelligence
              </h2>
              <p className="text-[#979FAC] text-sm font-light leading-relaxed mb-8">
                We believe that superior returns are built on fundamental, systematic research. We publish regular macro updates and sector-specific analyses on the Quy Nhon real estate landscape to guide our investment committee and partners.
              </p>

              {/* Research Cover Generated Image */}
              <div className="relative border border-[#3D434C] p-2 bg-[#1A1D23] overflow-hidden group">
                <img 
                  src={RESEARCH_COVER} 
                  alt="Research Cover" 
                  className="w-full h-auto grayscale contrast-125 group-hover:grayscale-0 transition-all duration-700"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-[#090B0F] via-transparent to-transparent opacity-80" />
                <div className="absolute bottom-6 left-6 right-6">
                  <span className="text-[10px] font-mono tracking-widest text-[#8EB8FE] block mb-1">FEATURED REPORT</span>
                  <h4 className="text-md font-bold text-white">Quy Nhon Real Estate Megatrends 2026-2030</h4>
                </div>
              </div>
            </div>

            {/* Right Column: Research Feed (Bridgewater Style) */}
            <div className="lg:col-span-2 space-y-6">
              {researchReports.map((report, index) => (
                <div 
                  key={report.id}
                  className="bg-[#1A1D23] border border-[#3D434C] p-6 hover:border-[#8EB8FE] transition-all duration-300 cursor-pointer group"
                  onClick={() => setSelectedResearch(selectedResearch === report.id ? null : report.id)}
                >
                  <div className="flex items-center justify-between mb-4">
                    <div className="flex items-center gap-3">
                      <span className="text-xs font-mono text-[#8EB8FE] bg-[#8EB8FE]/10 px-2 py-0.5">
                        {report.category}
                      </span>
                      <span className="text-xs font-mono text-[#6B727E]">{report.date}</span>
                    </div>
                    <span className="text-xs font-mono text-[#979FAC]">{report.readTime}</span>
                  </div>

                  <h3 className="text-xl font-bold text-white mb-2 group-hover:text-[#8EB8FE] transition-colors duration-300 flex items-center justify-between">
                    {report.title}
                    <ChevronDown className={`h-5 w-5 text-[#6B727E] transition-transform duration-300 ${selectedResearch === report.id ? "rotate-180 text-[#8EB8FE]" : ""}`} />
                  </h3>

                  <p className="text-[#979FAC] text-sm font-light leading-relaxed">
                    {report.summary}
                  </p>

                  <AnimatePresence>
                    {selectedResearch === report.id && (
                      <motion.div 
                        initial={{ height: 0, opacity: 0 }}
                        animate={{ height: "auto", opacity: 1 }}
                        exit={{ height: 0, opacity: 0 }}
                        transition={{ duration: 0.3 }}
                        className="overflow-hidden mt-6 pt-6 border-t border-[#3D434C]/50"
                      >
                        <div className="bg-[#111418] p-4 border border-[#3D434C] mb-4">
                          <h4 className="text-xs font-bold tracking-widest text-[#E6E1D3] mb-2 uppercase">Key Takeaways:</h4>
                          <ul className="list-disc pl-5 text-xs text-[#979FAC] space-y-2 font-light">
                            <li>Nhon Hoi Economic Zone remains the highest-growth vector due to tax incentives and deepwater port expansion.</li>
                            <li>The hospitality yield gap in Quy Nhon stands at +3.2% compared to Da Nang, indicating premium arbitrage potential.</li>
                            <li>Regulatory tailwinds for foreign ownership in coastal villa complexes are expected to finalize by Q4 2026.</li>
                          </ul>
                        </div>

                        <div className="flex justify-between items-center">
                          <span className="text-xs font-mono text-[#6B727E]">Author: {report.author}</span>
                          <button className="flex items-center gap-2 text-xs font-bold tracking-widest text-[#8EB8FE] hover:text-white transition-colors">
                            <Download className="h-4 w-4" /> DOWNLOAD FULL PDF REPORT
                          </button>
                        </div>
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
              ))}
            </div>

          </div>

        </div>
      </section>

      {/* SECTION 4: METRICS & CALCULATOR (Binance Practical Interface) */}
      <section id="calculator" className="py-24 px-6 relative z-20 bg-[#111418] border-t border-[#3D434C]/50">
        <div className="max-w-7xl mx-auto">
          
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
            
            {/* Left Column: Interactive Calculator */}
            <div className="bg-[#1A1D23] border border-[#3D434C] p-8 glow-effect">
              <div className="flex items-center gap-2 mb-6">
                <Sliders className="h-5 w-5 text-[#8EB8FE]" />
                <h3 className="text-lg font-bold tracking-wider text-white">INVESTMENT ESTIMATOR</h3>
              </div>

              {/* Amount Slider */}
              <div className="mb-8">
                <div className="flex justify-between text-xs font-mono mb-2">
                  <span className="text-[#979FAC]">PRINCIPAL AMOUNT</span>
                  <span className="text-white font-bold">${calculatorAmount.toLocaleString()} USD</span>
                </div>
                <input 
                  type="range" 
                  min="10000" 
                  max="1000000" 
                  step="10000"
                  value={calculatorAmount} 
                  onChange={(e) => setCalculatorAmount(Number(e.target.value))}
                  className="w-full accent-[#8EB8FE] bg-[#3D434C] h-1"
                />
                <div className="flex justify-between text-[10px] text-[#6B727E] font-mono mt-1">
                  <span>$10,000</span>
                  <span>$1,000,000</span>
                </div>
              </div>

              {/* Period Slider */}
              <div className="mb-8">
                <div className="flex justify-between text-xs font-mono mb-2">
                  <span className="text-[#979FAC]">HOLDING PERIOD</span>
                  <span className="text-white font-bold">{calculatorPeriod} Years</span>
                </div>
                <input 
                  type="range" 
                  min="1" 
                  max="10" 
                  step="1"
                  value={calculatorPeriod} 
                  onChange={(e) => setCalculatorPeriod(Number(e.target.value))}
                  className="w-full accent-[#8EB8FE] bg-[#3D434C] h-1"
                />
                <div className="flex justify-between text-[10px] text-[#6B727E] font-mono mt-1">
                  <span>1 Year</span>
                  <span>10 Years</span>
                </div>
              </div>

              {/* Outputs (Binance Style HUD) */}
              <div className="grid grid-cols-2 gap-4 bg-[#111418] p-6 border border-[#3D434C] mb-6">
                <div>
                  <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">ESTIMATED IRR</span>
                  <span className="text-2xl font-bold text-[#78D694] font-mono">18.4%</span>
                </div>
                <div>
                  <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">ESTIMATED PROFIT</span>
                  <span className="text-2xl font-bold text-white font-mono">${Math.round(estimatedProfit).toLocaleString()}</span>
                </div>
                <div className="col-span-2 border-t border-[#3D434C]/50 pt-4 mt-2">
                  <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">TOTAL PORTFOLIO VALUE</span>
                  <span className="text-3xl font-bold text-[#8EB8FE] font-mono">${Math.round(estimatedReturn).toLocaleString()}</span>
                </div>
              </div>

              <button className="w-full py-4 bg-[#8EB8FE] text-[#090B0F] font-bold text-xs tracking-widest hover:bg-white transition-all duration-300">
                REQUEST STRUCTURING FOR THIS ALLOCATION
              </button>
            </div>

            {/* Right Column: Key Fund Metrics */}
            <div>
              <span className="text-[#8EB8FE] text-xs font-bold tracking-[0.2em] mb-2 block">
                FUND METRICS
              </span>
              <h2 className="text-3xl md:text-5xl font-bold text-white tracking-tight mb-8">
                Institutional-Grade Performance
              </h2>

              <div className="grid grid-cols-2 gap-8">
                <div className="border-l-2 border-[#8EB8FE] pl-6 py-2">
                  <span className="text-4xl font-bold text-white font-mono tracking-tight">$450M+</span>
                  <span className="text-xs text-[#979FAC] block mt-2 tracking-wider">ASSETS UNDER MANAGEMENT</span>
                </div>
                <div className="border-l-2 border-[#8EB8FE] pl-6 py-2">
                  <span className="text-4xl font-bold text-[#78D694] font-mono tracking-tight">19.2%</span>
                  <span className="text-xs text-[#979FAC] block mt-2 tracking-wider">HISTORICAL NET IRR</span>
                </div>
                <div className="border-l-2 border-[#8EB8FE] pl-6 py-2">
                  <span className="text-4xl font-bold text-white font-mono tracking-tight">100%</span>
                  <span className="text-xs text-[#979FAC] block mt-2 tracking-wider">ON-TIME CAPITAL DISTRIBUTION</span>
                </div>
                <div className="border-l-2 border-[#8EB8FE] pl-6 py-2">
                  <span className="text-4xl font-bold text-[#E6E1D3] font-mono tracking-tight">0.45%</span>
                  <span className="text-xs text-[#979FAC] block mt-2 tracking-wider">LOAN-TO-VALUE (LTV) RATIO</span>
                </div>
              </div>

              <div className="mt-12 bg-[#1A1D23] border border-[#3D434C] p-6 flex items-center gap-4">
                <div className="p-3 bg-[#8EB8FE]/10 text-[#8EB8FE]">
                  <Shield className="h-6 w-6" />
                </div>
                <div>
                  <h4 className="text-sm font-bold text-white mb-1">Sovereign Wealth Alignment</h4>
                  <p className="text-xs text-[#979FAC] font-light leading-relaxed">
                    EV Investment operates under the Binh Dinh Provincial Development framework, aligning our acquisitions with state-backed infrastructure timelines.
                  </p>
                </div>
              </div>
            </div>

          </div>

        </div>
      </section>

      {/* SECTION 5: TEAM & GOVERNANCE (Stronghold Team Page style) */}
      <section id="team" className="py-24 px-6 relative z-20 bg-[#090B0F] border-t border-[#3D434C]/50">
        <div className="max-w-7xl mx-auto">
          
          <div className="text-center max-w-3xl mx-auto mb-20">
            <span className="text-[#8EB8FE] text-xs font-bold tracking-[0.2em] mb-2 block">
              STEWARDSHIP
            </span>
            <h2 className="text-3xl md:text-5xl font-bold text-white tracking-tight mb-6">
              Experienced Leadership
            </h2>
            <p className="text-[#979FAC] text-sm font-light leading-relaxed">
              Our partners bring together international institutional private equity standards and deep local relationships in Quy Nhon, Vietnam.
            </p>
          </div>

          {/* Team Cards Grid (Stronghold style) */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {team.map((member, idx) => (
              <div 
                key={idx}
                className="bg-[#1A1D23] border border-[#3D434C] overflow-hidden group hover:border-[#8EB8FE] transition-all duration-500 flex flex-col justify-between"
              >
                <div>
                  {/* Team Portrait */}
                  <div className="relative overflow-hidden aspect-square bg-[#111418] border-b border-[#3D434C]">
                    <img 
                      src={TEAM_PORTRAIT} 
                      alt={member.name} 
                      className="w-full h-full object-cover grayscale contrast-115 group-hover:grayscale-0 group-hover:scale-105 transition-all duration-700"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-[#1A1D23] via-transparent to-transparent opacity-60" />
                  </div>

                  <div className="p-8">
                    <span className="text-[10px] font-mono tracking-widest text-[#8EB8FE] uppercase block mb-1">
                      {member.role}
                    </span>
                    <h3 className="text-xl font-bold text-white mb-4">
                      {member.name}
                    </h3>
                    <p className="text-[#979FAC] text-xs font-light leading-relaxed mb-6">
                      {member.bio}
                    </p>
                  </div>
                </div>

                <div className="px-8 pb-8 pt-4 border-t border-[#3D434C]/30 bg-[#111418]/30">
                  <span className="text-[10px] text-[#6B727E] tracking-wider block mb-1">PRIMARY FOCUS</span>
                  <span className="text-xs font-semibold text-[#E6E1D3] tracking-wide block">
                    {member.focus}
                  </span>
                </div>
              </div>
            ))}
          </div>

        </div>
      </section>

      {/* FOOTER */}
      <footer className="bg-[#090B0F] text-[#979FAC] border-t border-[#3D434C]/50 py-16 px-6 relative z-20">
        <div className="max-w-7xl mx-auto">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-12 mb-12">
            
            <div className="md:col-span-2">
              <Logo className="h-10 mb-6" />
              <p className="text-xs font-light leading-relaxed max-w-sm mb-6">
                EV Investment is a premier real estate private equity fund focused exclusively on the coastal expansion of Quy Nhon city, Binh Dinh province, Vietnam.
              </p>
              <span className="text-xs font-mono text-[#6B727E]">© 2026 EV Investment Fund. All rights reserved.</span>
            </div>

            <div>
              <h4 className="text-xs font-bold tracking-widest text-white uppercase mb-4">LEGAL & REGULATORY</h4>
              <ul className="space-y-3 text-xs font-light">
                <li><a href="#" className="hover:text-white transition-colors">Fund Prospectus</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Regulatory Disclosures</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Sovereign Wealth Alignment</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Privacy Policy</a></li>
              </ul>
            </div>

            <div>
              <h4 className="text-xs font-bold tracking-widest text-white uppercase mb-4">OFFICES</h4>
              <p className="text-xs font-light leading-relaxed mb-4">
                <strong>Quy Nhon Office:</strong><br />
                An Phu Thinh Plaza, Quy Nhon City, Binh Dinh, Vietnam
              </p>
              <p className="text-xs font-light leading-relaxed">
                <strong>Ho Chi Minh City Office:</strong><br />
                Deutsches Haus, District 1, HCMC, Vietnam
              </p>
            </div>

          </div>

          <div className="border-t border-[#3D434C]/30 pt-8 flex flex-col sm:flex-row justify-between items-center gap-4">
            <span className="text-xs font-mono text-[#6B727E]">Designed for Institutional & Accredited Investors Only.</span>
            <div className="flex gap-6 text-xs">
              <a href="#" className="hover:text-white transition-colors">LinkedIn</a>
              <a href="#" className="hover:text-white transition-colors">Bloomberg Terminal: EVINV &lt;GO&gt;</a>
            </div>
          </div>
        </div>
      </footer>

    </div>
  );
}
