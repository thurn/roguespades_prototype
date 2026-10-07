import { useSyncExternalStore } from 'react'
import { getSigil, sigilText } from '../sigils'
import { CardText } from './CardText'
import { SigilChip } from './SigilChip'
import { getTip, hideTip, subscribeTip } from './tip'
import styles from './Tooltip.module.css'

export function TooltipLayer() {
  const t = useSyncExternalStore(subscribeTip, getTip)
  if (!t) return null
  const width = Math.min(t.sigils?.length ? 340 : 300, window.innerWidth - 16)
  const left = Math.max(8, Math.min(window.innerWidth - width - 8, t.x - width / 2))
  const below = t.top < 240
  return (
    <div
      className={styles.tip}
      role="tooltip"
      style={{
        left,
        width,
        ...(below ? { top: t.bottom + 8 } : { bottom: window.innerHeight - t.top + 8 }),
      }}
      onPointerDown={hideTip}
    >
      {t.sigils?.map(({ id, grow, extra }) => {
        const s = getSigil(id)
        return (
          <div key={id} className={styles.entry}>
            <div className={styles.head}>
              <SigilChip id={id} />
              <span className={styles.name}>{s?.name ?? id}</span>
              <span className={styles.rarity} data-rarity={s?.rarity}>
                {s?.rarity}
              </span>
            </div>
            <CardText className={styles.text} text={sigilText(id, grow)} />
            {extra && <span className={styles.extra}>{extra}</span>}
          </div>
        )
      })}
      {t.lines?.map((l, i) => (
        <CardText key={i} className={styles.text} text={l} />
      ))}
    </div>
  )
}
