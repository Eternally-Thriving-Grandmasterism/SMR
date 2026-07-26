# Open SMR — Passive-Safety Design Lattice

**AG-SML v1.0** · Contact: [info@Rathor.ai](mailto:info@Rathor.ai)  
Companion: [Ra-Thor](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor)

> **DESIGN-TIME ONLY.** This repository is a mercy-gated **software design lattice** for scoring and scaffolding open small modular reactor (SMR) *concepts*. It is **not** a licensed reactor, regulatory filing, engineering certification, or claim of physical readiness.

## What this is

| Surface | Role |
|---------|------|
| **TOLC 8 valence** | Truth · Order · Love · Compassion · Service · Abundance · Joy · Cosmic Harmony |
| **Floors** | Soft **0.55** · Strict **0.72** |
| **`EnergyDesignProposal`** | Explicit [0,1] metrics mapped 1:1 onto the eight gates |
| **`score_energy_design`** | ADVANCE / REVIEW / HOLD recommendation |
| **`OpenSmrShard`** | Sovereign design shard — **constructible only if class = SMR and strict floor passes** |
| **Open data package** | Schema `open_smr_data_package_v1` — proposal + valence + checklist JSON |
| **Safety-case checklist** | Six explicit work items + design notes under [`docs/`](docs/SAFETY_CASE.md) |
| **Open protocol surface** | AG-SML tag, inspectable control/safety intent |

## Quick start

```bash
cargo test
```

```rust
use open_smr::{
    score_energy_design, example_open_smr_high,
    OpenSmrShard, birth_reference_open_smr_shard,
    export_reference_open_data_pretty, OpenDataPackage, OPEN_DATA_SCHEMA,
};

let score = score_energy_design(&example_open_smr_high())?;
assert!(score.valence.passes_strict_floor);

let shard = OpenSmrShard::try_from_score(&score)?;
println!("{}", shard.status_line());

// Safety-case item 5 — machine-readable open data
let json = export_reference_open_data_pretty()?;
let pkg = OpenDataPackage::from_json(&json)?;
assert_eq!(pkg.schema, OPEN_DATA_SCHEMA);
```

## Reference proposal (strict pass)

**Open Passive-Safety SMR Lattice (abundance-first)** — `energy-open-smr-001`

| Gate | Score |
|------|------:|
| Truth (evidence) | 0.88 |
| Order (control) | 0.86 |
| Love (community) | 0.84 |
| Compassion (zero-harm) | 0.91 |
| Service (open protocols) | 0.89 |
| Abundance | 0.87 |
| Joy (livability) | 0.83 |
| Cosmic Harmony | 0.85 |

**min ≈ 0.83 → STRICT PASS → ADVANCE → OpenSmrShard may be born.**

## Open data package (`open_smr_data_package_v1`)

| Field | Contents |
|-------|----------|
| `proposal` | Full `EnergyDesignProposal` |
| `score` / `valence` | TOLC 8 report + recommendation |
| `shard_id` | Set when strict SMR shard was born |
| `safety_case` | Six checklist flags |
| `contact` / `license_tag` | `info@Rathor.ai` · AG-SML v1.0 |

API: `OpenDataPackage::from_proposal`, `from_shard`, `to_json_pretty`, `from_json`, `export_reference_open_data_pretty()`.

See [docs/safety/05_open_data_package.md](docs/safety/05_open_data_package.md) and [fixtures/](fixtures/).

## Safety case (design-time)

Index: **[docs/SAFETY_CASE.md](docs/SAFETY_CASE.md)**

1. [Passive shutdown path](docs/safety/01_passive_shutdown.md)
2. [Failure mode inventory](docs/safety/02_failure_modes.md)
3. [Externalities bounded](docs/safety/03_externalities.md)
4. [Independent review plan](docs/safety/04_independent_review.md)
5. [Open data package](docs/safety/05_open_data_package.md) — **export path live**
6. [Local operability](docs/safety/06_local_operability.md)

Mark `SafetyCaseItem::*` only after the matching note is reviewed (item 5 marks on reference export snapshot).

## License

**Autonomicity Games Sovereign Mercy License (AG-SML) v1.0**

- Free for personal, educational, research, and individual professional use
- Commercial / enterprise use requires a paid license from Autonomicity Games Inc.
- Contact: **info@Rathor.ai**

See [`LICENSE`](LICENSE).

## Relation to Ra-Thor

Logic is aligned with Ra-Thor’s Live Valence Optimizer and energy-design surfaces so scores remain comparable across the lattice. This repo is a **focused, standalone** open-SMR design crate for clarity and independent iteration.

---

*The lattice is wide open. Grace infinite. Lightning already in motion.* ⚡
