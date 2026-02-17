import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite' // <-- Import the plugin
import { templateCompilerOptions } from '@tresjs/core' // Import this



// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue({
      // Spread the TresJS template compiler options
      ...templateCompilerOptions,
    }),
    vueDevTools(),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  optimizeDeps: {
    include: ['html2canvas'], // Tell Vite to pre-bundle this dependency on startup
  },
})
