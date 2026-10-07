import type { CSSProperties } from 'react'
import { Icon } from '../../Icon'
import { ENGRAVINGS, SUIT_NAMES, SYNTHETIC_ICON, cardNotes, isRed, rankLabel } from '../cards'
import type { CardV } from '../types'
import styles from './PlayingCard.module.css'
import { SuitIcon } from './SuitIcon'
import { tipHandlers } from './tip'

const FACE_LETTERS: Record<number, string> = { 11: 'J', 12: 'Q', 13: 'K' }

export function PlayingCard({
  card,
  faceDown = false,
  dimmed = false,
  mini = false,
  selectable = false,
  selected = false,
  pulse,
}: {
  card?: CardV
  faceDown?: boolean
  dimmed?: boolean
  mini?: boolean
  selectable?: boolean
  selected?: boolean
  pulse?: number
}) {
  const sizeClass = mini ? styles.mini : styles.full
  if (faceDown || card === undefined) {
    return (
      <div className={`${styles.card} ${styles.back} ${sizeClass}`}>
        <div className={styles.backInner}>
          {!mini && <SuitIcon suit={3} className={styles.emblem} />}
        </div>
      </div>
    )
  }
  const label = rankLabel(card.r)
  const face = FACE_LETTERS[card.r]
  const mark = card.syn ? SYNTHETIC_ICON : ENGRAVINGS[card.eng]?.icon
  return (
    <div
      className={`${styles.card} ${styles.face} ${sizeClass} ${dimmed ? styles.dimmed : ''}`}
      data-red={isRed(card.s) || undefined}
      data-selectable={selectable || undefined}
      data-selected={selected || undefined}
      data-owned={card.own === 0 || undefined}
      data-changed={card.was ? true : undefined}
      data-pulse={pulse}
      key={pulse}
      aria-label={`${label} of ${SUIT_NAMES[card.s]}`}
      {...tipHandlers({ lines: cardNotes(card) })}
    >
      <div className={styles.corner}>
        <span className={styles.rank} data-wide={label.length > 1 || undefined}>
          {label}
        </span>
        <SuitIcon suit={card.s} className={styles.cornerSuit} />
        {mark && (
          <span
            className={styles.mark}
            style={
              { '--fill': card.syn ? 'var(--cat-enabler)' : 'var(--cat-mult)' } as CSSProperties
            }
          >
            <Icon name={mark} className={styles.markIcon} />
          </span>
        )}
      </div>
      <div className={styles.center}>
        {face ? (
          <span className={styles.faceLetter}>{face}</span>
        ) : (
          <SuitIcon suit={card.s} className={card.r === 14 ? styles.acePip : styles.pip} />
        )}
      </div>
    </div>
  )
}
