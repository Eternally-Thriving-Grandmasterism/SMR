# 06 — Local Operability Notes (Design-Time)

**Checklist flag:** `local_operability_notes`  
**Gate emphasis:** Service · Joy · Abundance  
**Contact:** info@Rathor.ai  
**Tied proposal:** `energy-open-smr-001` · fixture [`fixtures/reference_open_smr_package.json`](../../fixtures/reference_open_smr_package.json)  
**Revision:** 2026-07-26 (module seams + inspection + restore latch aligned with 01)

> DESIGN-TIME ONLY. Operability intent for community-scale understanding — **not** an operations manual, training program, or staffing plan for any real plant.

## Intent

Keep the concept operable in principle by people and institutions that are **not** mega-capital operators alone: clear interfaces, modularity, inspectability, and a deliberate restore path from the passive safe state.

## Operability map

| # | Theme | Design-time intent | Consistency check |
|---|-------|--------------------|-------------------|
| 1 | **Module boundaries** | Factory module vs site systems; clear mechanical / electrical / I&C seams | Supports FM-05 handling modes |
| 2 | **Inspection access** | Passive features and local sensors inspectable without heroic outages | Safe-state must remain **locally observable** (01 §3) |
| 3 | **Staffing philosophy** | Reduce reliance on continuous high-intensity control actions | Aligns with passive priority (01 philosophy) |
| 4 | **Language & docs** | Safety and protocol docs remain human-readable in this repo | AG-SML + open_smr_data_package_v1 |
| 5 | **Local benefit** | Heat/power product concepts admit district or community coupling discussion without proprietary lock-in | Abundance / Service gates |
| 6 | **Safe-state restore** | Exit from latched safe state requires deliberate, **multi-person, audited** human action | **01 §4 exit condition** |

## Restore latch (link to passive path)

From [`01_passive_shutdown.md`](01_passive_shutdown.md):

> Safe state is latched until deliberate, multi-person, audited human action restores the system.

Local operability therefore includes:

- Clear **indication** that the unit is in the latched safe state.
- A **restore procedure intent** that cannot be a single silent software or single-person action (design-time principle only).
- Documentation that stays available **without** continuous remote connectivity.

## Alignment with failure modes

| FM | Operability relevance |
|----|----------------------|
| FM-05 | Module seams + handling |
| FM-08 | Inspectable open control-logic intent |
| FM-09 | Reduced continuous high-intensity actions; clear procedures |
| FM-01/02/06 | Local observability of safe state after terminate-to-safe-state |

## Non-goals

- Not a claim of simplified licensing.
- Not a staffing plan for any real plant.
- Not a claim that untrained personnel can operate a reactor.

---

*Mark `SafetyCaseItem::LocalOperability` when module boundary and inspection intents are consistent with the passive-shutdown and failure-mode notes (this revision targets that bar).*
