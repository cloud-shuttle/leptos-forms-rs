import { defineConfig } from "vite";

export default defineConfig({
  build: {
    target: "esnext",
    minify: "terser",
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
    rollupOptions: {
      output: {
        manualChunks: {
          "leptos-forms": ["./leptos-forms-rs/pkg/leptos_forms_rs.js"],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },
  optimizeDeps: {
    include: ["leptos-forms-rs"],
  },
  server: {
    fs: {
      allow: [".."],
    },
  },
});
