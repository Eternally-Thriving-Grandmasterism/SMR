# 02 — Failure Mode Inventory (Design-Time)

**Checklist flag:** `failure_mode_inventory_drafted`  
**Gate emphasis:** Truth · Order · Compassion  
**Contact:** info@Rathor.ai  
**Tied proposal:** `energy-open-smr-001` · fixture [`fixtures/reference_open_smr_package.json`](../../fixtures/reference_open_smr_package.json)  
**Revision:** 2026-07-26 (FM-01/02/06 mitigation closed by reference to terminate-to-safe-state)

> DESIGN-TIME ONLY. Structured inventory — **not** a completed FMEA/HAZOP package, PSA, or licensed safety analysis.

## Intent

Maintain an open, living list of credible failure modes for the **Open Passive-Safety SMR Lattice** concept class, with harm pathways and mitigation *intent* (not certified mitigations). Rows are scored against the same open-data package so valence claims (especially Compassion / Zero-Harm **0.91** and Order **0.86**) remain auditable.

## Shared audit artifact

| Field | Value |
|-------|-------|
| Schema | `open_smr_data_package_v1` |
| Proposal id | `energy-open-smr-001` |
| Shard id | `shard-smr-energy-open-smr-001` |
| Strict pass | yes (min gate 0.83) |
| Zero-harm (compassion) | 0.91 |
| Control stability (order) | 0.86 |

Any change to proposal gate scores should trigger a pass over this inventory.

## Inventory (concept class)

| ID | Mode | Pathway to harm | Detection intent | Mitigation intent | Open data / refs needed | Primary TOLC gates |
|----|------|-----------------|------------------|-------------------|-------------------------|--------------------|
| FM-01 | Loss of forced flow | Core / primary heat-up | Flow, ΔT, inventory proxies | **Terminate-to-safe-state** ([01](01_passive_shutdown.md)): natural circulation + geometry bias removes residual heat | TH benchmarks, friction maps | Order, Compassion |
| FM-02 | Reactivity insertion (concept) | Power excursion → fuel/temp challenge | Flux / period / reactivity proxies | **Terminate-to-safe-state** ([01](01_passive_shutdown.md)): inherent feedback + passive path terminates excursion | Kinetics refs, Doppler/moderator notes | Truth, Compassion |
| FM-03 | Ultimate heat sink degradation | Over-temp / pressure challenge | Sink temp, inventory, ambient proxies | Redundant / diverse sink geometry intent | Site climate envelopes | Order, Cosmic Harmony |
| FM-04 | Instrumentation common-cause | Blind or misleading control | Cross-check diversity, watchdog | Manual/passive priority; fail-safe defaults | Diversity/CCF study outline | Truth, Order |
| FM-05 | Module handling / transport error | Mechanical damage, release pathway | Procedural holds, load/path sensors | Factory QA, interlocks, clear module seams | Logistics + drop/impact models | Service, Compassion |
| FM-06 | Loss of offsite power (LOOP) | Coincident loss of active support | Grid / bus status | **Terminate-to-safe-state** ([01](01_passive_shutdown.md)): path requires zero continuous off-site power | Station blackout concept note | Order, Compassion |
| FM-07 | Primary pressure boundary challenge | Leak / loss of inventory | Pressure, level, activity proxies | Inventory makeup philosophy (design-time); leak-before-break *discussion only* | Materials + NDE intent | Truth, Compassion |
| FM-08 | Software / I&C logic fault | Spurious actuation or inhibit | Diverse logic, audit logs | Open control-logic intent (AG-SML); inspectable state | Open protocol + test vectors | Truth, Service |
| FM-09 | Human performance / procedure gap | Delayed recognition or wrong action | Training proxies, simplified HMI intent | Reduce reliance on continuous high-intensity actions (align 01 + 06) | Local operability notes | Love, Service |
| FM-10 | Spent / used module disposition error | Long-lived residual mismanagement | Chain-of-custody, inventory tags | Open accounting preference; intergenerational notes (see 03) | Waste pathway outline | Cosmic Harmony, Abundance |
| FM-11 | Supply-chain critical part fraud/defect | Latent common-mode hardware | Receipt inspection, pedigree | Dual-source intent where practical; open BOM preference | BOM + pedigree schema | Truth, Service |
| FM-12 | Malevolent / security event (concept) | Forced unsafe state or information compromise | Access control, anomaly | Design-time security principles only — not a threat assessment | High-level security note | Order, Compassion |

**Closed modes:** none. Do not delete a row without a dated rationale and fixture revision.

## Linkage to passive shutdown (item 1)

FM-01, FM-02, and FM-06 **mitigation intent is completed by reference** to the terminate-to-safe-state sequence in [`01_passive_shutdown.md`](01_passive_shutdown.md):

1. Trigger detection via passive proxies  
2. Inherent response (feedback + natural circulation)  
3. Safe-state definition (decay heat only, local observability)  
4. Latched exit until audited multi-person restore  

If that note cannot absorb a mode, either deepen 01 or lower Compassion / Order scores on the proposal and re-export the open-data package.

## How to update

1. Edit this table.
2. Re-run / refresh `fixtures/reference_open_smr_package.json` if proposal scores change.
3. Note the date and open-data package version in the revision line above.
4. Only then mark `SafetyCaseItem::FailureModes` on a shard snapshot.

## Non-goals

- Not a claim that mitigations are sufficient.
- Not a substitute for licensed PSA, HAZOP, or regulatory review.
- Not site-specific.

---

*Mark `SafetyCaseItem::FailureModes` when this inventory is populated for the current concept revision and cross-checked against the open-data fixture.*
