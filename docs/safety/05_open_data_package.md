# 05 — Open Data Package Outline (Design-Time)

**Checklist flag:** `open_data_package_outlined`  
**Gate emphasis:** Truth · Service · Abundance  
**Contact:** info@Rathor.ai

> DESIGN-TIME ONLY. Data package outline — not a release of proprietary or controlled nuclear data.

## Intent

Define what must be publishable so others can reproduce valence scores, challenge assumptions, and extend the lattice without closed black boxes.

## Package contents (outline)

| Artifact | Format | Notes |
|----------|--------|-------|
| Proposal JSON | `EnergyDesignProposal` serde | Gate scores + notes |
| Valence report | `LiveValenceReport` | min/mean/floors |
| Shard snapshot | `OpenSmrShard` | checklist + protocol |
| Safety notes | `docs/safety/*.md` | this tree |
| Assumption log | markdown/table | dated revisions |
| External references | bibliography | no controlled data |

## Rules

- Prefer AG-SML-compatible artifacts.
- No inclusion of export-controlled or classified material.
- Commercial packaging of the lattice requires a paid license path (see LICENSE).

## Non-goals

- Not a claim that all physics data will be open (some inputs may remain external).
- Not a data room for investors or regulators.

---

*Mark `SafetyCaseItem::OpenDataPackage` when a machine-readable export path exists for proposal + valence + checklist.*
