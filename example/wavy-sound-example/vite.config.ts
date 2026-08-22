import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [wasm()],
  build: {
    target: 'esnext'
  },
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ['..', '../../pkg']
    }
  },
  optimizeDeps: {
    exclude: ['wavy-sounds']
  }
})