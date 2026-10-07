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
              'Your round so far, if the contract is made: (10 × contract + contract points) × (10 + contract multiplier) × ×multipliers. Nil scores are added inside the parentheses.',
            ],
          })}
        >
          ({fmt(10 * (view.contracts![0] ?? 0))}
          {live.cp ? <b> + {fmt(live.cp)}</b> : null}) × <b>{fmt(live.mult)}</b>
          {live.x !== 1 && (
            <>
              {' '}
              × <b>{fmt(live.x)}</b>
            </>
          )}
        </div>
      )}
    </div>
  )
}
