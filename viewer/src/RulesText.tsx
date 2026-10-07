import styles from './App.module.css'

const RED = new Set(['♦', '♥'])

/** Renders generated rules text, turning bracket tokens like [A], [♦], [4♣] into card glyphs. */
export function RulesText({ text, className }: { text: string | undefined; className?: string }) {
  if (!text) return <p className={`${className ?? ''} ${styles.missing}`}>No rules text yet.</p>
  const parts = text.split(/(\[[^\]]+\])/)
  return (
    <p className={className}>
      {parts.map((part, i) =>
        part.startsWith('[') && part.endsWith(']') ? (
          <Token key={i} token={part.slice(1, -1)} />
        ) : (
          part
        ),
      )}
    </p>
  )
}

function Token({ token }: { token: string }) {
  return (
    <span className={styles.token}>
      {[...token].map((ch, i) =>
        '♠♥♦♣'.includes(ch) ? (
          <span key={i} className={RED.has(ch) ? styles.suitRed : styles.suitBlack}>
            {ch}
          </span>
        ) : (
          ch
        ),
      )}
    </span>
  )
}
