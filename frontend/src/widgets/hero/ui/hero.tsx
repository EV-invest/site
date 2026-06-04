"use client";

import { useState, useEffect, useRef } from "react";
import { ArrowRight } from "lucide-react";
import { Button } from "@/shared/ui/button";
import { Text } from "@/shared/ui/text";
import { ASSETS } from "@/shared/config/assets";

export function HeroA() {
  const [zoomLevel, setZoomLevel] = useState(1);
  const heroRef = useRef<HTMLDivElement>(null);

  // Обработка интерактивного зума (Stronghold Fund Metaphor)
  useEffect(() => {
    const handleScroll = () => {
      if (!heroRef.current) return;
      const scrollY = window.scrollY;
      const threshold = window.innerHeight;

      if (scrollY < threshold) {
        // Рассчитываем зум от 1 до 4 в зависимости от прокрутки
        const factor = 1 + (scrollY / threshold) * 3;
        setZoomLevel(factor);
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  // 2. HERO PAGE WITH ZOOM METAPHOR (Stronghold Fund Inspired)
  return (
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
          backgroundPosition: "center",
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
            opacity: Math.max(0.1, 1 - (zoomLevel - 1) * 0.5),
          }}
        >
          <h1 className="text-4xl sm:text-6xl md:text-8xl font-serif-display font-light text-white leading-tight mb-6">
            Invest in{" "}
            <span className="italic text-main-accent-t1">Quy Nhon</span>
            <br />
            Through Institutional Vision.
          </h1>

          <Text className="sm:text-base md:text-lg max-w-2xl mx-auto mb-12">
            EV Investment bridges the gap between premium coastal real estate
            development and sophisticated investors. Experience high-yield real
            estate assets in Vietnam’s fastest-growing coastal hub.
          </Text>
        </div>

        {/* Interactive Action / Zoom Indicator */}
        <div className="flex flex-col items-center gap-4">
          <Button
            className="bg-main-mist text-main-brand hover:bg-main-accent-t1 hover:text-main-black hover:scale-105 active:scale-95 transition-all duration-300 font-mono-tech text-xs tracking-widest uppercase px-8 py-6 rounded-none"
            onClick={() => {
              const portfolioSec = document.getElementById("portfolio");
              portfolioSec?.scrollIntoView({ behavior: "smooth" });
            }}
          >
            Explore Assets <ArrowRight className="w-4 h-4 ml-2" />
          </Button>

          <div className="mt-8 flex flex-col items-center gap-1.5">
            <Text asChild variant="secondary">
              <span className="text-[9px] font-mono-tech tracking-[0.3em] uppercase">
                Scroll to zoom in & discover
              </span>
            </Text>
          </div>
        </div>
      </div>

      {/* Bottom stats ribbon — accent-led, with ONE headline highlight (Target IRR). */}
      <div className="absolute bottom-0 left-0 w-full bg-main-black/80 border-t border-main-mist/10 py-6 backdrop-blur-sm z-20">
        <div className="container grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
          <div>
            <Text
              variant="secondary"
              className="text-xs font-mono-tech uppercase tracking-widest mb-1"
            >
              Target IRR
            </Text>
            <p className="text-2xl sm:text-3xl font-serif-display text-main-accent-t3 font-bold">
              22.4% +
            </p>
          </div>
          <div>
            <Text
              variant="secondary"
              className="text-xs font-mono-tech uppercase tracking-widest mb-1"
            >
              AUM Under Advisory
            </Text>
            <p className="text-2xl sm:text-3xl font-serif-display text-white font-bold">
              $145M
            </p>
          </div>
          <div>
            <Text
              variant="secondary"
              className="text-xs font-mono-tech uppercase tracking-widest mb-1"
            >
              Coastal Coastline
            </Text>
            <p className="text-2xl sm:text-3xl font-serif-display text-main-accent-t1 font-bold">
              72 km
            </p>
          </div>
          <div>
            <Text
              variant="secondary"
              className="text-xs font-mono-tech uppercase tracking-widest mb-1"
            >
              Institutional Grade
            </Text>
            <p className="text-2xl sm:text-3xl font-serif-display text-main-accent-t4 font-bold">
              100%
            </p>
            {/* Lotus jewel #1 — the punchline stat */}
          </div>
        </div>
      </div>
    </section>
  );
}
