//! Translation of `manus/client/src/pages/Home.tsx`.
//!
//! Markup and copy are preserved verbatim. The original
//! `useState<number | null>(hoveredStrategy)` was set on hover but never
//! consumed in rendering (hover styling lives in CSS `group-hover`), so it is
//! intentionally dropped.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowRight, Building2, ChartColumn, Globe, Mail, TrendingUp, Zap};
use shadcn_ui::{Badge, BadgeVariant, Button, ButtonSize, ButtonVariant};

use crate::components::{BrandMark, Section, SectionCTA, SectionHeader};

const HERO_BG: &str = "url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/hero-abstract-geometric-aN4Syi2GikJPmcNcLqLZW6.webp)";
const THESIS_BG: &str = "url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/investment-thesis-bg-FiurCJtFbt5x2tz3UJ3qfM.webp)";
const PORTFOLIO_BG: &str = "url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/portfolio-grid-accent-WinPocsLWLcPuAoz7axpGH.webp)";
const MARKET_BG: &str = "url(https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/8bbPqutcwodgn4efm3i7rX/market-intelligence-visual-Q46D2cSCbwq3bE52ykdXXi.webp)";

#[component]
pub fn Home() -> Element {
	rsx! {
		div { class: "min-h-screen bg-background text-foreground overflow-hidden",

			// ---------- Header ----------
			header {
				class: "sticky top-0 z-50 bg-background/80 backdrop-blur-md border-b border-border",
				div {
					class: "container flex items-center justify-between py-4",
					BrandMark { text_class: "font-bold text-lg tracking-tight".to_string() }

					nav {
						class: "hidden md:flex items-center gap-8",
						a { href: "#thesis", class: "text-sm font-medium hover:text-primary transition-colors", "Thesis" }
						a { href: "#strategies", class: "text-sm font-medium hover:text-primary transition-colors", "Strategies" }
						a { href: "#portfolio", class: "text-sm font-medium hover:text-primary transition-colors", "Portfolio" }
						a { href: "#contact", class: "text-sm font-medium hover:text-primary transition-colors", "Contact" }
					}

					Button {
						variant: ButtonVariant::Default,
						class: "hidden sm:inline-flex",
						Mail { class: "w-4 h-4" }
						"Get Started"
					}
				}
			}

			// ---------- Hero ----------
			section {
				class: "relative min-h-screen flex items-center justify-center overflow-hidden",
				div {
					class: "absolute inset-0 z-0",
					style: "background-image: {HERO_BG}; background-size: cover; background-position: center; background-attachment: fixed;",
				}
				div { class: "absolute inset-0 bg-gradient-to-b from-background/40 via-background/20 to-background/80 z-10" }
				div {
					class: "relative z-20 container max-w-4xl text-center px-4 md:px-8",
					div { class: "space-y-6 fade-in-up",
						Badge {
							variant: BadgeVariant::Outline,
							class: "px-4 py-2 text-sm font-semibold text-primary border-primary/30 bg-primary/5 backdrop-blur-sm",
							"Real Estate Reimagined"
						}
						h1 {
							class: "text-5xl md:text-7xl font-bold leading-tight tracking-tight",
							"The Future of "
							span { class: "text-gradient", "Property Investment" }
						}
						p {
							class: "text-xl md:text-2xl text-muted-foreground max-w-2xl mx-auto leading-relaxed",
							"Institutional-grade real estate investment powered by data, technology, and forward-thinking strategy."
						}
						div { class: "flex flex-col sm:flex-row gap-4 justify-center pt-8",
							Button {
								variant: ButtonVariant::Default,
								size: ButtonSize::Lg,
								class: "text-base",
								"Explore Opportunities"
								ArrowRight { class: "w-5 h-5" }
							}
							Button {
								variant: ButtonVariant::Outline,
								size: ButtonSize::Lg,
								class: "text-base",
								"Learn More"
							}
						}
					}
				}
			}

			// ---------- Investment Thesis ----------
			Section {
				id: "thesis".to_string(),
				bg_image: THESIS_BG.to_string(),
				bg_opacity: 0.5,
				overlay_class: "absolute inset-0 bg-background/60".to_string(),
				extra_class: "overflow-hidden".to_string(),
				max_width: "5xl".to_string(),
				SectionHeader {
					title: "Investment Thesis".to_string(),
					description: "We identify and capitalize on emerging opportunities in the real estate market.".to_string(),
				}
				div { class: "grid md:grid-cols-3 gap-8",
					ThesisCard {
						title: "Market Growth".to_string(),
						description: "Positioned to capture long-term appreciation in high-growth markets and emerging segments.".to_string(),
						icon: rsx! { TrendingUp { class: "w-6 h-6 text-primary" } },
					}
					ThesisCard {
						title: "Operational Excellence".to_string(),
						description: "Technology-driven approach to asset management, tenant relations, and value creation.".to_string(),
						icon: rsx! { Zap { class: "w-6 h-6 text-primary" } },
					}
					ThesisCard {
						title: "Global Diversification".to_string(),
						description: "Strategic exposure across geographies and property types to optimize risk-adjusted returns.".to_string(),
						icon: rsx! { Globe { class: "w-6 h-6 text-primary" } },
					}
				}
			}

			// ---------- Strategies ----------
			Section {
				id: "strategies".to_string(),
				extra_class: "bg-gradient-to-b from-background via-background to-primary/5".to_string(),
				max_width: "6xl".to_string(),
				SectionHeader {
					title: "Investment Strategies".to_string(),
					description: "Diversified approaches tailored to market conditions and investor objectives.".to_string(),
				}
				div { class: "grid md:grid-cols-2 gap-8",
					StrategyCard {
						number: "01".to_string(),
						title: "Core Stabilized".to_string(),
						description: "Established properties with consistent cash flows and long-term tenant relationships.".to_string(),
						metrics: vec!["5-7% Target Yield".to_string(), "Low Volatility".to_string(), "Institutional Grade".to_string()],
					}
					StrategyCard {
						number: "02".to_string(),
						title: "Value-Add Development".to_string(),
						description: "Strategic renovations and repositioning to unlock significant upside potential.".to_string(),
						metrics: vec!["12-15% IRR".to_string(), "3-5 Year Hold".to_string(), "Active Management".to_string()],
					}
					StrategyCard {
						number: "03".to_string(),
						title: "Emerging Markets".to_string(),
						description: "High-growth regions with demographic tailwinds and infrastructure development.".to_string(),
						metrics: vec!["15-20% IRR".to_string(), "Growth Focused".to_string(), "Market Leaders".to_string()],
					}
					StrategyCard {
						number: "04".to_string(),
						title: "Mixed-Use Innovation".to_string(),
						description: "Adaptive reuse and modern mixed-use developments for evolving urban needs.".to_string(),
						metrics: vec!["10-14% IRR".to_string(), "Future-Proof".to_string(), "Flexible Tenancy".to_string()],
					}
				}
			}

			// ---------- Portfolio ----------
			Section {
				id: "portfolio".to_string(),
				bg_image: PORTFOLIO_BG.to_string(),
				bg_opacity: 0.3,
				max_width: "6xl".to_string(),
				SectionHeader {
					title: "Portfolio Overview".to_string(),
					description: "Curated selection of institutional-quality real estate assets.".to_string(),
				}
				div { class: "grid md:grid-cols-3 gap-6 mb-12",
					PortfolioStat { label: "Total Assets".to_string(), value: "$2.4B+".to_string(), icon: rsx! { Building2 { class: "w-8 h-8 text-primary mx-auto mb-4" } } }
					PortfolioStat { label: "Properties".to_string(), value: "145+".to_string(), icon: rsx! { ChartColumn { class: "w-8 h-8 text-primary mx-auto mb-4" } } }
					PortfolioStat { label: "Avg. Yield".to_string(), value: "7.2%".to_string(), icon: rsx! { TrendingUp { class: "w-8 h-8 text-primary mx-auto mb-4" } } }
				}
				div { class: "grid md:grid-cols-2 gap-8",
					AllocationBar { kind: "Office".to_string(), allocation: "32%".to_string(), gradient_from: "from-primary".to_string() }
					AllocationBar { kind: "Residential".to_string(), allocation: "28%".to_string(), gradient_from: "from-accent".to_string() }
					AllocationBar { kind: "Retail".to_string(), allocation: "18%".to_string(), gradient_from: "from-primary/60".to_string() }
					AllocationBar { kind: "Industrial".to_string(), allocation: "22%".to_string(), gradient_from: "from-accent/60".to_string() }
				}
			}

			// ---------- Market Intelligence ----------
			Section {
				bg_image: MARKET_BG.to_string(),
				bg_opacity: 0.4,
				extra_class: "bg-gradient-to-b from-background to-primary/5".to_string(),
				max_width: "5xl".to_string(),
				SectionHeader {
					title: "Market Intelligence".to_string(),
					description: "Latest insights and market updates from our research team.".to_string(),
				}
				div { class: "space-y-6",
					NewsItem {
						date: "MAY 15, 2026".to_string(),
						title: "Urban Residential Market Shows Strong Momentum".to_string(),
						excerpt: "Demand for mixed-use residential properties continues to outpace supply in major metropolitan areas.".to_string(),
					}
					NewsItem {
						date: "MAY 08, 2026".to_string(),
						title: "Industrial Sector Benefits from E-Commerce Growth".to_string(),
						excerpt: "Last-mile logistics facilities command premium valuations as supply chain optimization accelerates.".to_string(),
					}
					NewsItem {
						date: "APR 30, 2026".to_string(),
						title: "Interest Rate Environment Stabilizes Portfolio".to_string(),
						excerpt: "Fixed-rate debt refinancing opportunities emerge as market conditions normalize.".to_string(),
					}
				}
			}

			// ---------- Contact / CTA ----------
			Section {
				id: "contact".to_string(),
				overlay_class: "absolute inset-0 bg-gradient-to-r from-primary/5 via-accent/5 to-primary/5 z-0".to_string(),
				extra_class: "overflow-hidden".to_string(),
				max_width: "3xl".to_string(),
				SectionCTA {
					title: "Ready to Invest?".to_string(),
					description: "Join institutional investors in accessing premium real estate opportunities. Contact our team to discuss your investment objectives.".to_string(),
					primary_label: "Schedule a Consultation".to_string(),
					primary_icon: rsx! { Mail { class: "w-5 h-5" } },
					secondary_label: "Download Prospectus".to_string(),
					disclaimer: "This material is for informational purposes only and does not constitute an offer to sell or a solicitation to buy any securities.".to_string(),
				}
			}

			// ---------- Footer ----------
			footer { class: "border-t border-border bg-background/50 backdrop-blur-sm",
				div { class: "container py-12",
					div { class: "grid md:grid-cols-4 gap-8 mb-8",
						div {
							div { class: "mb-4",
								BrandMark { text_class: "font-bold".to_string() }
							}
							p { class: "text-sm text-muted-foreground", "Institutional real estate investment for the future." }
						}
						FooterColumn {
							heading: "Navigation".to_string(),
							links: vec![
								("Thesis".to_string(), "#thesis".to_string()),
								("Strategies".to_string(), "#strategies".to_string()),
								("Portfolio".to_string(), "#portfolio".to_string()),
							],
						}
						FooterColumn {
							heading: "Resources".to_string(),
							links: vec![
								("Research".to_string(), "#".to_string()),
								("News".to_string(), "#".to_string()),
								("Reports".to_string(), "#".to_string()),
							],
						}
						FooterColumn {
							heading: "Legal".to_string(),
							links: vec![
								("Privacy".to_string(), "#".to_string()),
								("Terms".to_string(), "#".to_string()),
								("Disclaimer".to_string(), "#".to_string()),
							],
						}
					}
					div { class: "border-t border-border pt-8 flex flex-col md:flex-row items-center justify-between text-sm text-muted-foreground",
						p { "© 2026 RealEstate Fund. All rights reserved." }
						div { class: "flex gap-6 mt-4 md:mt-0",
							a { href: "#", class: "hover:text-primary transition-colors", "Twitter" }
							a { href: "#", class: "hover:text-primary transition-colors", "LinkedIn" }
							a { href: "#", class: "hover:text-primary transition-colors", "Email" }
						}
					}
				}
			}
		}
	}
}

