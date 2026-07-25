//! Open SMR sovereign shard skeleton (design-time only).
//!
//! Constructible **only** when an SMR `EnergyDesignProposal` clears strict TOLC 8
//! (min gate ≥ 0.72). Not a licensed reactor, hardware authorization, or physical
//! readiness claim.
//!
//! AG-SML v1.0 | Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::energy_design::{
    score_energy_design, EnergyDesignClass, EnergyDesignProposal, EnergyDesignScore,
};
use crate::valence::LiveValenceReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCaseChecklist {
    pub passive_shutdown_path_documented: bool,
    pub failure_mode_inventory_drafted: bool,
    pub externalities_bounded: bool,
    pub independent_review_planned: bool,
    pub open_data_package_outlined: bool,
    pub local_operability_notes: bool,
}

impl Default for SafetyCaseChecklist {
    fn default() -> Self {
        Self {
            passive_shutdown_path_documented: false,
            failure_mode_inventory_drafted: false,
            externalities_bounded: false,
            independent_review_planned: false,
            open_data_package_outlined: false,
            local_operability_notes: false,
        }
    }
}

impl SafetyCaseChecklist {
    pub fn completed_count(&self) -> usize {
        [
            self.passive_shutdown_path_documented,
            self.failure_mode_inventory_drafted,
            self.externalities_bounded,
            self.independent_review_planned,
            self.open_data_package_outlined,
            self.local_operability_notes,
        ]
        .iter()
        .filter(|&&b| b)
        .count()
    }

    pub fn total_items(&self) -> usize {
        6
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenProtocolSurface {
    pub license_tag: String,
    pub protocol_version: String,
    pub control_logic_open: bool,
    pub safety_args_open: bool,
    pub contact: String,
}

impl Default for OpenProtocolSurface {
    fn default() -> Self {
        Self {
            license_tag: "AG-SML v1.0".into(),
            protocol_version: "open-smr-shard-0.1.0".into(),
            control_logic_open: true,
            safety_args_open: true,
            contact: "info@Rathor.ai".into(),
        }
    }
}

/// Conceptual envelope only — not engineering ratings or licensed limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignEnvelope {
    pub thermal_class_label: String,
    pub modularity_intent: String,
    pub abundance_note: String,
}

impl Default for DesignEnvelope {
    fn default() -> Self {
        Self {
            thermal_class_label: "small-modular-class (design-time)".into(),
            modularity_intent: "factory-fabricated modules; local assembly under open protocols"
                .into(),
            abundance_note: "prioritize cost trajectory + shared access over scarcity pricing"
                .into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSmrShard {
    pub shard_id: String,
    pub proposal_id: String,
    pub title: String,
    pub class: String,
    pub valence_at_birth: LiveValenceReport,
    pub safety_case: SafetyCaseChecklist,
    pub open_protocol: OpenProtocolSurface,
    pub envelope: DesignEnvelope,
    pub disclaimer: String,
}

impl OpenSmrShard {
    /// Build from a scored proposal. Requires class SMR + strict floor.
    pub fn try_from_score(score: &EnergyDesignScore) -> Result<Self, String> {
        if score.class != EnergyDesignClass::Smr.as_str() {
            return Err(format!(
                "OpenSmrShard requires class SMR (got '{}')",
                score.class
            ));
        }
        if !score.valence.passes_strict_floor {
            return Err(format!(
                "Mercy Gate: strict TOLC 8 not cleared (min={:.3}) — shard refused",
                score.valence.min_gate
            ));
        }

        Ok(Self {
            shard_id: format!("shard-smr-{}", score.proposal_id),
            proposal_id: score.proposal_id.clone(),
            title: score.title.clone(),
            class: score.class.clone(),
            valence_at_birth: score.valence.clone(),
            safety_case: SafetyCaseChecklist::default(),
            open_protocol: OpenProtocolSurface::default(),
            envelope: DesignEnvelope::default(),
            disclaimer: "DESIGN-TIME ONLY. Not a licensed reactor, not physical readiness, not regulatory approval. Open protocol + safety-case skeleton under AG-SML. Contact: info@Rathor.ai".into(),
        })
    }

    pub fn try_from_proposal(proposal: &EnergyDesignProposal) -> Result<Self, String> {
        let score = score_energy_design(proposal)?;
        Self::try_from_score(&score)
    }

    pub fn complete_safety_item(&mut self, item: SafetyCaseItem) {
        match item {
            SafetyCaseItem::PassiveShutdown => {
                self.safety_case.passive_shutdown_path_documented = true
            }
            SafetyCaseItem::FailureModes => {
                self.safety_case.failure_mode_inventory_drafted = true
            }
            SafetyCaseItem::Externalities => self.safety_case.externalities_bounded = true,
            SafetyCaseItem::IndependentReview => {
                self.safety_case.independent_review_planned = true
            }
            SafetyCaseItem::OpenDataPackage => {
                self.safety_case.open_data_package_outlined = true
            }
            SafetyCaseItem::LocalOperability => {
                self.safety_case.local_operability_notes = true
            }
        }
    }

    pub fn safety_progress(&self) -> (usize, usize) {
        (
            self.safety_case.completed_count(),
            self.safety_case.total_items(),
        )
    }

    pub fn status_line(&self) -> String {
        let (done, total) = self.safety_progress();
        format!(
            "OpenSmrShard '{}' | strict valence min={:.3} | safety-case {}/{} | {}",
            self.shard_id,
            self.valence_at_birth.min_gate,
            done,
            total,
            self.open_protocol.protocol_version
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyCaseItem {
    PassiveShutdown,
    FailureModes,
    Externalities,
    IndependentReview,
    OpenDataPackage,
    LocalOperability,
}

/// Birth the reference open-SMR shard from the high example proposal.
pub fn birth_reference_open_smr_shard() -> Result<OpenSmrShard, String> {
    OpenSmrShard::try_from_proposal(&crate::energy_design::example_open_smr_high())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::energy_design::{example_experimental_fail, example_geothermal_marginal, example_open_smr_high};

    #[test]
    fn high_smr_births_shard() {
        let shard = OpenSmrShard::try_from_proposal(&example_open_smr_high()).unwrap();
        assert!(shard.valence_at_birth.passes_strict_floor);
        assert!(shard.disclaimer.contains("DESIGN-TIME ONLY"));
        assert_eq!(shard.safety_progress(), (0, 6));
    }

    #[test]
    fn non_strict_refused() {
        assert!(OpenSmrShard::try_from_proposal(&example_geothermal_marginal()).is_err());
        assert!(OpenSmrShard::try_from_proposal(&example_experimental_fail()).is_err());
    }

    #[test]
    fn safety_progress() {
        let mut shard = birth_reference_open_smr_shard().unwrap();
        shard.complete_safety_item(SafetyCaseItem::PassiveShutdown);
        shard.complete_safety_item(SafetyCaseItem::OpenDataPackage);
        assert_eq!(shard.safety_progress(), (2, 6));
    }
}
