import { AnimatePresence, motion } from 'motion/react'
import { SEAT_VECTORS } from '../cards'
import type { Seat, TrickPlay } from '../types'
import { PlayingCard } from './PlayingCard'
import styles from './Trick.module.css'

/** Resting offsets as percentages of the card's own size, plus a slight natural tilt. */
const REST: Record<Seat, { x: string; y: string; rotate: number }> = {
  0: { x: '0%', y: '53%', rotate: -3 },
  1: { x: '-108%', y: '0%', rotate: 4 },
  2: { x: '0%', y: '-53%', rotate: 2 },
  3: { x: '108%', y: '0%', rotate: -5 },
}

const offscreen = (seat: Seat, scale: number) => ({
  x: `${SEAT_VECTORS[seat].x * 420 * scale}%`,
  y: `${SEAT_VECTORS[seat].y * 260 * scale}%`,
})

const variants = {
  exit: (winner: Seat | null) => ({
    ...(winner === null ? {} : offscreen(winner, 1)),
    scale: 0.6,
    opacity: 0,
    rotate: 0,
    transition: { duration: 0.45, ease: [0.55, 0, 0.8, 0.4] as const },
  }),
}

export function Trick({
  plays,
  winner,
  exitTo,
  pulses,
}: {
  plays: TrickPlay[]
  /** The seat that won a finished trick still on the table. */
  winner: Seat | null
  /** Where collected cards fly: the last trick's winner. */
  exitTo: Seat | null
  pulses: Record<string, number>
}) {
  return (
    <AnimatePresence custom={exitTo}>
      {plays.map((p, i) => (
        <motion.div
          key={p.card.id}
          className={styles.slot}
          style={{ zIndex: i }}
          custom={exitTo}
          variants={variants}
          initial={{ ...offscreen(p.seat, 0.9), opacity: 0, rotate: 0, scale: 0.9 }}
          animate={{ ...REST[p.seat], opacity: 1, scale: 1 }}
          exit="exit"
          transition={{ type: 'spring', stiffness: 260, damping: 26 }}
        >
          <div className={styles.glow} data-on={winner === p.seat || undefined}>
            <PlayingCard card={p.card} pulse={pulses[`c:${p.card.id}`]} />
          </div>
        </motion.div>
      ))}
    </AnimatePresence>
  )
}
