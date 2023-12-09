import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import alias from "@rollup/plugin-alias";
import path from "path";

const projectRootDir = path.resolve(__dirname);

export default defineConfig(async () => ({
  plugins: [
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
