import react from '@vitejs/plugin-react'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'

const repoRoot = fileURLToPath(new URL('..', import.meta.url))
const sigilData = fileURLToPath(new URL('../data/sigils', import.meta.url))

export default defineConfig({
  plugins: [
    react(),
    {
      // The sigil data lives outside the Vite root; watch it so new files hot-reload.
      name: 'watch-sigil-data',
      configureServer(server) {
        server.watcher.add(sigilData)
      },
    },
  ],
  server: {
    port: 5173,
    strictPort: true,
    open: false,
    fs: { allow: [repoRoot] },
  },
})
