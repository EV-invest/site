import { Playfair_Display, Space_Grotesk, Space_Mono } from "next/font/google";

// Self-hosted via next/font (no render-blocking <link> to Google Fonts).
// Each exposes a CSS variable consumed by globals.css / the Tailwind theme.
export const fontGrotesk = Space_Grotesk({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-grotesk",
});

export const fontPlayfair = Playfair_Display({
  subsets: ["latin"],
  display: "swap",
  style: ["normal", "italic"],
  variable: "--font-playfair",
});

export const fontMono = Space_Mono({
  subsets: ["latin"],
  weight: ["400", "700"],
  style: ["normal", "italic"],
  display: "swap",
  variable: "--font-space-mono",
});
