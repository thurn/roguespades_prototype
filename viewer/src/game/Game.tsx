import { AnimatePresence, motion } from 'motion/react'
import { useEffect, useMemo, useRef, useState } from 'react'
import { cardLabel, sortHand } from './cards'
import './game.css'
import styles from './Game.module.css'
import { type GameState, PARAMS, dispatch, loadReplay, startGame, useGame } from './store'
import { HUMAN, type Seat, type View } from './types'
import { CardText } from './ui/CardText'
import { Hand } from './ui/Hand'
import { MiniHand } from './ui/MiniHand'
import { Nameplate } from './ui/Nameplate'
import { Notice, type NoticeItem } from './ui/Notice'
import { BidPrompt, LeadPrompt, RoundSummary, SwapPrompt } from './ui/Prompts'
import { ReplayBar } from './ui/ReplayBar'
import { Sandbox, SandboxToggle } from './ui/Sandbox'
import { ScoreBoard } from './ui/ScoreBoard'
import { Shop } from './ui/Shop'
import { SuitIcon } from './ui/SuitIcon'
import { TooltipLayer } from './ui/Tooltip'
import { Trick } from './ui/Trick'

const fade = {
  initial: { opacity: 0, y: 16, scale: 0.97 },
  animate: { opacity: 1, y: 0, scale: 1 },
  exit: { opacity: 0, y: 8, scale: 0.98 },
  transition: { duration: 0.25, ease: [0.22, 1, 0.36, 1] as const },
}

let started = false

export default function Game() {
  const state = useGame()
  useEffect(() => {
    document.title = 'Rogue Spades'
    if (started) return
    started = true
    if (PARAMS.replay) loadReplay(PARAMS.replay)
    else startGame()
  }, [])
  const view = state.replay ? state.replay.steps[state.replay.index]?.view : state.view
  return (
    <div className="game">
      {view ? (
        <Table key={state.generation} view={view} state={state} />
      ) : (
        <div className={styles.loading}>
          {state.fatal ?? (state.replay ? 'Loading replay…' : '')}
        </div>
      )}
      {state.replay && <ReplayBar replay={state.replay} />}
      <SandboxToggle open={state.sandbox} />
      <Sandbox state={state} />
      <TooltipLayer />
    </div>
  )
}

