import type { CSSProperties } from 'react'
import { Icon } from '../../Icon'
import { categoryColor, getSigil, rarityColor } from '../sigils'
import styles from './SigilChip.module.css'

/** A sigil's glyph on a small card-face tile: glyph color is its category, rim its rarity. */
export function SigilChip({
  id,
  count,
  hidden = false,
  size,
}: {
  id?: string
  count?: number | string
  hidden?: boolean
  size?: number
}) {
  const s = id ? getSigil(id) : undefined
  const style = {
    '--fill': categoryColor(s?.category),
    '--rim': rarityColor(s?.rarity),
    ...(size ? { '--chip': `${size}px` } : {}),
  } as CSSProperties
  return (
    <span className={styles.chip} data-hidden={hidden || undefined} style={style}>
      {hidden ? (
        <span className={styles.unknown}>?</span>
      ) : (
        <Icon name={s?.icon} className={styles.icon} />
      )}
      {count !== undefined && <span className={styles.count}>{count}</span>}
    </span>
  )
}
