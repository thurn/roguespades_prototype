import { fmt, num } from '../cards'
import type { View } from '../types'
import { Gold } from './Coin'
import styles from './ScoreBoard.module.css'
import { tipHandlers } from './tip'
import { Tray } from './Tray'

const TEAMS = [
  { label: 'Us', team: 'us' },
  { label: 'Them', team: 'them' },
] as const

/** Round, both teams' scores, gold, contracts, sigils, and your running tally. */
export function ScoreBoard({ view, pulses }: { view: View; pulses: Record<string, number> }) {
  const inPlay = view.phase === 'playing' && view.contracts
  const live = view.live
  return (
    <div className={styles.board}>
      <div className={styles.round} title="Round">
        Round {view.round}
        <span>/{view.rounds}</span>
      </div>
      {TEAMS.map(({ label, team }, t) => {
        const tm = view.teams[t]
        const contract = view.contracts?.[t] ?? 0
        return (
          <div key={team} className={styles.team} data-team={team}>
            <div className={styles.row}>
              <span className={styles.swatch} />
              <span className={styles.label}>{label}</span>
              <span className={styles.score}>{num(Math.round(tm.score))}</span>
              <span
                className={styles.bags}
                data-warn={tm.bags >= 7 || undefined}
                {...tipHandlers({
                  lines: [`${tm.bags} bags. Every 10 bags cost 1,000 points.`],
                })}
              >
                {tm.bags}b
              </span>
              <Gold amount={tm.gold} className={styles.gold} />
              {inPlay && (
                <span
                  className={styles.contract}
                  data-made={(view.contractTricks![t] >= contract && contract > 0) || undefined}
                  {...tipHandlers({ lines: ['Tricks toward the contract / contract'] })}
                >
                  {contract ? `${view.contractTricks![t]}/${contract}` : 'nil'}
                </span>
              )}
            </div>
            <Tray team={t} sigils={tm.sigils} hidden={tm.hidden} pulses={pulses} />
          </div>
        )
      })}
      {inPlay && live && (
        <div
          className={styles.live}
          {...tipHandlers({
            lines: [
              'Your round so far, if the contract is made: (base + contract points) × (starting multiplier + contract multiplier) × ×multipliers. Nil scores are added inside the parentheses.',
            ],
          })}
        >
          ({fmt(live.base)}
          {live.cp ? <b> + {fmt(live.cp)}</b> : null}) × <b>{fmt(live.mult)}</b>
          {live.x !== 1 && (
            <>
              {' '}
              × <b>{fmt(live.x)}</b>
            </>
          )}
        </div>
      )}
      {inPlay && view.deck && (view.deck.extra.length > 0 || view.deck.missing.length > 0) && (
        <div
          className={styles.live}
          {...tipHandlers({
            lines: [
              "This round's deck differs from a standard deck after Opening changes. Everyone knows the deck, not who holds each card.",
            ],
          })}
        >
          Deck: {view.deck.extra.length > 0 && <b>+{view.deck.extra.join(' ')}</b>}
          {view.deck.missing.length > 0 && <> −{view.deck.missing.join(' ')}</>}
        </div>
      )}
    </div>
  )
}
