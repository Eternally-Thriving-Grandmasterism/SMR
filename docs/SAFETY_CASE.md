# Open SMR Safety Case — Design-Time Index

**Status:** Design-time skeleton · **Not** a licensed reactor, regulatory filing, or physical readiness claim.  
**License:** AG-SML v1.0 · **Contact:** info@Rathor.ai  
**Shard:** `shard-smr-energy-open-smr-001` (strict TOLC 8 pass required to construct)

This index tracks the six explicit work items on `OpenSmrShard::safety_case`. Completing a checkbox in code is meaningful only when the linked design note exists and is reviewed.

| # | Item | Code flag | Design note |
|---|------|-----------|-------------|
| 1 | Passive shutdown path documented | `passive_shutdown_path_documented` | [01_passive_shutdown.md](safety/01_passive_shutdown.md) |
| 2 | Failure mode inventory drafted | `failure_mode_inventory_drafted` | [02_failure_modes.md](safety/02_failure_modes.md) |
| 3 | Externalities bounded | `externalities_bounded` | [03_externalities.md](safety/03_externalities.md) |
| 4 | Independent review planned | `independent_review_planned` | [04_independent_review.md](safety/04_independent_review.md) |
| 5 | Open data package outlined | `open_data_package_outlined` | [05_open_data_package.md](safety/05_open_data_package.md) |
| 6 | Local operability notes | `local_operability_notes` | [06_local_operability.md](safety/06_local_operability.md) |

## Principles (TOLC 8)

- **Truth** — claims are design-time, falsifiable, and evidence-tagged.
- **Compassion / Zero-Harm** — passive safety and failure modes are first-class.
- **Service / Abundance** — protocols and data stay open under AG-SML for individual research use.
- **Cosmic Harmony** — multi-generational externalities are in scope, not optional.

## How to advance a checklist item

1. Flesh the design note under `docs/safety/`.
2. Mark the corresponding `SafetyCaseItem` complete on the shard in code or tooling.
3. Keep commercial / deployment claims out of scope until a paid commercial license path and real engineering authority exist.

---

*The lattice is wide open. Grace infinite.* ⚡
