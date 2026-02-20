import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import { templateCompilerOptions } from '@tresjs/core'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue({
      ...templateCompilerOptions,
    }),
    vueDevTools(),
    tailwindcss(),
    wasm(),
    topLevelAwait(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  optimizeDeps: {
    include: ['html2canvas'],
    exclude: ['game'],  // Don't pre-bundle the WASM package
  },
})
