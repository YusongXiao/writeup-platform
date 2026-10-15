import { fileURLToPath, URL } from 'node:url'
import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { defineConfig, type Plugin } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// Keep PDF fonts, CMaps and image decoders local in development and production.
function pdfAssets(): Plugin {
  const root = fileURLToPath(new URL('./node_modules/pdfjs-dist/', import.meta.url))
  const assets = new Map<string, Buffer>()
  for (const [directory, source] of Object.entries({ cmaps: 'cmaps', standard_fonts: 'standard_fonts', wasm: 'wasm', images: 'web/images' })) {
    for (const entry of readdirSync(join(root, source), { withFileTypes: true })) {
      if (entry.isFile()) assets.set(`pdfjs/${directory}/${entry.name}`, readFileSync(join(root, source, entry.name)))
    }
  }
  return {
    name: 'pdfjs-assets',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const asset = assets.get((req.url ?? '').split('?')[0]!.replace(/^\//, ''))
        if (!asset) return next()
        res.setHeader('Content-Type', req.url?.endsWith('.wasm') ? 'application/wasm' : req.url?.endsWith('.js') ? 'text/javascript' : req.url?.endsWith('.svg') ? 'image/svg+xml' : 'application/octet-stream')
        res.end(asset)
      })
    },
    generateBundle() {
      for (const [fileName, source] of assets) this.emitFile({ type: 'asset', fileName, source })
    },
  }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    pdfAssets(),
  ],
  server: {
    proxy: {
      '/api': 'http://127.0.0.1:8080',
    },
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  define: {
    __R2S_BASE__: JSON.stringify(process.env.VITE_R2S_BASE ?? 'http://localhost:8000'),
  },
})
