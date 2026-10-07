import { StrictMode, Suspense, lazy } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'
import './index.css'

// The game client lives at /; the sigil viewer at /sigils.
// eslint-disable-next-line react-refresh/only-export-components
const Game = lazy(() => import('./game/Game'))
const sigils = location.pathname.startsWith('/sigils')

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    {sigils ? (
      <App />
    ) : (
      <Suspense>
        <Game />
      </Suspense>
    )}
  </StrictMode>,
)
