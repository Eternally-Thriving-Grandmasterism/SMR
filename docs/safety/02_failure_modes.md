# 02 — Failure Mode Inventory (Design-Time)

**Checklist flag:** `failure_mode_inventory_drafted`  
**Gate emphasis:** Truth · Order · Compassion  
**Contact:** info@Rathor.ai

> DESIGN-TIME ONLY. Structured inventory template — not a completed FMEA/HAZOP package.

## Intent

Maintain an open, living list of credible failure modes for the concept class, with harm pathways and mitigation *intent* (not certified mitigations).

## Inventory template

| ID | Mode | Pathway to harm | Detection intent | Mitigation intent | Open data needed |
|----|------|-----------------|------------------|-------------------|------------------|
| FM-01 | Loss of forced flow | Heat-up | Flow / ΔT proxies | Passive circulation bias | TH benchmarks |
| FM-02 | Reactivity insertion (concept) | Power excursion | Flux / period proxies | Inherent feedback + shutdown path | Kinetics refs |
| FM-03 | Heat sink degradation | Over-temp | Sink / inventory proxies | Redundant sink geometry | Site climate data |
| FM-04 | Instrumentation common-cause | Blind control | Diverse sensors | Manual / passive priority | Diversity study |
| FM-05 | Module handling error | Mechanical / release | Procedural holds | Factory QA + interlocks | Logistics model |

Add rows as analysis deepens. Never delete a closed mode without a dated rationale.

## Non-goals

- Not a claim that mitigations are sufficient.
- Not a substitute for licensed PSA.

---

*Mark `SafetyCaseItem::FailureModes` only after the table is populated for the current concept revision.*
