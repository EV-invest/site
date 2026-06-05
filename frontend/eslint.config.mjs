import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({ baseDirectory: __dirname });

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),
  {
    ignores: [".next/**", "node_modules/**", "dist/**"],
  },
  {
    rules: {
      // The shadcn/ui kit and a couple of generic helpers lean on `any`;
      // surface it as a warning rather than failing the build.
      "@typescript-eslint/no-explicit-any": "warn",
      // Marketing copy legitimately contains quotes/apostrophes in JSX text.
      "react/no-unescaped-entities": "off",
    },
  },
];

export default eslintConfig;
