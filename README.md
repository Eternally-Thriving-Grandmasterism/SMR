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
| **Safety-case checklist** | Six explicit work items (start incomplete) |
| **Open protocol surface** | AG-SML tag, inspectable control/safety intent |

## Quick start

```bash
cargo test
```

```rust
use open_smr::{
    score_energy_design, example_open_smr_high,
    OpenSmrShard, birth_reference_open_smr_shard,
};

let score = score_energy_design(&example_open_smr_high())?;
assert!(score.valence.passes_strict_floor);

let shard = OpenSmrShard::try_from_score(&score)?;
// or: let shard = birth_reference_open_smr_shard()?;
println!("{}", shard.status_line());
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
