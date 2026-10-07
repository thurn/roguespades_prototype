import { motion } from 'motion/react'
import { type CSSProperties, type RefObject, useEffect, useState } from 'react'
import { sortHand } from '../cards'
import type { CardV } from '../types'
import styles from './Hand.module.css'
import { PlayingCard } from './PlayingCard'

const DRAG_PLAY_DISTANCE = 90

export function Hand({
  cards,
  legal,
  active,
  picking,
  picked,
  pulses,
  dropZone,
  onPlay,
  onPick,
  onDragChange,
}: {
  cards: CardV[]
  legal: Set<number>
  active: boolean
  /** Choosing cards (an Opening swap) instead of playing one. */
  picking: boolean
  picked: Set<number>
  pulses: Record<string, number>
  dropZone: RefObject<HTMLElement | null>
  onPlay: (id: number) => void
  onPick: (id: number) => void
  onDragChange: (dragging: boolean) => void
}) {
  // Stagger the deal animation, then let cards respond immediately.
  const [settled, setSettled] = useState(false)
  useEffect(() => {
    const id = setTimeout(() => setSettled(true), 1200)
    return () => clearTimeout(id)
  }, [])

  const sorted = sortHand(cards)
  const mid = (sorted.length - 1) / 2

  const inDropZone = (x: number, y: number) => {
    const r = dropZone.current?.getBoundingClientRect()
    return !!r && x >= r.left && x <= r.right && y >= r.top && y <= r.bottom
  }

  return (
    <div className={styles.hand} style={{ '--n': sorted.length } as CSSProperties}>
      {sorted.map((card, i) => {
        const offset = i - mid
        const playable = !picking && active && legal.has(card.id)
        const lift = playable || picking
        return (
          <motion.div
            key={card.id}
            layout="position"
            className={styles.slot}
            data-playable={lift || undefined}
            style={{ zIndex: i, '--o': offset } as CSSProperties}
            initial={{ y: 220, opacity: 0 }}
            animate={{
              y: picked.has(card.id) ? -26 : 0,
              opacity: 1,
              transition: {
                type: 'spring',
                stiffness: 300,
                damping: 30,
                delay: settled ? 0 : i * 0.045,
              },
            }}
            whileHover={lift ? { y: -22, zIndex: 50, transition: { duration: 0.15 } } : undefined}
            whileTap={lift ? { y: -12, scale: 0.98 } : undefined}
            drag={playable}
            dragSnapToOrigin
            dragElastic={0.9}
            dragMomentum={false}
            whileDrag={{ scale: 1.08, zIndex: 100 }}
            onDragStart={() => onDragChange(true)}
            onDragEnd={(_, info) => {
              onDragChange(false)
              if (inDropZone(info.point.x, info.point.y) || info.offset.y < -DRAG_PLAY_DISTANCE) {
                onPlay(card.id)
              }
            }}
            onTap={() => (picking ? onPick(card.id) : playable && onPlay(card.id))}
          >
            <div className={styles.arc}>
              <PlayingCard
                card={card}
                selectable={picking}
                selected={picked.has(card.id)}
                dimmed={active && !playable && !picking}
                pulse={pulses[`c:${card.id}`]}
              />
            </div>
          </motion.div>
        )
      })}
    </div>
  )
}
