# 04 — Independent Review Planned (Design-Time)

**Checklist flag:** `independent_review_planned`  
**Gate emphasis:** Truth · Service  
**Contact:** info@Rathor.ai

> DESIGN-TIME ONLY. Review process intent — not an executed formal peer review or regulatory step.

## Intent

Ensure no single author or closed group is the sole arbiter of safety arguments.

## Planned review layers

1. **Open lattice review** — public issues/PRs on this repository under AG-SML.
2. **Domain specialists** — thermal-hydraulics, materials, human factors (invited, unpaid or separately contracted; not claimed here).
3. **Mercy / ethics pass** — TOLC 8 re-score after material design changes.
4. **Adversarial read** — explicit attempt to break passive-safety and externality claims.

## Cadence (suggested)

- After any change that alters failure modes or passive path assumptions → re-open review.
- Before marking multiple checklist items complete in one batch → independent second reader.

## Record-keeping

- Reviewer identity or handle (optional privacy).
- Date, revision hash, findings, disposition.

## Non-goals

- Not NRC/CNSC/IAEA engagement.
- Not certification.

---

*Mark `SafetyCaseItem::IndependentReview` when the plan above is adopted and a first review slot is scheduled or opened as a public issue.*
