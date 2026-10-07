import react from '@vitejs/plugin-react'
import { execFileSync, execSync } from 'node:child_process'
import fs from 'node:fs'
import type { IncomingMessage, ServerResponse } from 'node:http'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { type Plugin, defineConfig } from 'vite'

const repoRoot = fileURLToPath(new URL('..', import.meta.url))
const sigilData = fileURLToPath(new URL('../data/sigils', import.meta.url))
const simDir = path.join(repoRoot, 'sim')
const logDir = path.join(repoRoot, 'logs')
const LOG_NAME = /^[\w.-]+\.jsonl$/

/** Builds the simulator's wasm session (sim/web) at startup and whenever a Rust file changes. */
function simWasm(): Plugin {
  const build = () =>
    execFileSync(
      'cargo',
      ['build', '-q', '-p', 'rsweb', '--target', 'wasm32-unknown-unknown', '--release'],
      { cwd: simDir, stdio: 'inherit' },
    )
  return {
    name: 'sim-wasm',
    buildStart() {
      build()
    },
    configureServer(server) {
      server.watcher.add(path.join(simDir, 'src'))
      server.watcher.add(path.join(simDir, 'web', 'src'))
      server.watcher.on('change', (file) => {
        if (!file.endsWith('.rs')) return
        try {
          build()
          server.ws.send({ type: 'full-reload' })
        } catch {
          server.config.logger.error('sim wasm build failed')
        }
      })
    },
  }
}

const send = (res: ServerResponse, status: number, body: string, type = 'application/json') => {
  res.statusCode = status
  res.setHeader('content-type', type)
  res.end(body)
}

const readBody = (req: IncomingMessage) =>
  new Promise<string>((resolve, reject) => {
    let data = ''
    req.on('data', (c) => (data += c))
    req.on('end', () => resolve(data))
    req.on('error', reject)
  })

/** Game session logs: the client appends JSONL events to logs/<file> through the dev server. */
function sessionLogs(): Plugin {
  return {
    name: 'session-logs',
    configureServer(server) {
      server.middlewares.use('/api/log', async (req, res) => {
        if (req.method !== 'POST') return send(res, 405, '{}')
        try {
          const { file, lines } = JSON.parse(await readBody(req)) as {
            file: string
            lines: unknown[]
          }
          if (!LOG_NAME.test(file)) return send(res, 400, '{"error":"bad file name"}')
          fs.mkdirSync(logDir, { recursive: true })
          fs.appendFileSync(
            path.join(logDir, file),
            lines.map((l) => JSON.stringify(l) + '\n').join(''),
          )
          send(res, 200, '{"ok":true}')
        } catch (e) {
          send(res, 500, JSON.stringify({ error: String(e) }))
        }
      })
      server.middlewares.use('/api/logs', (req, res) => {
        const name = decodeURIComponent((req.url ?? '/').slice(1))
        if (!name) {
          const files = fs.existsSync(logDir)
            ? fs
                .readdirSync(logDir)
                .filter((f) => LOG_NAME.test(f))
                .map((f) => {
                  const st = fs.statSync(path.join(logDir, f))
                  return { name: f, size: st.size, mtime: st.mtimeMs }
                })
                .sort((a, b) => b.mtime - a.mtime)
            : []
          return send(res, 200, JSON.stringify(files))
        }
        const file = path.join(logDir, name)
        if (!LOG_NAME.test(name) || !fs.existsSync(file)) return send(res, 404, '{}')
        send(res, 200, fs.readFileSync(file, 'utf8'), 'application/x-ndjson')
      })
    },
  }
}

function gitVersion(): string {
  try {
    const sha = execSync('git rev-parse --short HEAD', { cwd: repoRoot }).toString().trim()
    const dirty = execSync('git status --porcelain', { cwd: repoRoot }).toString().trim()
    return dirty ? `${sha}-dirty` : sha
  } catch {
    return 'unknown'
  }
}

export default defineConfig({
  plugins: [
    react(),
    simWasm(),
    sessionLogs(),
    {
      // The sigil data lives outside the Vite root; watch it so new files hot-reload.
      name: 'watch-sigil-data',
      configureServer(server) {
        server.watcher.add(sigilData)
      },
    },
  ],
  define: {
    __GIT_VERSION__: JSON.stringify(gitVersion()),
  },
  server: {
    port: 5173,
    strictPort: true,
    open: false,
    fs: { allow: [repoRoot] },
  },
})
