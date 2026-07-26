# 05 — Open Data Package Outline (Design-Time)

**Checklist flag:** `open_data_package_outlined`  
**Gate emphasis:** Truth · Service · Abundance  
**Contact:** info@Rathor.ai  
**Status:** Machine-readable export path **implemented** (`open_smr_data_package_v1`).

> DESIGN-TIME ONLY. Data package outline + JSON export — not a release of proprietary or controlled nuclear data.

## Intent

Define what must be publishable so others can reproduce valence scores, challenge assumptions, and extend the lattice without closed black boxes.

## Schema

- **Name:** `open_smr_data_package_v1`
- **API:** `OpenDataPackage`, `export_reference_open_data_pretty()`, `export_proposal_open_data_pretty()`

### Envelope fields

| Field | Contents |
|-------|----------|
| `schema` | `open_smr_data_package_v1` |
| `package_version` | crate version |
| `contact` | `info@Rathor.ai` |
| `license_tag` | `AG-SML v1.0` |
| `disclaimer` | design-time only |
| `proposal` | full `EnergyDesignProposal` |
| `score` | `EnergyDesignScore` + recommendation |
| `valence` | `LiveValenceReport` (8 gates + floors) |
| `shard_id` | present if strict SMR shard was born |
| `safety_case` | six checklist booleans |
| `safety_progress_*` | completed / total |

## Usage

```rust
use open_smr::{export_reference_open_data_pretty, OpenDataPackage};

let json = export_reference_open_data_pretty()?;
let pkg = OpenDataPackage::from_json(&json)?;
assert_eq!(pkg.schema, "open_smr_data_package_v1");
```

```bash
cargo test open_data
```

## Rules

- Prefer AG-SML-compatible artifacts.
- No inclusion of export-controlled or classified material.
- Commercial packaging of the lattice requires a paid license path (see LICENSE).

## Non-goals

- Not a claim that all physics data will be open (some inputs may remain external).
- Not a data room for investors or regulators.

---

*Mark `SafetyCaseItem::OpenDataPackage` when exporting via this path (reference helper marks it on the snapshot).*
