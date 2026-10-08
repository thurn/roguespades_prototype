import { useState } from 'react'
import rules from '../../../../data/rules.json'
import { ENGRAVINGS, cardLabel } from '../cards'
import { getSigil, sigilText } from '../sigils'
import type { Action, CardV, View } from '../types'
import { Button } from './Button'
import { CardText } from './CardText'
import { Gold } from './Coin'
import { Panel } from './Panel'
import { PlayingCard } from './PlayingCard'
import styles from './Shop.module.css'
import { SigilChip } from './SigilChip'
import { Tray } from './Tray'

const MAX_SIGILS = rules.slots
const MAX_CARDS = 8

function cardBlurb(c: CardV): string {
  if (c.syn) return `Synthetic [${cardLabel(c)}]: replaces your [${c.syn}]`
  if (c.eng) return `${ENGRAVINGS[c.eng].name}: ${ENGRAVINGS[c.eng].text}`
  return ''
}

export function Shop({
  view,
  pulses,
  readOnly,
  onAct,
}: {
  view: View
  pulses: Record<string, number>
  readOnly: boolean
  onAct: (a: Action) => void
}) {
  const shop = view.shop!
  const us = view.teams[0]
  const [sellSigil, setSellSigil] = useState<number | null>(null)
  const [sellCard, setSellCard] = useState<number | null>(null)
  const sigilFull = us.sigils.length >= MAX_SIGILS
  const cardsFull = us.cards.length >= MAX_CARDS
  const selSigil = us.sigils.find((s) => s.k === sellSigil)
  const selCard = sellCard !== null ? us.cards[sellCard] : undefined
  return (
    <Panel className={styles.shop} aria-label="Shop">
      <div className={styles.head}>
        <span className={styles.title}>
          Shop · round {view.round}
          <span className={styles.sub}>Bought cards are dealt to you every round</span>
        </span>
        <Gold amount={us.gold} className={styles.gold} />
      </div>

      <div className={styles.offers}>
        {shop.sigils.map((o, i) => {
          if (!o)
            return (
              <div key={`s${i}`} className={styles.sold}>
                Sold
              </div>
            )
          const s = getSigil(o.id)
          const short = o.price > us.gold
          return (
            <button
              key={o.id}
              className={styles.offer}
              disabled={readOnly || short || sigilFull}
              data-short={short || undefined}
              onClick={() => onAct({ t: 'buySigil', i: o.i })}
            >
              <span className={styles.offerHead}>
                <SigilChip id={o.id} />
                <span className={styles.name}>{s?.name ?? o.id}</span>
                <Gold amount={o.price} className={styles.price} />
              </span>
              <CardText className={styles.text} text={sigilText(o.id)} />
              <span className={styles.rarity} data-rarity={s?.rarity}>
                {s?.rarity}
              </span>
            </button>
          )
        })}
      </div>

      <div className={`${styles.offers} ${styles.cardRow}`}>
        {shop.cards.map((c, i) => {
          if (!c)
            return (
              <div key={`c${i}`} className={styles.sold}>
                Sold
              </div>
            )
          const short = (c.price ?? 0) > us.gold
          return (
            <button
              key={`${c.id}-${c.s}-${c.r}`}
              className={`${styles.offer} ${styles.cardOffer}`}
              disabled={readOnly || short || cardsFull}
              data-short={short || undefined}
              onClick={() => onAct({ t: 'buyCard', i: c.i! })}
            >
              <span className={styles.cardFace}>
                <PlayingCard card={c} />
              </span>
              <span className={styles.cardInfo}>
                <Gold amount={c.price ?? 0} className={styles.price} />
                {cardBlurb(c) && <CardText className={styles.text} text={cardBlurb(c)} />}
              </span>
            </button>
          )
        })}
      </div>

      <div className={styles.collection}>
        <span className={styles.count}>
          Sigils {us.sigils.length}
          <span>/{MAX_SIGILS}</span>
        </span>
        <Tray
          team={0}
          sigils={us.sigils}
          pulses={pulses}
          selected={sellSigil}
          onChip={(k) => {
            setSellCard(null)
            setSellSigil(k === sellSigil ? null : k)
          }}
          extra={(s) => `Sells for ${s.sell}`}
        />
      </div>
      <div className={styles.collection}>
        <span className={styles.count}>
          Cards {us.cards.length}
          <span>/{MAX_CARDS}</span>
        </span>
        <div className={styles.owned}>
          {us.cards.map((c, k) => (
            <button
              key={c.id}
              className={styles.ownedCard}
              data-selected={sellCard === k || undefined}
              onClick={() => {
                setSellSigil(null)
                setSellCard(k === sellCard ? null : k)
              }}
            >
              <PlayingCard card={{ ...c, own: 0 }} mini={false} />
            </button>
          ))}
        </div>
      </div>

      <div className={styles.actions}>
        {(selSigil || selCard) && !readOnly && (
          <Button
            variant="ghost"
            onClick={() => {
              if (selSigil) onAct({ t: 'sellSigil', k: selSigil.k })
              else if (sellCard !== null) onAct({ t: 'sellCard', k: sellCard })
              setSellSigil(null)
              setSellCard(null)
            }}
          >
            Sell for <Gold amount={selSigil?.sell ?? selCard?.sell ?? 0} />
          </Button>
        )}
        <Button
          variant="ghost"
          disabled={readOnly || shop.reroll > us.gold}
          onClick={() => onAct({ t: 'reroll' })}
        >
          Reroll <Gold amount={shop.reroll} className={styles.rerollCost} />
        </Button>
        <Button disabled={readOnly} onClick={() => onAct({ t: 'shopDone' })}>
          Deal
        </Button>
      </div>
    </Panel>
  )
}
