import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import tailwindcss from "@tailwindcss/vite";
import checker from "vite-plugin-checker";
import path from "node:path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [checker({ typescript: true }), react(), tailwindcss(), wasm()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
});
