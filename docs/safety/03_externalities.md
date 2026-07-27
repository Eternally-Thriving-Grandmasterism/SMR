# 03 — Externalities Bounded (Design-Time)

**Checklist flag:** `externalities_bounded`  
**Gate emphasis:** Cosmic Harmony · Compassion · Abundance  
**Contact:** info@Rathor.ai  
**Tied proposal:** `energy-open-smr-001` · fixture [`fixtures/reference_open_smr_package.json`](../../fixtures/reference_open_smr_package.json)  
**Revision:** 2026-07-26 (named proxies + owners + open-data expectations per category)

> DESIGN-TIME ONLY. Scope map for multi-generational and community externalities — **not** an environmental impact statement, EIA/EA, or license to site or emit.

## Intent

Name the externalities the design lattice must bound so **abundance claims cannot hide harm transfer**. Boundedness here is lattice discipline, not regulatory closure.

## Boundedness rule

An externality is “bounded” in this lattice when it has:

1. a **name**,
2. a **measurement or proxy** approach,
3. a **responsible review party** (even if future / external),
4. **open data expectations** stated under AG-SML.

## Category table (energy-open-smr-001)

| # | Category | Proxy / measurement intent | Review party (intent) | Open data expectation | Linked FM / notes |
|---|----------|----------------------------|------------------------|------------------------|-------------------|
| 1 | **Local community** | Land footprint class; construction traffic intensity band; noise envelope class | Future site host + open lattice issue | Publish footprint class + traffic assumptions in package notes | Align Love / community_benefit 0.84 |
| 2 | **Water / air** | Qualitative pathway list only (no numeric release claims) | Independent domain reader (item 4) | Pathway list in safety notes; never invent emission numbers | Compassion |
| 3 | **Waste & end-of-life** | Module disposition pathway tag; inventory continuity | Future operator + open accounting preference | Chain-of-custody fields when modules exist (design intent) | **FM-10** |
| 4 | **Supply chain** | Critical-part pedigree intent; dual-source preference where practical | Receipt inspection owner (future) | BOM/pedigree schema outline (no controlled data) | **FM-11** |
| 5 | **Intergenerational** | Knowledge retention of passive path + FM inventory; residual longevity class | Future operators + public docs | This repo + fixture remain the living record | Cosmic Harmony 0.85 |
| 6 | **Equity of access** | Design does not *require* only capital-dense operators; AG-SML individual research use | Lattice maintainers | License + open protocol surface stay public | Abundance 0.87 · Service 0.89 |

## Linkage to other safety items

- **Passive path (01)** — safe-state latch must not export unmanaged residuals into category 3/5.
- **Failure modes (02)** — FM-10 and FM-11 are the primary externality-bearing modes.
- **Open data (05)** — fixture and package are the shared audit object for these bounds.
- **Independent review (04)** — external readers should stress-test categories 2, 3, and 5 especially.

## Non-goals

- Not regulatory EIA/EA.
- Not a license to site or emit.
- Not a claim that all pathways are fully quantified.

---

*Mark `SafetyCaseItem::Externalities` when each category has at least one named proxy and owner (this revision meets that design-time bar).*
