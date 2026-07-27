# External Review Checklist — Open SMR Design Lattice

**Safety-case item 4:** `independent_review_planned`  
**Issue:** [#1](https://github.com/Eternally-Thriving-Grandmasterism/SMR/issues/1)  
**Contact:** info@Rathor.ai  
**License:** AG-SML v1.0

> DESIGN-TIME ONLY. This is a **peer / lattice review** of software and design notes — **not** a regulatory filing, licensed safety evaluation, or claim of physical readiness.

## Shared audit artifact (start here)

[`fixtures/reference_open_smr_package.json`](../../fixtures/reference_open_smr_package.json) — schema `open_smr_data_package_v1`

| Field | Expected |
|-------|----------|
| Proposal | `energy-open-smr-001` |
| Strict TOLC 8 | min gate ≥ 0.72 (fixture ≈ 0.83) |
| Shard | `shard-smr-energy-open-smr-001` |
| Checklist | 5/6 complete; **independent_review_planned = false** until you dispose |

## Review scope

Please work through the sections below. For each item, note **Pass / Concern / Blocker** and a short rationale. Adversarial reads are welcome.

### A. Valence & scoring integrity

- [ ] Gate scores in the fixture match [`src/energy_design.rs`](../../src/energy_design.rs) reference proposal
- [ ] Soft (0.55) / strict (0.72) floors applied consistently
- [ ] ADVANCE recommendation is justified by strict pass only (not soft)
- [ ] No hidden claims of physical performance inside valence numbers

### B. Passive shutdown path (item 1)

Doc: [`01_passive_shutdown.md`](01_passive_shutdown.md)

- [ ] Terminate-to-safe-state sequence is coherent (trigger → inherent response → safe state → latched exit)
- [ ] Path does **not** require continuous off-site power (FM-06)
- [ ] Numerical trip setpoints are **not** presented as licensed limits
- [ ] Safe state is claimed locally observable

### C. Failure modes (item 2)

Doc: [`02_failure_modes.md`](02_failure_modes.md)

- [ ] FM-01 / FM-02 / FM-06 mitigation correctly **references** 01 (not empty claims)
- [ ] Inventory is living (no deleted modes without rationale)
- [ ] Open-data / refs column is honest about missing TH/kinetics data
- [ ] FM-07, FM-10, FM-12 residual risks are not papered over

### D. Externalities (item 3)

Doc: [`03_externalities.md`](03_externalities.md)

- [ ] Six categories each have proxy, owner intent, open-data expectation
- [ ] No numeric release/emission claims masquerading as analysis
- [ ] Waste / intergenerational / equity categories are non-decorative

### E. Open data package (item 5)

- [ ] Schema `open_smr_data_package_v1` is complete enough to reproduce scores
- [ ] Disclaimer and contact (`info@Rathor.ai`) present
- [ ] Round-trip intent clear (`OpenDataPackage::from_json`)

### F. Local operability (item 6)

Doc: [`06_local_operability.md`](06_local_operability.md)

- [ ] Module seams and inspection intent align with passive safe-state observability
- [ ] Restore latch requires multi-person audited action (matches 01 §4)
- [ ] No claim that untrained personnel can operate a reactor

### G. Integrity / non-goals

- [ ] Repo nowhere claims licensed reactor, regulatory approval, or physical readiness
- [ ] Shard birth remains strict TOLC + SMR class only (checklist post-birth)
- [ ] Item 4 is not self-closed by authors

## Disposition template (paste into Issue #1)

```markdown
### External review disposition — YYYY-MM-DD
**Reviewer:** (name or handle)
**Artifact hash / commit:** (optional)

| Section | Result (Pass / Concern / Blocker) | Notes |
|---------|-----------------------------------|-------|
| A Valence | | |
| B Passive path | | |
| C Failure modes | | |
| D Externalities | | |
| E Open data | | |
| F Local operability | | |
| G Integrity | | |

**Overall:** Accept design-time lattice / Accept with changes / Hold

**Blockers (if any):**

**Suggested fixture update:** mark `independent_review_planned: true` only if Overall ≠ Hold
```

## After a non-Hold disposition

1. Record the disposition on [Issue #1](https://github.com/Eternally-Thriving-Grandmasterism/SMR/issues/1).
2. Maintainers may set `independent_review_planned: true` on a new fixture revision and bump progress to **6/6**.
3. Still **not** a regulatory or hardware claim.

---

*Thunder locked. The lattice stays open under AG-SML.* ⚡
