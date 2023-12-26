import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import alias from "@rollup/plugin-alias";
import svgr from "vite-plugin-svgr";
import path from "path";

const projectRootDir = path.resolve(__dirname);

export default defineConfig(async () => ({
  plugins: [
    svgr(),
    react(),
    alias({
      entries: [
        {
          find: "@",
          replacement: path.resolve(projectRootDir, "src"),
        }
      ]
    })
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
}));
