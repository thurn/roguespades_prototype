import { SEAT_NAMES, cardLabel } from '../cards'
import type { CardV, Seat } from '../types'
import { CardText } from './CardText'
import styles from './Nameplate.module.css'
import { tipHandlers } from './tip'

export function Nameplate({
  seat,
  bid,
  won,
  bidding,
  active,
  thinking,
  dealer,
  ownsCards,
  known,
}: {
  seat: Seat
  /** -1 before bidding, 0 nil. */
  bid: number
  won: number
  bidding: boolean
  active: boolean
  thinking: boolean
  dealer: boolean
  ownsCards: boolean
  known?: CardV[]
}) {
  const name = SEAT_NAMES[seat]
  return (
    <div className={styles.wrap}>
      <div
        className={styles.plate}
        data-active={active || undefined}
        data-team={seat % 2 === 0 ? 'us' : 'them'}
        {...tipHandlers({
          lines: [
            seat === 0
              ? 'You (South)'
              : seat === 2
                ? 'Sage (North), your partner'
                : `${name} (${seat === 1 ? 'West' : 'East'}), opponent`,
            ownsCards ? 'Receives this team’s owned cards every round' : '',
          ].filter(Boolean),
        })}
      >
        {dealer && (
          <span className={styles.dealer} title="Dealer">
            D
          </span>
        )}
        <div className={styles.avatar}>{name[0]}</div>
        <span className={styles.name}>{name}</span>
        <Stat bid={bid} won={won} bidding={bidding} thinking={thinking} />
      </div>
      {known && known.length > 0 && (
        <div
          className={styles.known}
          {...tipHandlers({ lines: ['Owned cards your partner holds'] })}
        >
          {known.map((c) => (
            <CardText key={c.id} text={`[${cardLabel(c)}]`} />
          ))}
        </div>
      )}
    </div>
  )
}

function Stat({
  bid,
  won,
  bidding,
  thinking,
}: {
  bid: number
  won: number
  bidding: boolean
  thinking: boolean
}) {
  if (bid < 0) {
    return thinking ? (
      <span className={styles.stat} aria-label="Thinking">
        <span className={styles.dots}>
          <i />
          <i />
          <i />
        </span>
      </span>
    ) : null
  }
  if (bid === 0) {
    return (
      <span className={styles.stat} data-nil data-broken={won > 0 || undefined}>
        Nil
        {won > 0 && <b>{won}</b>}
      </span>
    )
  }
  if (bidding) {
    return (
      <span className={styles.stat} title="Bid">
        <b>{bid}</b>
      </span>
    )
  }
  return (
    <span className={styles.stat} title="Tricks won / bid" data-made={won >= bid || undefined}>
      <b>{won}</b>
      <span className={styles.of}>/{bid}</span>
    </span>
  )
}
