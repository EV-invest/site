import { Button } from "@/components/ui/button";
import { ArrowRight, TrendingUp, Building2, Zap, Globe, BarChart3, Mail } from "lucide-react";
import { useState } from "react";

/**
 * Design Philosophy: Futuristic Minimalism
 * - Geometric abstraction with diagonal cuts
 * - Neon green (#00D084) and cool blue (#00B4D8) accents
 * - Minimal elements, maximum whitespace
 * - Space Grotesk for headers (futuristic), Inter for body
 * - Smooth animations and micro-interactions
 */

export default function Home() {
  const [hoveredStrategy, setHoveredStrategy] = useState<number | null>(null);

  return (
    <div className="min-h-screen bg-background text-foreground overflow-hidden">
      {/* Header */}
      <header className="sticky top-0 z-50 bg-background/80 backdrop-blur-md border-b border-border">
        <div className="container flex items-center justify-between py-4">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center">
              <Building2 className="w-5 h-5 text-white" />
            </div>
            <span className="font-bold text-lg tracking-tight">RealEstate Fund</span>
          </div>
          
          <nav className="hidden md:flex items-center gap-8">
            <a href="#thesis" className="text-sm font-medium hover:text-primary transition-colors">Thesis</a>
            <a href="#strategies" className="text-sm font-medium hover:text-primary transition-colors">Strategies</a>
            <a href="#portfolio" className="text-sm font-medium hover:text-primary transition-colors">Portfolio</a>
            <a href="#contact" className="text-sm font-medium hover:text-primary transition-colors">Contact</a>
          </nav>
          
          <Button className="btn-primary hidden sm:inline-flex gap-2">
            <Mail className="w-4 h-4" />
            Get Started
          </Button>
        </div>
      </header>

      {/* Hero Section */}
      <section className="relative min-h-screen flex items-center justify-center overflow-hidden">
        {/* Background Image */}
        <div 
          className="absolute inset-0 z-0"
          style={{
            backgroundImage: 'url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/hero-abstract-geometric-aN4Syi2GikJPmcNcLqLZW6.webp)',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            backgroundAttachment: 'fixed',
          }}
        />
        
        {/* Overlay */}
        <div className="absolute inset-0 bg-gradient-to-b from-background/40 via-background/20 to-background/80 z-10" />
        
        {/* Content */}
        <div className="relative z-20 container max-w-4xl text-center px-4 md:px-8">
          <div className="space-y-6 fade-in-up">
            <div className="inline-block px-4 py-2 rounded-full border border-primary/30 bg-primary/5 backdrop-blur-sm">
              <span className="text-sm font-semibold text-primary">Real Estate Reimagined</span>
            </div>
            
            <h1 className="text-5xl md:text-7xl font-bold leading-tight tracking-tight">
              The Future of <span className="text-gradient">Property Investment</span>
            </h1>
            
            <p className="text-xl md:text-2xl text-muted-foreground max-w-2xl mx-auto leading-relaxed">
              Institutional-grade real estate investment powered by data, technology, and forward-thinking strategy.
            </p>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center pt-8">
              <Button className="btn-primary gap-2 h-12 text-base">
                Explore Opportunities
                <ArrowRight className="w-5 h-5" />
              </Button>
              <Button className="btn-secondary h-12 text-base">
                Learn More
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* Investment Thesis Section */}
      <section id="thesis" className="relative py-24 md:py-32 overflow-hidden">
        <div 
          className="absolute inset-0 z-0"
          style={{
            backgroundImage: 'url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/investment-thesis-bg-FiurCJtFbt5x2tz3UJ3qfM.webp)',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            opacity: 0.5,
          }}
        />
        
        <div className="absolute inset-0 bg-background/60 z-5" />
        
        <div className="relative z-10 container max-w-5xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold mb-4">Investment Thesis</h2>
            <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
              We identify and capitalize on emerging opportunities in the real estate market.
            </p>
          </div>
          
          <div className="grid md:grid-cols-3 gap-8">
            {[
              {
                icon: TrendingUp,
                title: "Market Growth",
                description: "Positioned to capture long-term appreciation in high-growth markets and emerging segments."
              },
              {
                icon: Zap,
                title: "Operational Excellence",
                description: "Technology-driven approach to asset management, tenant relations, and value creation."
              },
              {
                icon: Globe,
                title: "Global Diversification",
                description: "Strategic exposure across geographies and property types to optimize risk-adjusted returns."
              }
            ].map((item, idx) => (
              <div key={idx} className="card-minimal group hover:border-primary/50 hover:shadow-lg transition-all duration-300">
                <div className="w-12 h-12 rounded-lg bg-gradient-to-br from-primary/20 to-accent/20 flex items-center justify-center mb-4 group-hover:from-primary/40 group-hover:to-accent/40 transition-all">
                  <item.icon className="w-6 h-6 text-primary" />
                </div>
                <h3 className="text-xl font-bold mb-2">{item.title}</h3>
                <p className="text-muted-foreground">{item.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Strategies Section */}
      <section id="strategies" className="relative py-24 md:py-32 bg-gradient-to-b from-background via-background to-primary/5">
        <div className="container max-w-6xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold mb-4">Investment Strategies</h2>
            <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
              Diversified approaches tailored to market conditions and investor objectives.
            </p>
          </div>
          
          <div className="grid md:grid-cols-2 gap-8">
            {[
              {
                number: "01",
                title: "Core Stabilized",
                description: "Established properties with consistent cash flows and long-term tenant relationships.",
                metrics: ["5-7% Target Yield", "Low Volatility", "Institutional Grade"]
              },
              {
                number: "02",
                title: "Value-Add Development",
                description: "Strategic renovations and repositioning to unlock significant upside potential.",
                metrics: ["12-15% IRR", "3-5 Year Hold", "Active Management"]
              },
              {
                number: "03",
                title: "Emerging Markets",
                description: "High-growth regions with demographic tailwinds and infrastructure development.",
                metrics: ["15-20% IRR", "Growth Focused", "Market Leaders"]
              },
              {
                number: "04",
                title: "Mixed-Use Innovation",
                description: "Adaptive reuse and modern mixed-use developments for evolving urban needs.",
                metrics: ["10-14% IRR", "Future-Proof", "Flexible Tenancy"]
              }
            ].map((strategy, idx) => (
              <div
                key={idx}
                className="relative group cursor-pointer"
                onMouseEnter={() => setHoveredStrategy(idx)}
                onMouseLeave={() => setHoveredStrategy(null)}
              >
                <div className="absolute inset-0 bg-gradient-to-br from-primary/10 to-accent/10 rounded-lg opacity-0 group-hover:opacity-100 transition-all duration-300 blur-xl" />
                
                <div className="relative card-minimal border-2 border-transparent group-hover:border-primary/30 h-full">
                  <div className="text-5xl font-bold text-primary/20 mb-4 group-hover:text-primary/40 transition-colors">
                    {strategy.number}
                  </div>
                  
                  <h3 className="text-2xl font-bold mb-3">{strategy.title}</h3>
                  <p className="text-muted-foreground mb-6">{strategy.description}</p>
                  
                  <div className="space-y-2 pt-6 border-t border-border">
                    {strategy.metrics.map((metric, midx) => (
                      <div key={midx} className="flex items-center gap-2 text-sm">
                        <div className="w-1.5 h-1.5 rounded-full bg-primary" />
                        <span className="text-muted-foreground">{metric}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Portfolio Section */}
      <section id="portfolio" className="relative py-24 md:py-32">
        <div 
          className="absolute inset-0 z-0"
          style={{
            backgroundImage: 'url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/portfolio-grid-accent-WinPocsLWLcPuAoz7axpGH.webp)',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            opacity: 0.3,
          }}
        />
        
        <div className="relative z-10 container max-w-6xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold mb-4">Portfolio Overview</h2>
            <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
              Curated selection of institutional-quality real estate assets.
            </p>
          </div>
          
          <div className="grid md:grid-cols-3 gap-6 mb-12">
            {[
              { label: "Total Assets", value: "$2.4B+", icon: Building2 },
              { label: "Properties", value: "145+", icon: BarChart3 },
              { label: "Avg. Yield", value: "7.2%", icon: TrendingUp }
            ].map((stat, idx) => (
              <div key={idx} className="card-minimal text-center">
                <stat.icon className="w-8 h-8 text-primary mx-auto mb-4" />
                <p className="text-sm text-muted-foreground mb-1">{stat.label}</p>
                <p className="text-3xl font-bold text-gradient">{stat.value}</p>
              </div>
            ))}
          </div>
          
          <div className="grid md:grid-cols-2 gap-8">
            {[
              { type: "Office", allocation: "32%", color: "from-primary" },
              { type: "Residential", allocation: "28%", color: "from-accent" },
              { type: "Retail", allocation: "18%", color: "from-primary/60" },
              { type: "Industrial", allocation: "22%", color: "from-accent/60" }
            ].map((item, idx) => (
              <div key={idx} className="card-minimal">
                <div className="flex items-center justify-between mb-4">
                  <h4 className="font-semibold">{item.type}</h4>
                  <span className="text-lg font-bold text-primary">{item.allocation}</span>
                </div>
                <div className="w-full h-2 bg-muted rounded-full overflow-hidden">
                  <div 
                    className={`h-full bg-gradient-to-r ${item.color} to-accent`}
                    style={{ width: item.allocation }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Market Intelligence Section */}
      <section className="relative py-24 md:py-32 bg-gradient-to-b from-background to-primary/5">
        <div 
          className="absolute inset-0 z-0"
          style={{
            backgroundImage: 'url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/market-intelligence-visual-Q46D2cSCbwq3bE52ykdXXi.webp)',
            backgroundSize: 'cover',
            backgroundPosition: 'center',
            opacity: 0.4,
          }}
        />
        
        <div className="relative z-10 container max-w-5xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold mb-4">Market Intelligence</h2>
            <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
              Latest insights and market updates from our research team.
            </p>
          </div>
          
          <div className="space-y-6">
            {[
              {
                date: "MAY 15, 2026",
                title: "Urban Residential Market Shows Strong Momentum",
                excerpt: "Demand for mixed-use residential properties continues to outpace supply in major metropolitan areas."
              },
              {
                date: "MAY 08, 2026",
                title: "Industrial Sector Benefits from E-Commerce Growth",
                excerpt: "Last-mile logistics facilities command premium valuations as supply chain optimization accelerates."
              },
              {
                date: "APR 30, 2026",
                title: "Interest Rate Environment Stabilizes Portfolio",
                excerpt: "Fixed-rate debt refinancing opportunities emerge as market conditions normalize."
              }
            ].map((news, idx) => (
              <div key={idx} className="card-minimal group hover:border-primary/50 cursor-pointer">
                <div className="flex items-start justify-between gap-4">
                  <div className="flex-1">
                    <p className="text-xs font-semibold text-primary uppercase tracking-wider mb-2">
                      {news.date}
                    </p>
                    <h4 className="text-lg font-bold mb-2 group-hover:text-primary transition-colors">
                      {news.title}
                    </h4>
                    <p className="text-muted-foreground">{news.excerpt}</p>
                  </div>
                  <ArrowRight className="w-5 h-5 text-primary opacity-0 group-hover:opacity-100 transition-all transform group-hover:translate-x-1" />
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Contact / CTA Section */}
      <section id="contact" className="relative py-24 md:py-32 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-r from-primary/5 via-accent/5 to-primary/5 z-0" />
        
        <div className="relative z-10 container max-w-3xl text-center">
          <h2 className="text-4xl md:text-5xl font-bold mb-6">Ready to Invest?</h2>
          <p className="text-lg text-muted-foreground mb-8 max-w-xl mx-auto">
            Join institutional investors in accessing premium real estate opportunities. Contact our team to discuss your investment objectives.
          </p>
          
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button className="btn-primary gap-2 h-12 text-base">
              Schedule a Consultation
              <Mail className="w-5 h-5" />
            </Button>
            <Button className="btn-secondary h-12 text-base">
              Download Prospectus
            </Button>
          </div>
          
          <div className="mt-12 pt-12 border-t border-border">
            <p className="text-xs text-muted-foreground">
              This material is for informational purposes only and does not constitute an offer to sell or a solicitation to buy any securities.
            </p>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-border bg-background/50 backdrop-blur-sm">
        <div className="container py-12">
          <div className="grid md:grid-cols-4 gap-8 mb-8">
            <div>
              <div className="flex items-center gap-2 mb-4">
                <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center">
                  <Building2 className="w-5 h-5 text-white" />
                </div>
                <span className="font-bold">RealEstate Fund</span>
              </div>
              <p className="text-sm text-muted-foreground">Institutional real estate investment for the future.</p>
            </div>
            
            <div>
              <h4 className="font-semibold mb-4">Navigation</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><a href="#thesis" className="hover:text-primary transition-colors">Thesis</a></li>
                <li><a href="#strategies" className="hover:text-primary transition-colors">Strategies</a></li>
                <li><a href="#portfolio" className="hover:text-primary transition-colors">Portfolio</a></li>
              </ul>
            </div>
            
            <div>
              <h4 className="font-semibold mb-4">Resources</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><a href="#" className="hover:text-primary transition-colors">Research</a></li>
                <li><a href="#" className="hover:text-primary transition-colors">News</a></li>
                <li><a href="#" className="hover:text-primary transition-colors">Reports</a></li>
              </ul>
            </div>
            
            <div>
              <h4 className="font-semibold mb-4">Legal</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><a href="#" className="hover:text-primary transition-colors">Privacy</a></li>
                <li><a href="#" className="hover:text-primary transition-colors">Terms</a></li>
                <li><a href="#" className="hover:text-primary transition-colors">Disclaimer</a></li>
              </ul>
            </div>
          </div>
          
          <div className="border-t border-border pt-8 flex flex-col md:flex-row items-center justify-between text-sm text-muted-foreground">
            <p>&copy; 2026 RealEstate Fund. All rights reserved.</p>
            <div className="flex gap-6 mt-4 md:mt-0">
              <a href="#" className="hover:text-primary transition-colors">Twitter</a>
              <a href="#" className="hover:text-primary transition-colors">LinkedIn</a>
              <a href="#" className="hover:text-primary transition-colors">Email</a>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}