#[component]
fn ThesisCard(title: String, description: String, icon: Element) -> Element {
	rsx! {
		div { class: "card-minimal group hover:border-primary/50 hover:shadow-lg transition-all duration-300",
			div {
				class: "w-12 h-12 rounded-lg bg-gradient-to-br from-primary/20 to-accent/20 flex items-center justify-center mb-4 group-hover:from-primary/40 group-hover:to-accent/40 transition-all",
				{icon}
			}
			h3 { class: "text-xl font-bold mb-2", "{title}" }
			p { class: "text-muted-foreground", "{description}" }
		}
	}
}

#[component]
fn StrategyCard(number: String, title: String, description: String, metrics: Vec<String>) -> Element {
	rsx! {
		div { class: "relative group cursor-pointer",
			div { class: "absolute inset-0 bg-gradient-to-br from-primary/10 to-accent/10 rounded-lg opacity-0 group-hover:opacity-100 transition-all duration-300 blur-xl" }
			div { class: "relative card-minimal border-2 border-transparent group-hover:border-primary/30 h-full",
				div { class: "text-5xl font-bold text-primary/20 mb-4 group-hover:text-primary/40 transition-colors", "{number}" }
				h3 { class: "text-2xl font-bold mb-3", "{title}" }
				p { class: "text-muted-foreground mb-6", "{description}" }
				div { class: "space-y-2 pt-6 border-t border-border",
					for metric in metrics {
						div { class: "flex items-center gap-2 text-sm",
							div { class: "w-1.5 h-1.5 rounded-full bg-primary" }
							span { class: "text-muted-foreground", "{metric}" }
						}
					}
				}
			}
		}
	}
}

