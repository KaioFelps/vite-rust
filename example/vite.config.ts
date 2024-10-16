import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from "node:path"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  base: "/",
  build: {
    rollupOptions: {
      input: ["src/www/main.tsx", "src/www/index.css"]
    },
    manifest: true,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, './src'),
      "$": path.resolve(__dirname, './'),
    }
  }
})
