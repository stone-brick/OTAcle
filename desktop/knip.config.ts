import type { KnipConfig } from "knip";

const config: KnipConfig = {
  project: ["src/**/*.ts", "src/**/*.vue"],
  ignoreDependencies: [
    // Tauri plugins - 前端可能不会直接 import，但运行时需要
    /^@tauri-apps\//,
    "@tailwindcss/vite",
    "tailwindcss",
  ],
};

export default config;