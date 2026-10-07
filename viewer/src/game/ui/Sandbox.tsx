import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from 'react'
import { SUIT_SYMBOLS, rankLabel } from '../cards'
import { ALL_SIGILS, sigilName } from '../sigils'
import {
  type GameState,
  dispatch,
  listLogs,
  loadReplay,
  setPaused,
  setReveal,
  setSandbox,
  startGame,
  stepOnce,
} from '../store'
import type { LogEvent } from '../types'
import { Button } from './Button'
import styles from './Sandbox.module.css'

const TEAMS = ['Us', 'Them']

export function SandboxToggle({ open }: { open: boolean }) {
  return (
    <button
      className={styles.wrench}
      data-open={open || undefined}
      aria-label="Sandbox"
      title="Sandbox"
      onClick={() => setSandbox(!open)}
    >
      <svg viewBox="0 0 24 24" aria-hidden>
        <path d="M21.7 18.6 13.4 10.3a5.5 5.5 0 0 0-7.1-7.1l3.4 3.4-2.1 2.1-3.4-3.4a5.5 5.5 0 0 0 7.1 7.1l8.3 8.3a1.5 1.5 0 0 0 2.1-2.1Z" />
      </svg>
    </button>
  )
}

export function Sandbox({ state }: { state: GameState }) {
  if (!state.sandbox) return null
  return (
    <aside className={styles.drawer} aria-label="Sandbox">
      <div className={styles.top}>
        <span className={styles.title}>Sandbox</span>
      </div>
      <GameSection state={state} />
      {!state.replay && <GrantSection state={state} />}
      <LogSection state={state} />
      <ReplaySection />
    </aside>
  )
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className={styles.section}>
      <h3 className={styles.heading}>{title}</h3>
      {children}
    </section>
  )
}

function GameSection({ state }: { state: GameState }) {
  const [seed, setSeed] = useState('')
  const [tier, setTier] = useState(state.config?.tier ?? 1)
  const [auto, setAuto] = useState(state.config?.auto ?? false)
  const v = state.view
  return (
    <Section title="Game">
      <div className={styles.meta}>
        {state.replay ? 'Replay: ' : `Seed ${state.config?.seed} · `}tier {v?.tier} · step {v?.step}
        {state.log && (
          <>
            {' · '}
            <a href={`/api/logs/${state.log.file}`} target="_blank" rel="noreferrer">
              {state.log.file}
            </a>
          </>
        )}
      </div>
      <div className={styles.row}>
        <input
          className={styles.input}
          placeholder="Seed (random)"
          value={seed}
          inputMode="numeric"
          onChange={(e) => setSeed(e.target.value.replace(/\D/g, ''))}
        />
        <select
          className={styles.select}
          value={tier}
          onChange={(e) => setTier(Number(e.target.value))}
          aria-label="AI tier"
        >
          {[0, 1, 2].map((t) => (
            <option key={t} value={t}>
              Tier {t}
            </option>
          ))}
        </select>
        <label className={styles.check}>
          <input type="checkbox" checked={auto} onChange={(e) => setAuto(e.target.checked)} /> Auto
        </label>
      </div>
      <div className={styles.row}>
        <Button
          variant="ghost"
          className={styles.small}
          onClick={() => startGame({ seed: seed ? Number(seed) : undefined, tier, auto })}
        >
          New game
        </Button>
        <Button
          variant="ghost"
          className={styles.small}
          data-on={state.paused || undefined}
          onClick={() => setPaused(!state.paused)}
        >
          {state.paused ? 'Resume AI' : 'Pause AI'}
        </Button>
        {state.paused && (
          <Button variant="ghost" className={styles.small} onClick={stepOnce}>
            Step
          </Button>
        )}
      </div>
      <div className={styles.row}>
        <label className={styles.check}>
          <input
            type="checkbox"
            checked={state.reveal}
            onChange={(e) => setReveal(e.target.checked)}
          />{' '}
          Show all hands and hidden sigils
        </label>
      </div>
      {!state.replay && (
        <div className={styles.row}>
          <span className={styles.label}>AI tier now</span>
          {[0, 1, 2].map((t) => (
            <Button
              key={t}
              variant="ghost"
              className={styles.small}
              data-on={v?.tier === t || undefined}
              onClick={() => dispatch({ t: 'tier', tier: t })}
            >
              {t}
            </Button>
          ))}
        </div>
      )}
    </Section>
  )
}

