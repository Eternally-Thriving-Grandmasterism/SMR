//! Energy design proposals scored through TOLC 8 floors.
//!
//! DESIGN-TIME ONLY — not a licensed reactor or physical readiness claim.
//! AG-SML v1.0 | Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::valence::{LiveValenceReport, ValenceVector, THETA_MIN_SOFT, THETA_MIN_STRICT};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyDesignClass {
    Smr,
    Geothermal,
    FusionAdjacent,
    He3Pathway,
    SolarAbundance,
    HybridLattice,
    Experimental,
}

impl EnergyDesignClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnergyDesignClass::Smr => "SMR",
            EnergyDesignClass::Geothermal => "Geothermal",
            EnergyDesignClass::FusionAdjacent => "FusionAdjacent",
            EnergyDesignClass::He3Pathway => "He3Pathway",
            EnergyDesignClass::SolarAbundance => "SolarAbundance",
            EnergyDesignClass::HybridLattice => "HybridLattice",
            EnergyDesignClass::Experimental => "Experimental",
        }
    }
}

/// Explicit [0,1] metrics mapped 1:1 onto TOLC 8 gates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyDesignProposal {
    pub id: String,
    pub title: String,
    pub class: EnergyDesignClass,
    pub evidence_grounding: f64,
    pub control_stability: f64,
    pub community_benefit: f64,
    pub zero_harm_safety: f64,
    pub open_serviceability: f64,
    pub abundance_density: f64,
    pub livability_impact: f64,
    pub long_horizon_harmony: f64,
    #[serde(default)]
    pub notes: String,
}

impl EnergyDesignProposal {
    pub fn validate_bounds(&self) -> Result<(), String> {
        let fields = [
            ("evidence_grounding", self.evidence_grounding),
            ("control_stability", self.control_stability),
            ("community_benefit", self.community_benefit),
            ("zero_harm_safety", self.zero_harm_safety),
            ("open_serviceability", self.open_serviceability),
            ("abundance_density", self.abundance_density),
            ("livability_impact", self.livability_impact),
            ("long_horizon_harmony", self.long_horizon_harmony),
        ];
        for (name, v) in fields {
            if !(0.0..=1.0).contains(&v) {
                return Err(format!(
                    "Mercy Gate (Truth): energy design field '{}' out of [0,1] (got {})",
                    name, v
                ));
            }
        }
        if self.id.trim().is_empty() || self.title.trim().is_empty() {
            return Err("Mercy Gate (Truth): energy design id/title must be non-empty".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyDesignScore {
    pub proposal_id: String,
    pub title: String,
    pub class: String,
    pub valence: LiveValenceReport,
    pub recommendation: String,
}

fn map_proposal_to_vector(p: &EnergyDesignProposal) -> ValenceVector {
    ValenceVector {
        truth: p.evidence_grounding.clamp(0.0, 1.0),
        order: p.control_stability.clamp(0.0, 1.0),
        love: p.community_benefit.clamp(0.0, 1.0),
        compassion: p.zero_harm_safety.clamp(0.0, 1.0),
        service: p.open_serviceability.clamp(0.0, 1.0),
        abundance: p.abundance_density.clamp(0.0, 1.0),
        joy: p.livability_impact.clamp(0.0, 1.0),
        cosmic_harmony: p.long_horizon_harmony.clamp(0.0, 1.0),
    }
}

/// Score an energy design under TOLC 8 (soft 0.55 / strict 0.72).
pub fn score_energy_design(proposal: &EnergyDesignProposal) -> Result<EnergyDesignScore, String> {
    proposal.validate_bounds()?;
    let vector = map_proposal_to_vector(proposal);
    let valence = LiveValenceReport::from_vector(vector);

    let recommendation = if valence.passes_strict_floor {
        format!(
            "ADVANCE: '{}' ({}) clears strict TOLC 8. Open shard + formal safety case next.",
            proposal.title,
            proposal.class.as_str()
        )
    } else if valence.passes_soft_floor {
        format!(
            "REVIEW: '{}' ({}) soft-pass only (min={:.3}). Raise weakest gates before hardware path.",
            proposal.title,
            proposal.class.as_str(),
            valence.min_gate
        )
    } else {
        format!(
            "HOLD: '{}' ({}) fails soft floor (min={:.3}). Redesign under Zero-Harm + Truth before cascade.",
            proposal.title,
            proposal.class.as_str(),
            valence.min_gate
        )
    };

    Ok(EnergyDesignScore {
        proposal_id: proposal.id.clone(),
        title: proposal.title.clone(),
        class: proposal.class.as_str().into(),
        valence,
        recommendation,
    })
}

/// Reference open passive-safety SMR concept (clears strict floor).
/// Design-time illustration only — not a licensed reactor claim.
pub fn example_open_smr_high() -> EnergyDesignProposal {
    EnergyDesignProposal {
        id: "energy-open-smr-001".into(),
        title: "Open Passive-Safety SMR Lattice (abundance-first)".into(),
        class: EnergyDesignClass::Smr,
        evidence_grounding: 0.88,
        control_stability: 0.86,
        community_benefit: 0.84,
        zero_harm_safety: 0.91,
        open_serviceability: 0.89,
        abundance_density: 0.87,
        livability_impact: 0.83,
        long_horizon_harmony: 0.85,
        notes: "Design-time open protocol + passive safety emphasis; not a licensed reactor claim."
            .into(),
    }
}

pub fn example_geothermal_marginal() -> EnergyDesignProposal {
    EnergyDesignProposal {
        id: "energy-geo-marginal-002".into(),
        title: "Closed-Loop Geothermal Node (early evidence)".into(),
        class: EnergyDesignClass::Geothermal,
        evidence_grounding: 0.62,
        control_stability: 0.70,
        community_benefit: 0.68,
        zero_harm_safety: 0.74,
        open_serviceability: 0.66,
        abundance_density: 0.58,
        livability_impact: 0.71,
        long_horizon_harmony: 0.64,
        notes: "Early-stage; abundance density and evidence need lift for strict floor.".into(),
    }
}

pub fn example_experimental_fail() -> EnergyDesignProposal {
    EnergyDesignProposal {
        id: "energy-exp-fail-003".into(),
        title: "Unverified Exotic Propulsion Heat Source".into(),
        class: EnergyDesignClass::Experimental,
        evidence_grounding: 0.35,
        control_stability: 0.40,
        community_benefit: 0.50,
        zero_harm_safety: 0.28,
        open_serviceability: 0.45,
        abundance_density: 0.60,
        livability_impact: 0.42,
        long_horizon_harmony: 0.38,
        notes: "Intentionally weak Zero-Harm + Truth to exercise HOLD path.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_smr_strict_pass() {
        let score = score_energy_design(&example_open_smr_high()).unwrap();
        assert!(score.valence.passes_strict_floor);
        assert!(score.valence.min_gate >= THETA_MIN_STRICT);
        assert!(score.recommendation.starts_with("ADVANCE"));
    }

    #[test]
    fn geothermal_soft_only() {
        let score = score_energy_design(&example_geothermal_marginal()).unwrap();
        assert!(score.valence.passes_soft_floor);
        assert!(!score.valence.passes_strict_floor);
    }

    #[test]
    fn experimental_fails_soft() {
        let score = score_energy_design(&example_experimental_fail()).unwrap();
        assert!(!score.valence.passes_soft_floor);
        assert!(score.valence.min_gate < THETA_MIN_SOFT);
    }
}
