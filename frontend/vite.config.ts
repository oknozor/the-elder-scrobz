import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";
import { defineConfig, loadEnv } from "vite";

export default ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };

  return defineConfig({
    plugins: [vue()],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
    server: {
      proxy: {
        "/api/v1": {
          target: process.env.VITE_API_BASE_URL || "/",
          secure: true,
          changeOrigin: true,
        },
        "/coverart": {
          target: "http://localhost:3000", // your backend
          changeOrigin: true,
        },
      },
    },
  });
};