function GrantSection({ state }: { state: GameState }) {
  const [team, setTeam] = useState(0)
  const [query, setQuery] = useState('')
  const [suit, setSuit] = useState(3)
  const [rank, setRank] = useState(14)
  const [eng, setEng] = useState(0)
  const v = state.view
  const inShop = v?.phase === 'shop'
  const match = useMemo(() => {
    const q = query.trim().toLowerCase()
    if (!q) return []
    return ALL_SIGILS.filter((s) =>
      [s.id, s.name ?? '', s.text ?? ''].some((x) => x.toLowerCase().includes(q)),
    ).slice(0, 8)
  }, [query])
  if (!v) return null
  const owned = state.reveal || team === 0 ? v.teams[team].sigils : v.teams[team].sigils
  return (
    <Section title="Grant">
      <div className={styles.row}>
        {TEAMS.map((t, i) => (
          <Button
            key={t}
            variant="ghost"
            className={styles.small}
            data-on={team === i || undefined}
            onClick={() => setTeam(i)}
          >
            {t}
          </Button>
        ))}
        <span className={styles.label}>gold</span>
        {[100, 500, -100].map((g) => (
          <Button
            key={g}
            variant="ghost"
            className={styles.small}
            onClick={() => dispatch({ t: 'gold', team, amount: g })}
          >
            {g > 0 ? `+${g}` : g}
          </Button>
        ))}
      </div>
      {!inShop && <div className={styles.meta}>Sigils and cards can be granted during a shop.</div>}
      <input
        className={styles.input}
        placeholder="Find a sigil (name, id, or text)"
        value={query}
        disabled={!inShop}
        onChange={(e) => setQuery(e.target.value)}
      />
      {match.map((s) => (
        <button
          key={s.id}
          className={styles.pick}
          onClick={() => {
            dispatch({ t: 'give', team, sigil: s.id })
            setQuery('')
          }}
        >
          <b>{s.name ?? s.id}</b> <span>{s.status}</span>
          <div>{s.text}</div>
        </button>
      ))}
      <div className={styles.owned}>
        {owned.map((s) => (
          <span key={s.k} className={styles.ownedRow}>
            {sigilName(s.id)}
            <button
              className={styles.x}
              disabled={!inShop}
              aria-label="Remove"
              onClick={() => dispatch({ t: 'take', team, k: s.k })}
            >
              ×
            </button>
          </span>
        ))}
      </div>
      <div className={styles.row}>
        <select
          className={styles.select}
          value={rank}
          onChange={(e) => setRank(Number(e.target.value))}
          aria-label="Rank"
        >
          {Array.from({ length: 13 }, (_, i) => 14 - i).map((r) => (
            <option key={r} value={r}>
              {rankLabel(r)}
            </option>
          ))}
        </select>
        <select
          className={styles.select}
          value={suit}
          onChange={(e) => setSuit(Number(e.target.value))}
          aria-label="Suit"
        >
          {[3, 2, 1, 0].map((s) => (
            <option key={s} value={s}>
              {SUIT_SYMBOLS[s]}
            </option>
          ))}
        </select>
        <select
          className={styles.select}
          value={eng}
          onChange={(e) => setEng(Number(e.target.value))}
          aria-label="Engraving"
        >
          {['Plain', 'Bonus', 'Herald', 'Multiplier', 'Synthetic'].map((n, i) => (
            <option key={n} value={i}>
              {n}
            </option>
          ))}
        </select>
        <Button
          variant="ghost"
          className={styles.small}
          disabled={!inShop}
          onClick={() => dispatch({ t: 'giveCard', team, suit, rank, eng })}
        >
          Give card
        </Button>
      </div>
    </Section>
  )
}

function useLogEvents(state: GameState): LogEvent[] {
  const log = state.log
  const subscribe = useMemo(() => (l: () => void) => log?.subscribe(l) ?? (() => {}), [log])
  const count = useSyncExternalStore(subscribe, () => log?.events.length ?? 0)
  return useMemo(() => {
    void count
    if (state.replay) {
      return state.replay.steps.slice(0, state.replay.index + 1).flatMap((s) => s.events)
    }
    return log?.events ?? []
  }, [log, count, state.replay])
}

const QUIET = new Set(['offers', 'aiShop'])

function LogSection({ state }: { state: GameState }) {
  const events = useLogEvents(state)
  const [filter, setFilter] = useState('')
  const [all, setAll] = useState(false)
  const box = useRef<HTMLDivElement>(null)
  const shown = useMemo(() => {
    const f = filter.trim().toLowerCase()
    return events
      .filter((e) => all || !QUIET.has(e.type))
      .filter((e) => !f || e.msg.toLowerCase().includes(f) || e.type.includes(f))
      .slice(-300)
  }, [events, filter, all])
  useEffect(() => {
    box.current?.scrollTo({ top: box.current.scrollHeight })
  }, [shown.length])
  return (
    <Section title="Log">
      <div className={styles.row}>
        <input
          className={styles.input}
          placeholder="Filter"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
        <label className={styles.check}>
          <input type="checkbox" checked={all} onChange={(e) => setAll(e.target.checked)} /> Shop
          detail
        </label>
        {state.log && !state.replay && (
          <Button variant="ghost" className={styles.small} onClick={() => state.log?.download()}>
            Download
          </Button>
        )}
      </div>
      <div className={styles.log} ref={box}>
        {shown.map((e, i) => (
          <div key={i} className={styles.line} data-type={e.type}>
            <span className={styles.step}>{e.step ?? ''}</span> {e.msg}
          </div>
        ))}
      </div>
    </Section>
  )
}

function ReplaySection() {
  const [logs, setLogs] = useState<{ name: string; size: number; mtime: number }[] | null>(null)
  return (
    <Section title="Replay">
      <div className={styles.row}>
        <Button variant="ghost" className={styles.small} onClick={() => listLogs().then(setLogs)}>
          List logs
        </Button>
        <label className={styles.file}>
          Open file…
          <input
            type="file"
            accept=".jsonl"
            onChange={async (e) => {
              const f = e.target.files?.[0]
              if (f) loadReplay(f.name, await f.text())
            }}
          />
        </label>
      </div>
      {logs?.length === 0 && <div className={styles.meta}>No logs yet.</div>}
      {logs?.slice(0, 30).map((l) => (
        <button key={l.name} className={styles.pick} onClick={() => loadReplay(l.name)}>
          <b>{l.name}</b> <span>{Math.round(l.size / 1024)} KB</span>
        </button>
      ))}
    </Section>
  )
}
