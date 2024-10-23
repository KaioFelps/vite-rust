import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import rustVitePlugin from "vite-rs-plugin"
import path from "node:path"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    rustVitePlugin({
      refresh: './src/www/**.*',
      entrypoints: ["src/www/main.tsx", "src/www/index.css"],
      assetsEndpoint: "/",
    })
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, './src'),
      "$": path.resolve(__dirname, './'),
    }
  },
})  
