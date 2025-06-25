// vite.config.ts
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "node:path";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  build: {
    outDir: "build",
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "@components": path.resolve(__dirname, "src/cubic/components"),
      "@layout": path.resolve(__dirname, "src/cubic/layout"),
      "@assets": path.resolve(__dirname, "src/assets"),
      "@stores": path.resolve(__dirname, "src/cubic/stores"),
      "@css": path.resolve(__dirname, "src/css"),
      "@views": path.resolve(__dirname, "src/cubic/views"),
    },
  },
});