function Table({ view, state }: { view: View; state: GameState }) {
  const tableRef = useRef<HTMLElement>(null)
  const [dragging, setDragging] = useState(false)
  const [picked, setPicked] = useState<Set<number>>(new Set())
  const interactive = !state.replay
  const held = interactive ? state.held : view.fresh ? view.lastTrick : null
  const pending = view.pending
  const myTurn = interactive && !!pending && !pending.ai && !held && !state.busy && !state.fatal
  const humanPlay = myTurn && pending!.kind === 'play'
  const legal = useMemo(() => new Set(humanPlay ? (view.legal ?? []) : []), [humanPlay, view.legal])
  const picking = myTurn && pending!.kind === 'swap'

  // A new decision clears any half-made swap pick.
  const step = view.step
  const [pickStep, setPickStep] = useState(step)
  if (pickStep !== step) {
    setPickStep(step)
    setPicked(new Set())
  }

  const plays = held ? held.plays : (view.trick ?? [])
  const exitTo = held?.winner ?? view.lastTrick?.winner ?? null
  const handCards = view.hands?.[HUMAN].cards ?? []

  const plate = (seat: Seat) => {
    const team = view.teams[seat % 2]
    const active =
      !!pending && pending.seat === seat && ['bid', 'play', 'lead', 'swap'].includes(pending.kind)
    return (
      <Nameplate
        seat={seat}
        bid={view.bids?.[seat] ?? -1}
        won={view.won?.[seat] ?? 0}
        bidding={view.phase === 'bidding' || view.phase === 'opening'}
        active={active && !held}
        thinking={active && pending!.ai && !held}
        dealer={view.dealer === seat}
        ownsCards={team.cardSeat === seat && team.cardCount > 0}
        known={seat === 2 && !fullyShown(view, seat) ? view.hands?.[2].cards : undefined}
      />
    )
  }

  const others = (seat: Seat) => {
    const h = view.hands?.[seat]
    if (h && fullyShown(view, seat)) {
      return (
        <div className={styles.revealed}>
          {sortHand(h.cards).map((c) => (
            <CardText key={c.id} text={`[${cardLabel(c)}]`} />
          ))}
        </div>
      )
    }
    return <MiniHand count={h?.count ?? 0} />
  }

  const notices: NoticeItem[] = [
    ...(view.phase !== 'shop'
      ? view.notes.map((n, i) => ({ key: `r${view.round}-${i}`, sigil: n.sigil, text: n.text }))
      : []),
    ...(state.toast && interactive
      ? [{ key: `t${state.toast.id}`, text: state.toast.text, tone: 'error' as const }]
      : []),
  ]

  const showSummary =
    (view.phase === 'roundOver' || view.phase === 'gameOver') && !!view.result && !held
  const showShop = view.phase === 'shop' && !!view.shop
  const overlay = showSummary ? 'summary' : showShop ? 'shop' : null
  const over = view.phase === 'gameOver'
  const partnerBid = view.bids?.[2] ?? -1

  return (
    <div className={styles.app} data-sandbox={state.sandbox || undefined}>
      <div className={styles.score}>
        <ScoreBoard view={view} pulses={state.pulses} />
      </div>

      <section className={styles.north}>
        {others(2)}
        {plate(2)}
      </section>
      <section className={styles.west}>
        {plate(1)}
        {others(1)}
      </section>
      <section className={styles.east}>
        {plate(3)}
        {others(3)}
      </section>

      <main
        ref={tableRef}
        className={styles.table}
        data-drop={dragging || undefined}
        data-your-turn={humanPlay || undefined}
      >
        <div className={styles.felt} />
        <SuitIcon
          suit={3}
          className={styles.spadeMark}
          data-broken={view.broken || undefined}
          aria-label={view.broken ? 'Spades broken' : 'Spades not broken'}
        />
        <Trick plays={plays} winner={held?.winner ?? null} exitTo={exitTo} pulses={state.pulses} />
        <AnimatePresence>
          {myTurn && pending!.kind === 'bid' && (
            <motion.div key="bid" className={styles.bidDock} {...fade}>
              <BidPrompt partnerBid={partnerBid} onBid={(bid) => dispatch({ t: 'bid', bid })} />
            </motion.div>
          )}
          {picking && (
            <motion.div key="swap" className={styles.promptDock} {...fade}>
              <SwapPrompt
                pending={pending!}
                sigil={pending!.sigil ?? null}
                picked={picked.size}
                onConfirm={() => dispatch({ t: 'swap', cards: [...picked] })}
              />
            </motion.div>
          )}
          {myTurn && pending!.kind === 'lead' && (
            <motion.div key="lead" className={styles.promptDock} {...fade}>
              <LeadPrompt onChoose={(pass) => dispatch({ t: 'lead', pass })} />
            </motion.div>
          )}
        </AnimatePresence>
        {view.auto && interactive && <div className={styles.spectating}>Autoplay</div>}
      </main>

      <section className={styles.south}>
        {plate(HUMAN)}
        <Hand
          key={view.round}
          cards={handCards}
          legal={legal}
          active={humanPlay}
          picking={picking}
          picked={picked}
          pulses={state.pulses}
          dropZone={tableRef}
          onPlay={(card) => dispatch({ t: 'play', card })}
          onPick={(id) => {
            const next = new Set(picked)
            if (next.has(id)) next.delete(id)
            else if (next.size < (pending?.n ?? 0)) next.add(id)
            setPicked(next)
          }}
          onDragChange={setDragging}
        />
      </section>

      <Notice items={notices} />

      <AnimatePresence>
        {overlay && (
          <motion.div
            key={overlay}
            className={styles.scrim}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            <motion.div {...fade}>
              {overlay === 'summary' && view.result && (
                <RoundSummary
                  result={view.result.teams}
                  round={view.result.round}
                  over={over}
                  winner={view.winner}
                  canContinue={interactive && (over || (myTurn && pending!.kind === 'next'))}
                  onContinue={() => (over ? startGame() : dispatch({ t: 'next' }))}
                />
              )}
              {overlay === 'shop' && (
                <Shop
                  view={view}
                  pulses={state.pulses}
                  readOnly={!myTurn}
                  onAct={(a) => dispatch(a)}
                />
              )}
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {interactive && (
        <button
          className={styles.restart}
          aria-label="New game"
          title="New game"
          onClick={() => confirm('Abandon this game and deal a new one?') && startGame()}
        >
          <svg viewBox="0 0 24 24" aria-hidden>
            <path d="M12 5V2L7 6l5 4V7a6 6 0 1 1-6 6H4a8 8 0 1 0 8-8Z" />
          </svg>
        </button>
      )}
    </div>
  )
}

/** Whether a seat's whole hand is visible (the sandbox's show-all, or a replay). */
function fullyShown(view: View, seat: Seat) {
  const h = view.hands?.[seat]
  return !!h && h.count > 0 && h.cards.length === h.count
}
