import { Fragment } from 'react'
import { SEAT_NAMES, fmt, num, signed } from '../cards'
import { sigilName } from '../sigils'
import type { LedgerV, Pending, TeamResult } from '../types'
import { Button } from './Button'
import { CoinIcon } from './Coin'
import { Panel } from './Panel'
import styles from './Prompts.module.css'
import { SigilChip } from './SigilChip'
import { tipHandlers } from './tip'

export function BidPrompt({
  onBid,
  partnerBid,
}: {
  onBid: (bid: number) => void
  partnerBid: number
}) {
  return (
    <Panel className={styles.bid} aria-label="Your bid">
      <div className={styles.bidGrid}>
        <Button variant="token" className={styles.nil} onClick={() => onBid(0)}>
          Nil
        </Button>
        {Array.from({ length: 13 }, (_, i) => i + 1).map((n) => (
          <Button key={n} variant="token" onClick={() => onBid(n)}>
            {n}
          </Button>
        ))}
      </div>
      {partnerBid >= 0 && (
        <span className={styles.hint}>Sage bid {partnerBid === 0 ? 'nil' : partnerBid}</span>
      )}
    </Panel>
  )
}

export function SwapPrompt({
  pending,
  sigil,
  picked,
  onConfirm,
}: {
  pending: Pending
  sigil: string | null
  picked: number
  onConfirm: () => void
}) {
  return (
    <Panel className={styles.prompt} role="dialog">
      <div className={styles.promptHead}>
        {sigil && <SigilChip id={sigil} />}
        {sigil ? sigilName(sigil) : 'Opening swap'}
      </div>
      <p className={styles.promptText}>
        Pick up to {pending.n} cards to give {SEAT_NAMES[2]}. You both give the same number: the
        smaller of your two picks.
      </p>
      <Button onClick={onConfirm}>
        Give {picked} card{picked === 1 ? '' : 's'}
      </Button>
    </Panel>
  )
}

export function LeadPrompt({ onChoose }: { onChoose: (pass: boolean) => void }) {
  return (
    <Panel className={styles.prompt} role="dialog">
      <div className={styles.promptHead}>Who leads next?</div>
      <div className={styles.actions}>
        <Button onClick={() => onChoose(false)}>I lead</Button>
        <Button variant="ghost" onClick={() => onChoose(true)}>
          {SEAT_NAMES[2]} leads
        </Button>
      </div>
    </Panel>
  )
}

const bidText = (b: number) => (b === 0 ? 'nil' : b < 0 ? '–' : String(b))

function ledgerCell(e: LedgerV | undefined, r: TeamResult) {
  if (!e) return { text: '—', sign: 0, void: false }
  const parts: string[] = []
  if (e.cp) parts.push(`${signed(e.cp)} pts`)
  if (e.add) parts.push(`${signed(e.add)} mult`)
  if (e.x !== 1) parts.push(`×${fmt(e.x)}`)
  if (e.nilp) parts.push(`${signed(e.nilp)} nil`)
  if (!parts.length) parts.push(`fired ×${e.fires}`)
  const onlyPoints = e.cp !== 0 && !e.add && e.x === 1 && !e.nilp
  return { text: parts.join(' '), sign: 1, void: !r.made && (onlyPoints || (r.flat && !e.nilp)) }
}

