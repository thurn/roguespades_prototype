import { isRed } from '../cards'
import styles from './CardText.module.css'

const SUITS = '♣♦♥♠'

/** Rules text with bracket tokens like [A], [♦], [4♣] drawn as small card glyphs. */
export function CardText({ text, className }: { text: string; className?: string }) {
  const parts = text.split(/(\[[^\]]+\])/)
  return (
    <span className={className}>
      {parts.map((part, i) =>
        part.startsWith('[') && part.endsWith(']') ? (
          <span key={i} className={styles.token}>
            {[...part.slice(1, -1)].map((ch, j) =>
              SUITS.includes(ch) ? (
                <span key={j} data-red={isRed(SUITS.indexOf(ch)) || undefined}>
                  {ch}
                </span>
              ) : (
                ch
              ),
            )}
          </span>
        ) : (
          part
        ),
      )}
    </span>
  )
}
