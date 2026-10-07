import { exitReplay, replayTo, type Replay } from '../store'
import { Button } from './Button'
import styles from './ReplayBar.module.css'

/** Steps through a replayed session; the table shows the view after each decision. */
export function ReplayBar({ replay }: { replay: Replay }) {
  const step = replay.steps[replay.index]
  const last = replay.steps.length - 1
  const msgs = (step?.events ?? []).filter((e) => e.type !== 'offers' && e.type !== 'aiShop')
  return (
    <div className={styles.bar}>
      <div className={styles.controls}>
        <Button
          variant="ghost"
          className={styles.btn}
          onClick={() => replayTo(0)}
          aria-label="First"
        >
          ⏮
        </Button>
        <Button
          variant="ghost"
          className={styles.btn}
          onClick={() => replayTo(replay.index - 1)}
          aria-label="Back"
        >
          ◀
        </Button>
        <span className={styles.pos}>
          {replay.loading
            ? `Loading ${replay.steps.length}/${replay.total + 1}`
            : `${replay.index}/${last}`}
        </span>
        <Button
          variant="ghost"
          className={styles.btn}
          onClick={() => replayTo(replay.index + 1)}
          aria-label="Forward"
        >
          ▶
        </Button>
        <Button
          variant="ghost"
          className={styles.btn}
          onClick={() => {
            // Jump to the next trick, bid round, or shop: the next step that ends a trick or round.
            for (let i = replay.index + 1; i <= last; i++) {
              const ev = replay.steps[i].events
              if (ev.some((e) => ['trick', 'score', 'deal', 'offers'].includes(e.type)))
                return replayTo(i)
            }
            replayTo(last)
          }}
          aria-label="Next trick"
        >
          ⏭
        </Button>
        <Button variant="ghost" className={styles.btn} onClick={exitReplay}>
          Exit
        </Button>
      </div>
      <div className={styles.name}>{replay.name}</div>
      {replay.diverged && <div className={styles.diverged}>{replay.diverged}</div>}
      <div className={styles.msgs}>
        {msgs.slice(-4).map((e, i) => (
          <div key={i}>{e.msg}</div>
        ))}
      </div>
    </div>
  )
}