export function RoundSummary({
  result,
  round,
  over,
  winner,
  onContinue,
  canContinue,
}: {
  result: [TeamResult, TeamResult]
  round: number
  over: boolean
  winner?: 0 | 1 | 'draw'
  onContinue: () => void
  canContinue: boolean
}) {
  const row = (
    label: string,
    values: (string | number)[],
    opts: { signs?: number[]; tip?: string; voids?: boolean[] } = {},
  ) => (
    <tr {...tipHandlers({ lines: opts.tip ? [opts.tip] : [] })}>
      <th>{label}</th>
      {values.map((v, t) => (
        <td key={t} data-sign={opts.signs?.[t]} data-void={opts.voids?.[t] || undefined}>
          {v}
        </td>
      ))}
    </tr>
  )
  const ids = [...new Set(result.flatMap((r) => r.ledger.map((e) => e.id)))]
  const eng = result.some((r) => r.engCp || r.engAdd)
  return (
    <Panel className={`${styles.center} ${styles.summary}`}>
      <h2 className={styles.title}>
        {over
          ? winner === 'draw'
            ? 'Draw'
            : winner === 0
              ? 'Victory'
              : 'Defeat'
          : `Round ${round}`}
      </h2>
      <table className={styles.table}>
        <thead>
          <tr>
            <th />
            <th data-team="us">Us</th>
            <th data-team="them">Them</th>
          </tr>
        </thead>
        <tbody>
          {row(
            'Bids',
            result.map((r) => `${bidText(r.bids[0])} + ${bidText(r.bids[1])}`),
          )}
          {row(
            'Won',
            result.map((r) => (r.contract ? `${r.tricks}/${r.contract}` : '—')),
            {
              signs: result.map((r) => (r.contract ? (r.made ? 1 : -1) : 0)),
              tip: 'Tricks won by non-nil bidders / contract',
            },
          )}
          {row(
            'Base',
            result.map((r) => signed(r.base)),
            {
              tip: '10 per trick in the contract; a set scores −100 per trick, untouched by sigils',
            },
          )}
          {row(
            'Points',
            result.map((r) => (r.flat ? '—' : signed(r.made ? r.cp : r.cpLost))),
            {
              voids: result.map((r) => !r.made && r.cpLost !== 0),
              tip: 'Contract points; a set ignores them',
            },
          )}
          {result.some((r) => r.nilBids > 0) &&
            row(
              'Nil',
              result.map((r) => (r.nilBids ? signed(r.nilScore) : '—')),
              { signs: result.map((r) => Math.sign(r.nilScore)) },
            )}
          {row(
            'Mult',
            result.map((r) => `×${fmt(r.mult)}`),
            {
              voids: result.map((r) => r.flat && !r.nilBids),
              tip: 'Contract multiplier: 10 plus every +multiplier; a set ignores it',
            },
          )}
          {result.some((r) => r.x !== 1) &&
            row(
              '×Mult',
              result.map((r) => `×${fmt(r.x)}`),
              {
                voids: result.map((r) => r.flat && !r.nilBids),
                tip: 'Compounding multipliers, applied last; a set ignores them',
              },
            )}
          {(ids.length > 0 || eng) && (
            <tr className={styles.ledgerHead}>
              <th colSpan={3}>Sigils</th>
            </tr>
          )}
          {ids.map((id) => (
            <tr key={id}>
              <th>
                <span className={styles.chipCell} {...tipHandlers({ sigils: [{ id }] })}>
                  <SigilChip id={id} size={24} />
                  {sigilName(id)}
                </span>
              </th>
              {result.map((r, t) => {
                const c = ledgerCell(
                  r.ledger.find((e) => e.id === id),
                  r,
                )
                return (
                  <td key={t} className={styles.small} data-void={c.void || undefined}>
                    {c.text}
                  </td>
                )
              })}
            </tr>
          ))}
          {eng &&
            row(
              'Engravings',
              result.map(
                (r) =>
                  [
                    r.engCp ? `${signed(r.engCp)} pts` : '',
                    r.engAdd ? `${signed(r.engAdd)} mult` : '',
                  ]
                    .filter(Boolean)
                    .join(' ') || '—',
              ),
            )}
          <tr>
            <th>Bags</th>
            {result.map((r, t) => (
              <td
                key={t}
                className={styles.small}
                {...tipHandlers({
                  lines: [
                    'Overtricks and tricks won by nil bidders. Every 10 bags cost 1,000 points.',
                  ],
                })}
              >
                +{r.bags} → {r.bagsCarried}
                {r.bagPenalty ? ` (${signed(r.bagPenalty)})` : ''}
              </td>
            ))}
          </tr>
          <tr className={styles.delta}>
            <th>Round</th>
            {result.map((r, t) => (
              <td key={t} data-sign={Math.sign(r.score)}>
                {signed(r.score)}
              </td>
            ))}
          </tr>
          <tr className={styles.total}>
            <th>Score</th>
            {result.map((r, t) => (
              <td key={t}>{num(r.total)}</td>
            ))}
          </tr>
          <tr className={styles.goldRow}>
            <th>
              <CoinIcon className={styles.coin} />
            </th>
            {result.map((r, t) => (
              <td
                key={t}
                {...tipHandlers({
                  lines: [
                    `Base ${r.income.base}, contract ${r.income.contract}, nil ${r.income.nil}; now ${r.gold}`,
                  ],
                })}
              >
                +{r.income.total}
              </td>
            ))}
          </tr>
        </tbody>
      </table>
      <p className={styles.formula}>
        {result.map((r, t) => (
          <Fragment key={t}>
            <b>{t === 0 ? 'Us' : 'Them'}:</b> {r.formula}
            {t === 0 && <br />}
          </Fragment>
        ))}
      </p>
      {canContinue && (
        <div className={styles.actions}>
          <Button onClick={onContinue}>
            {over ? 'Play again' : round === 8 ? 'Final score' : 'Next'}
          </Button>
        </div>
      )}
    </Panel>
  )
}
