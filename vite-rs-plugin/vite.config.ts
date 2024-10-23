import { resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  root: './',
  build: {
    lib: {
      name: 'RustVitePlugin',
      entry: [resolve(__dirname, 'src/index.ts')],
      formats: ['es', 'cjs'],
    },
    outDir: 'dist',
    target: 'esnext',
    rollupOptions: {
      external: ['node:path', 'path', 'fs', 'fsevents', 'vite'],
      input: ['./src/index.ts', './src/inertia.ts'],
    },
  },
})
