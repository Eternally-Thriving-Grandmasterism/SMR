# 01 — Passive Shutdown Path (Terminate-to-Safe-State)

**Checklist flag:** `passive_shutdown_path_documented`  
**Gate emphasis:** Compassion / Zero-Harm · Order  
**Contact:** info@Rathor.ai  
**Tied proposal:** `energy-open-smr-001` · fixture [`fixtures/reference_open_smr_package.json`](../../fixtures/reference_open_smr_package.json)  
**Revision:** 2026-07-26 (terminate-to-safe-state sequence documented)

> **DESIGN-TIME ONLY.** Conceptual lattice — **not** a licensed reactor, regulatory claim, certified protection system, or hardware specification.

## Goal

Document a **pure-geometry + natural-physics preference** that terminates the primary heat/reactivity pathways to a stable, inspectable, zero-power **safe state** without continuous active power or continuous operator intervention.

## Core philosophy

- **Passive priority** over active systems at every branch.
- Geometry and inherent feedback do the work; sensors and actuators are secondary monitors only.
- **Terminate-to-safe-state** is the single shared success criterion for **FM-01**, **FM-02**, and **FM-06**.
- Once the path is entered, the system must remain in the safe state even under station-blackout conditions (no off-site power, no forced flow).

## Terminate-to-safe-state sequence (conceptual)

### 1. Trigger detection (passive proxies only)

| Linked FM | Proxy intent |
|-----------|--------------|
| **FM-01** | Rising core ΔT / inventory reduction / natural-circulation flow inversion |
| **FM-02** | Positive period / flux spike beyond Doppler/moderator feedback window |
| **FM-06** | Loss of all active support buses |

### 2. Inherent response

- Negative reactivity insertion via temperature/void coefficients (**documented preference** — not a numerical claim).
- Geometry-driven **natural circulation** takes over residual heat removal.
- No reliance on pumps, powered valves, or external electricity after the first few seconds of the conceptual sequence.

### 3. Safe-state definition

- Core power → **decay-heat only**.
- Peak cladding / fuel temperature remains below design-time damage threshold **by geometry alone** (thresholds not asserted as licensed limits here).
- Inventory and pressure stabilize inside a passively cooled volume.
- State is **observable by simple local instrumentation** (no continuous remote link required).

### 4. Exit condition

- Safe state is **latched** until deliberate, multi-person, audited human action restores the system (aligns with item 6 local operability).

## Explicit linkage to failure modes

| FM | Pathway | How terminate-to-safe-state closes it | Primary TOLC gates |
|----|---------|--------------------------------------|--------------------|
| **FM-01** | Core / primary heat-up | Natural circulation + geometry bias removes residual heat | Order, Compassion |
| **FM-02** | Power excursion | Inherent feedback + passive shutdown path terminates excursion | Truth, Compassion |
| **FM-06** | Coincident loss of active support | Path requires **zero continuous off-site power** | Order, Compassion |

Mitigation intent for FM-01 / FM-02 / FM-06 is completed **by reference to this sequence** (see [`02_failure_modes.md`](02_failure_modes.md)).

## Shard & open-data integration

- `OpenSmrShard` is still **birthed** on strict TOLC 8 + class SMR; checklist items (including this one) advance **after** birth as explicit progress.
- When this note is accepted, the reference fixture sets:
  - `passive_shutdown_path_documented: true`
  - `safety_progress_completed` includes this item
- Export path (`export_reference_open_data_pretty` / `OpenDataPackage`) carries the flag in the shared audit artifact.

## Remaining design-time open points

- Concrete numerical thresholds and TH benchmarks remain **future work**.
- External independent review of this sequence is required before any claim of completeness (**item 4 stays false** until then).
- No substitution for site-specific engineering or licensed PSA.

## Non-goals

- No claim of regulatory acceptance.
- No numerical trip setpoints presented as licensed limits.
- No claim of physical readiness or hardware authorization.

---

*Mark `SafetyCaseItem::PassiveShutdown` when this terminate-to-safe-state note is reviewed and kept current against the open-data fixture.*
