import { motion } from 'motion/react'
import type { SigilV } from '../types'
import { SigilChip } from './SigilChip'
import { tipHandlers } from './tip'
import styles from './Tray.module.css'

/** A team's sigils as chips; each flashes when it fires. */
export function Tray({
  team,
  sigils,
  hidden = 0,
  pulses,
  selected,
  onChip,
  extra,
}: {
  team: number
  sigils: SigilV[]
  hidden?: number
  pulses: Record<string, number>
  selected?: number | null
  onChip?: (k: number) => void
  extra?: (s: SigilV) => string | undefined
}) {
  if (sigils.length === 0 && hidden === 0) return null
  return (
    <div className={styles.tray}>
      {sigils.map((s) => {
        const pulse = pulses[`s:${team}:${s.k}`]
        const tip = tipHandlers({
          sigils: [
            {
              id: s.id,
              grow: s.grow,
              extra: [
                `Fired ${s.fires} time${s.fires === 1 ? '' : 's'} this run${s.rf ? `, ${s.rf} this round` : ''}`,
                team === 1 && !s.revealed ? 'Hidden from you (sandbox reveal)' : '',
                extra?.(s) ?? '',
              ]
                .filter(Boolean)
                .join(' · '),
            },
          ],
        })
        const inner = (
          <motion.span
            key={pulse ?? 0}
            className={styles.pulse}
            initial={pulse ? { scale: 1.45, rotate: -8 } : false}
            animate={{ scale: 1, rotate: 0 }}
            transition={{ duration: 0.45, ease: [0.22, 1, 0.36, 1] }}
          >
            <SigilChip id={s.id} count={s.grow ? s.grow : undefined} />
          </motion.span>
        )
        return onChip ? (
          <button
            key={s.k}
            className={styles.button}
            data-selected={selected === s.k || undefined}
            onClick={() => onChip(s.k)}
            {...tip}
          >
            {inner}
          </button>
        ) : (
          <span
            key={s.k}
            className={styles.item}
            data-unrevealed={(team === 1 && !s.revealed) || undefined}
            {...tip}
          >
            {inner}
          </span>
        )
      })}
      {Array.from({ length: hidden }, (_, i) => (
        <span
          key={`h${i}`}
          className={styles.item}
          {...tipHandlers({
            lines: [
              'A hidden sigil: revealed the first time it changes a score, a legal play, or a trick’s winner',
            ],
          })}
        >
          <SigilChip hidden />
        </span>
      ))}
    </div>
  )
}
