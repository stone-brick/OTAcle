import type { KnipConfig } from "knip";

const config: KnipConfig = {
  entry: ["src/main.ts"],
  project: ["src/**/*.ts", "src/**/*.vue"],
  ignoreDependencies: ["@tailwindcss/vite"],
};

export default config;