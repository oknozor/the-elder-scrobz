import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";

const host = process.env.TAURI_DEV_HOST;

export default ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };

  return defineConfig({
    clearScreen: false,
    plugins: [vue()],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
    server: {
      strictPort: true,
      host: host || false,
      proxy: {
        "/api/v1": {
          target: process.env.VITE_API_BASE_URL || "/",
          secure: true,
          changeOrigin: true,
        },
      },
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
    envPrefix: ["VITE_", "TAURI_"],
    build: {
      target:
        process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
      minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
      sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
  });
};
