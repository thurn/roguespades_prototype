import { AnimatePresence, motion } from 'motion/react'
import { useEffect, useState } from 'react'
import { CardText } from './CardText'
import styles from './Notice.module.css'
import { SigilChip } from './SigilChip'

export interface NoticeItem {
  key: string
  sigil?: string
  text: string
  tone?: 'error'
}

/** Short private lines (Opening results, errors), shown for a few seconds. */
export function Notice({ items }: { items: NoticeItem[] }) {
  const [hidden, setHidden] = useState<Set<string>>(new Set())
  const key = items.map((i) => i.key).join('|')
  useEffect(() => {
    if (!key) return
    const id = setTimeout(() => setHidden((h) => new Set([...h, ...key.split('|')])), 6000)
    return () => clearTimeout(id)
  }, [key])
  const shown = items.filter((i) => !hidden.has(i.key))
  return (
    <div className={styles.stack}>
      <AnimatePresence>
        {shown.map((n) => (
          <motion.div
            key={n.key}
            className={styles.notice}
            data-tone={n.tone}
            initial={{ opacity: 0, y: -8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0 }}
            onClick={() => setHidden((h) => new Set([...h, n.key]))}
          >
            {n.sigil && <SigilChip id={n.sigil} size={24} />}
            <CardText text={n.text} />
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  )
}