#[component]
fn PortfolioStat(label: String, value: String, icon: Element) -> Element {
	rsx! {
		div { class: "card-minimal text-center",
			{icon}
			p { class: "text-sm text-muted-foreground mb-1", "{label}" }
			p { class: "text-3xl font-bold text-gradient", "{value}" }
		}
	}
}

#[component]
fn AllocationBar(kind: String, allocation: String, gradient_from: String) -> Element {
	rsx! {
		div { class: "card-minimal",
			div { class: "flex items-center justify-between mb-4",
				h4 { class: "font-semibold", "{kind}" }
				span { class: "text-lg font-bold text-primary", "{allocation}" }
			}
			div { class: "w-full h-2 bg-muted rounded-full overflow-hidden",
				div {
					class: "h-full bg-gradient-to-r {gradient_from} to-accent",
					style: "width: {allocation};",
				}
			}
		}
	}
}

#[component]
fn NewsItem(date: String, title: String, excerpt: String) -> Element {
	rsx! {
		div { class: "card-minimal group hover:border-primary/50 cursor-pointer",
			div { class: "flex items-start justify-between gap-4",
				div { class: "flex-1",
					p { class: "text-xs font-semibold text-primary uppercase tracking-wider mb-2", "{date}" }
					h4 { class: "text-lg font-bold mb-2 group-hover:text-primary transition-colors", "{title}" }
					p { class: "text-muted-foreground", "{excerpt}" }
				}
				ArrowRight { class: "w-5 h-5 text-primary opacity-0 group-hover:opacity-100 transition-all transform group-hover:translate-x-1" }
			}
		}
	}
}

#[component]
fn FooterColumn(heading: String, links: Vec<(String, String)>) -> Element {
	rsx! {
		div {
			h4 { class: "font-semibold mb-4", "{heading}" }
			ul { class: "space-y-2 text-sm text-muted-foreground",
				for (label, href) in links {
					li {
						a { href: "{href}", class: "hover:text-primary transition-colors", "{label}" }
					}
				}
			}
		}
	}
}
