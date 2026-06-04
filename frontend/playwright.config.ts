import { defineConfig, devices } from "@playwright/test";

// Visual-regression config. Browsers come from nixpkgs (see flake.nix:
// PLAYWRIGHT_BROWSERS_PATH), pinned to the same revision as @playwright/test,
// so screenshots render identically across every machine on this flake.
const PORT = 3000;

export default defineConfig({
  testDir: "./tests",
  // Baselines live beside the test file: tests/sections.spec.ts-snapshots/.
  snapshotPathTemplate: "{testDir}/__screenshots__/{arg}{ext}",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: 0,
  reporter: process.env.CI ? "github" : "list",

  // A pixel diff above this fraction fails the test. Small allowance absorbs
  // sub-pixel font rasterisation jitter without masking real layout changes.
  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.01,
      animations: "disabled",
    },
  },

  use: {
    baseURL: `http://localhost:${PORT}`,
    viewport: { width: 1280, height: 720 },
    deviceScaleFactor: 1,
    // Animations off + fixed colour scheme keep every run deterministic.
    colorScheme: "dark",
  },

  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1280, height: 720 },
      },
    },
  ],

  webServer: {
    command: "npm run dev -- --port " + PORT,
    url: `http://localhost:${PORT}`,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
