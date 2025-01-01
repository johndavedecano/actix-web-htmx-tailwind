import path from "path";
import tailwindcss from "tailwindcss";
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    hmr: true
  },
  build: {
    outDir: "./static/",
    rollupOptions: {
      input: {
        app: path.resolve(__dirname, "./assets/app.ts"),
      },
      output: {
        entryFileNames: "[name].js",
        assetFileNames: "[name].[ext]",
      }
    },
    
  },
  css: {
    postcss: {
      plugins: [tailwindcss()],
    },
  },
});
