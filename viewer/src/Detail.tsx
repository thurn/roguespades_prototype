import { useEffect, useRef } from 'react'
import { Icon } from './Icon'
import {
  type Interval,
  type Metric,
  type Sigil,
  categoryLabel,
  displayName,
  fmt,
  rarityColor,
} from './model'
import { RulesText } from './RulesText'
import styles from './App.module.css'

type Props = {
  sigil: Sigil
  byId: Record<string, Sigil>
  onOpen: (id: string) => void
  onClose: () => void
  onStep: (dir: number) => void
}

export function Detail({ sigil, byId, onOpen, onClose, onStep }: Props) {
  const panelRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
      else if (e.key === 'ArrowLeft') onStep(-1)
      else if (e.key === 'ArrowRight') onStep(1)
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [onClose, onStep])

  useEffect(() => panelRef.current?.focus(), [])

  const link = (id: string) =>
    byId[id] ? (
      <button className={styles.link} onClick={() => onOpen(id)}>
        {displayName(byId[id])}
        {byId[id].name && <span className={styles.linkId}> {id}</span>}
      </button>
    ) : (
      <span>{id}</span>
    )

  const { design, simplicity, estimates: est } = sigil

  return (
    <div className={styles.overlay} onMouseDown={(e) => e.target === e.currentTarget && onClose()}>
      <div
        ref={panelRef}
        className={styles.panel}
        role="dialog"
        aria-modal
        aria-label={displayName(sigil)}
        tabIndex={-1}
        style={{ '--fill': rarityColor(sigil.rarity) } as React.CSSProperties}
      >
        <div className={styles.panelBar}>
          <button className={styles.iconButton} onClick={() => onStep(-1)} aria-label="Previous">
            ‹
          </button>
          <button className={styles.iconButton} onClick={() => onStep(1)} aria-label="Next">
            ›
          </button>
          <span className={styles.barCode}>{sigil.id}</span>
          <button className={styles.iconButton} onClick={onClose} aria-label="Close">
            ✕
          </button>
        </div>

        <div className={styles.panelBody}>
          <div className={styles.hero}>
            <Icon name={sigil.icon} className={styles.heroBadge} />
            <div>
              <h2 className={styles.heroName} data-unnamed={!sigil.name}>
                {displayName(sigil)}
              </h2>
              <div className={styles.heroMeta}>
                <span className={styles.rarity}>{sigil.rarity ?? 'no rarity'}</span>
                {sigil.price != null && <> · {sigil.price}g</>}
                {sigil.category && <> · {categoryLabel(sigil.category)}</>}
                {sigil.role && <> · {sigil.role}</>}
                {sigil.status && (
                  <>
                    {' · '}
                    <span className={styles.status} data-status={sigil.status}>
                      {sigil.status}
                    </span>
                  </>
                )}
                {sigil.replacedBy && <> by {link(sigil.replacedBy)}</>}
              </div>
            </div>
          </div>

          <RulesText text={sigil.text} className={styles.heroText} />

          <dl className={styles.facts}>
            <Fact label="Archetypes">{sigil.archetypes?.join(', ')}</Fact>
            <Fact label="Source">{sigil.source}</Fact>
            <Fact label="Created in">{sigil.createdIn}</Fact>
            <Fact label="Touches opponents">
              {sigil.touchesOpponents == null ? undefined : sigil.touchesOpponents ? 'yes' : 'no'}
            </Fact>
            <Fact label="Icon">
              {[sigil.icon, sigil.iconFamily, sigil.iconWord].some(Boolean)
                ? [sigil.icon ?? '—', sigil.iconFamily, sigil.iconWord].filter(Boolean).join(' · ')
                : undefined}
            </Fact>
            <Fact label="Signature" wide>
              {sigil.signature && <code className={styles.code}>{sigil.signature}</code>}
            </Fact>
          </dl>

          {design && (
            <Section title="Design">
              <dl className={styles.prose}>
                <Fact label="Decision">{design.decision}</Fact>
                <Fact label="Opponent">{design.opponent}</Fact>
                <Fact label="Rationale">{design.rationale}</Fact>
                <Fact label="Partners">
                  {design.partners?.length ? (
                    <span className={styles.inlineList}>
                      {design.partners.map((p) => (
                        <span key={p}>{link(p)}</span>
                      ))}
                    </span>
                  ) : undefined}
                </Fact>
              </dl>
            </Section>
          )}

          {simplicity && (
            <Section
              title="Simplicity"
              aside={
                <>
                  C {simplicity.C ?? '—'} · S {simplicity.S != null ? fmtS(simplicity.S) : '—'}
                </>
              }
            >
              {simplicity.items?.length ? (
                <table className={styles.table}>
                  <tbody>
                    {simplicity.items.map((it, i) => (
                      <tr key={i}>
                        <td>{it.burden}</td>
                        <td>
                          <RulesText text={it.detail} className={styles.inlineText} />
                        </td>
                        <td className={styles.num}>{it.cost}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              ) : (
                <p className={styles.missing}>No burden items.</p>
              )}
            </Section>
          )}

          <Section
            title="Estimates"
            aside={
              est && (
                <>
                  {est.step}
                  {est.exposures != null && <> · {est.exposures.toLocaleString()} exposures</>}
                </>
              )
            }
          >
            {!est ? (
              <p className={styles.missing}>Not measured yet.</p>
            ) : (
              <div className={styles.estimates}>
                {est.metrics?.map((m) => (
                  <MetricBar key={m.key} metric={m} />
                ))}
                <dl className={styles.facts}>
                  <Fact label="Amount slope">{est.amountSlope && interval(est.amountSlope)}</Fact>
                  <Fact label="Skill gradient">
                    {est.skillGradient && interval(est.skillGradient)}
                  </Fact>
                  <Fact label="Amount history">
                    {est.amountHistory?.length ? (
                      <span className={styles.inlineList}>
                        {est.amountHistory.map((h, i) => (
                          <span key={i}>
                            <span className={styles.dim}>{h.step}</span> {fmt(h.amount)}
                          </span>
                        ))}
                      </span>
                    ) : undefined}
                  </Fact>
                </dl>
                {est.pairs?.length ? (
                  <div>
                    <span className={styles.label}>Strongest pairs</span>
                    <table className={styles.table}>
                      <tbody>
                        {[...est.pairs]
                          .sort((a, b) => b.synergy - a.synergy)
                          .map((p) => (
                            <tr key={p.with}>
                              <td>{link(p.with)}</td>
                              <td className={styles.num}>{fmt(p.synergy, true)}</td>
                            </tr>
                          ))}
                      </tbody>
                    </table>
                  </div>
                ) : null}
              </div>
            )}
          </Section>

          {sigil.history?.length ? (
            <Section title="History">
              <ol className={styles.history}>
                {sigil.history.map((h, i) => (
                  <li key={i}>
                    <span className={styles.step}>{h.step}</span>
                    <span>{h.note}</span>
                  </li>
                ))}
              </ol>
            </Section>
          ) : null}

          {sigil.reports?.length ? (
            <Section title="Reports">
              <ul className={styles.reports}>
                {sigil.reports.map((r) => (
                  <li key={r}>
                    <code className={styles.code}>{r}</code>
                  </li>
                ))}
              </ul>
            </Section>
          ) : null}

          <details className={styles.raw}>
            <summary>Effect JSON</summary>
            <pre>{JSON.stringify(sigil.effect ?? null, null, 2)}</pre>
          </details>
        </div>
      </div>
    </div>
  )
}

const fmtS = (s: number) => (Number.isInteger(s) ? String(s) : s.toFixed(2))

function interval(i: Interval) {
  return (
    <span className={styles.numText}>
      {fmt(i.value, true)}{' '}
      <span className={styles.dim}>
        [{fmt(i.lo)}, {fmt(i.hi)}]
      </span>
    </span>
  )
}

function Section({
  title,
  aside,
  children,
}: {
  title: string
  aside?: React.ReactNode
  children: React.ReactNode
}) {
  return (
    <section className={styles.section}>
      <div className={styles.sectionHead}>
        <h3 className={styles.label}>{title}</h3>
        {aside && <span className={styles.aside}>{aside}</span>}
      </div>
      {children}
    </section>
  )
}

function Fact({
  label,
  wide,
  children,
}: {
  label: string
  wide?: boolean
  children: React.ReactNode
}) {
  if (children == null || children === '' || children === false) return null
  return (
    <div className={`${styles.fact} ${wide ? styles.wide : ''}`}>
      <dt className={styles.label}>{label}</dt>
      <dd>{children}</dd>
    </div>
  )
}

/** A metric's estimate and 90% interval drawn against its target band. */
function MetricBar({ metric: m }: { metric: Metric }) {
  const known = [m.value, m.lo, m.hi, m.bandLo, m.bandHi].filter((v): v is number => v != null)
  let min = Math.min(...known)
  let max = Math.max(...known)
  const pad = (max - min) * 0.15 || Math.abs(max) * 0.5 || 1
  min -= pad
  max += pad
  const pos = (v: number) => `${((v - min) / (max - min)) * 100}%`
  const bandL = m.bandLo ?? min
  const bandR = m.bandHi ?? max
  const inBand =
    (m.bandLo == null || m.value >= m.bandLo) && (m.bandHi == null || m.value <= m.bandHi)
  const band =
    m.bandLo == null && m.bandHi == null
      ? 'no band'
      : m.bandLo == null
        ? `≤ ${fmt(m.bandHi!)}`
        : m.bandHi == null
          ? `≥ ${fmt(m.bandLo)}`
          : `${fmt(m.bandLo)} – ${fmt(m.bandHi)}`

  return (
    <div className={styles.metric} data-in-band={inBand}>
      <div className={styles.metricHead}>
        <span className={styles.metricLabel}>{m.label}</span>
        <span className={styles.metricValue}>
          {fmt(m.value)}{' '}
          <span className={styles.dim}>
            [{fmt(m.lo)}, {fmt(m.hi)}]
          </span>
        </span>
      </div>
      <div className={styles.track}>
        <div
          className={styles.band}
          data-open-lo={m.bandLo == null}
          data-open-hi={m.bandHi == null}
          style={{ left: pos(bandL), width: `calc(${pos(bandR)} - ${pos(bandL)})` }}
        />
        <div
          className={styles.whisker}
          style={{ left: pos(m.lo), width: `calc(${pos(m.hi)} - ${pos(m.lo)})` }}
        />
        <div className={styles.point} style={{ left: pos(m.value) }} />
      </div>
      <div className={styles.metricFoot}>
        <span>
          {m.key} · band {band}
        </span>
        {m.n != null && <span>n {m.n.toLocaleString()}</span>}
      </div>
    </div>
  )
}
