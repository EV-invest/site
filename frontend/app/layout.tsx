import type { ReactNode } from "react";
import Script from "next/script";
import { fontInter, fontPlayfair } from "@/application/styles/fonts";
import { Providers } from "@/application/providers";
import { Header, Footer } from "@/application/layout";
import "@/application/styles/globals.css";

export { metadata, viewport } from "@/application/metadata";

const analyticsEndpoint = process.env.NEXT_PUBLIC_ANALYTICS_ENDPOINT;
const analyticsWebsiteId = process.env.NEXT_PUBLIC_ANALYTICS_WEBSITE_ID;

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html
      lang="en"
      className={`dark ${fontInter.variable} ${fontPlayfair.variable}`}
      suppressHydrationWarning
    >
      <body>
        <Providers>
          <Header />
          {children}
          <Footer />
        </Providers>
        {analyticsEndpoint && analyticsWebsiteId && (
          <Script
            defer
            src={`${analyticsEndpoint}/umami`}
            data-website-id={analyticsWebsiteId}
            strategy="afterInteractive"
          />
        )}
      </body>
    </html>
  );
}
