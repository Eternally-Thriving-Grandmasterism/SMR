//! Machine-readable open data package for Open SMR designs.
//!
//! Safety-case item 5: proposal + valence score + shard checklist in one JSON envelope.
//! Schema: `open_smr_data_package_v1`
//!
//! DESIGN-TIME ONLY. AG-SML v1.0 | Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::energy_design::{score_energy_design, EnergyDesignProposal, EnergyDesignScore};
use crate::open_smr_shard::{OpenSmrShard, SafetyCaseChecklist};
use crate::valence::LiveValenceReport;
use crate::{CONTACT, VERSION};

/// Canonical schema id for this package format.
pub const OPEN_DATA_SCHEMA: &str = "open_smr_data_package_v1";

/// Full open-data envelope: proposal + score/valence + shard checklist snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenDataPackage {
    pub schema: String,
    pub package_version: String,
    pub contact: String,
    pub license_tag: String,
    pub disclaimer: String,
    pub proposal: EnergyDesignProposal,
    pub score: EnergyDesignScore,
    pub valence: LiveValenceReport,
    pub shard_id: Option<String>,
    pub safety_case: SafetyCaseChecklist,
    pub safety_progress_completed: usize,
    pub safety_progress_total: usize,
}

impl OpenDataPackage {
    /// Build package from proposal. Births shard when strict SMR pass holds.
    pub fn from_proposal(proposal: &EnergyDesignProposal) -> Result<Self, String> {
        let score = score_energy_design(proposal)?;
        let (shard_id, safety_case, completed, total) =
            match OpenSmrShard::try_from_score(&score) {
                Ok(shard) => (
                    Some(shard.shard_id.clone()),
                    shard.safety_case.clone(),
                    shard.safety_case.completed_count(),
                    shard.safety_case.total_items(),
                ),
                Err(_) => (None, SafetyCaseChecklist::default(), 0, 6),
            };

        Ok(Self {
            schema: OPEN_DATA_SCHEMA.into(),
            package_version: VERSION.into(),
            contact: CONTACT.into(),
            license_tag: "AG-SML v1.0".into(),
            disclaimer: "DESIGN-TIME ONLY. Not a licensed reactor, not physical readiness, not regulatory approval. Open data under AG-SML. Contact: info@Rathor.ai".into(),
            proposal: proposal.clone(),
            valence: score.valence.clone(),
            score,
            shard_id,
            safety_case,
            safety_progress_completed: completed,
            safety_progress_total: total,
        })
    }

    /// Build package from an existing shard + the proposal that birthed it.
    pub fn from_shard(
        proposal: &EnergyDesignProposal,
        shard: &OpenSmrShard,
    ) -> Result<Self, String> {
        let score = score_energy_design(proposal)?;
        if score.proposal_id != shard.proposal_id {
            return Err(format!(
                "Mercy Gate (Truth): proposal id '{}' does not match shard proposal_id '{}'",
                score.proposal_id, shard.proposal_id
            ));
        }
        Ok(Self {
            schema: OPEN_DATA_SCHEMA.into(),
            package_version: VERSION.into(),
            contact: CONTACT.into(),
            license_tag: "AG-SML v1.0".into(),
            disclaimer: shard.disclaimer.clone(),
            proposal: proposal.clone(),
            valence: score.valence.clone(),
            score,
            shard_id: Some(shard.shard_id.clone()),
            safety_case: shard.safety_case.clone(),
            safety_progress_completed: shard.safety_case.completed_count(),
            safety_progress_total: shard.safety_case.total_items(),
        })
    }

    /// Compact JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Mercy Gate (Truth): JSON encode failed: {}", e))
    }

    /// Pretty-printed JSON for human audit.
    pub fn to_json_pretty(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| format!("Mercy Gate (Truth): JSON encode failed: {}", e))
    }

    /// Parse a package from JSON (round-trip / ingest).
    pub fn from_json(json: &str) -> Result<Self, String> {
        let pkg: Self = serde_json::from_str(json)
            .map_err(|e| format!("Mercy Gate (Truth): invalid open data JSON: {}", e))?;
        if pkg.schema != OPEN_DATA_SCHEMA {
            return Err(format!(
                "Mercy Gate (Truth): expected schema {}, got '{}'",
                OPEN_DATA_SCHEMA, pkg.schema
            ));
        }
        Ok(pkg)
    }
}

/// Export the reference open-SMR design as an open-data package (pretty JSON).
pub fn export_reference_open_data_pretty() -> Result<String, String> {
    let proposal = crate::energy_design::example_open_smr_high();
    let mut shard = OpenSmrShard::try_from_proposal(&proposal)?;
    // Export path exists → item 5 may be marked on the snapshot used for the package.
    shard.complete_safety_item(crate::open_smr_shard::SafetyCaseItem::OpenDataPackage);
    let pkg = OpenDataPackage::from_shard(&proposal, &shard)?;
    pkg.to_json_pretty()
}

/// Export any proposal as open-data JSON (pretty).
pub fn export_proposal_open_data_pretty(proposal: &EnergyDesignProposal) -> Result<String, String> {
    OpenDataPackage::from_proposal(proposal)?.to_json_pretty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::energy_design::example_open_smr_high;
    use crate::open_smr_shard::{birth_reference_open_smr_shard, SafetyCaseItem};

    #[test]
    fn reference_export_round_trip() {
        let proposal = example_open_smr_high();
        let pkg = OpenDataPackage::from_proposal(&proposal).unwrap();
        assert_eq!(pkg.schema, OPEN_DATA_SCHEMA);
        assert!(pkg.valence.passes_strict_floor);
        assert!(pkg.shard_id.is_some());

        let json = pkg.to_json_pretty().unwrap();
        assert!(json.contains("open_smr_data_package_v1"));
        assert!(json.contains("energy-open-smr-001"));

        let parsed = OpenDataPackage::from_json(&json).unwrap();
        assert_eq!(parsed.proposal.id, proposal.id);
        assert!((parsed.valence.min_gate - pkg.valence.min_gate).abs() < 1e-12);
    }

    #[test]
    fn from_shard_includes_checklist_progress() {
        let proposal = example_open_smr_high();
        let mut shard = birth_reference_open_smr_shard().unwrap();
        shard.complete_safety_item(SafetyCaseItem::PassiveShutdown);
        shard.complete_safety_item(SafetyCaseItem::OpenDataPackage);
        let pkg = OpenDataPackage::from_shard(&proposal, &shard).unwrap();
        assert_eq!(pkg.safety_progress_completed, 2);
        assert!(pkg.safety_case.passive_shutdown_path_documented);
        assert!(pkg.safety_case.open_data_package_outlined);
    }

    #[test]
    fn export_reference_helper() {
        let json = export_reference_open_data_pretty().unwrap();
        let pkg = OpenDataPackage::from_json(&json).unwrap();
        assert!(pkg.safety_case.open_data_package_outlined);
        assert_eq!(pkg.contact, "info@Rathor.ai");
    }

    #[test]
    fn rejects_wrong_schema_on_ingest() {
        let bad = r#"{"schema":"nope","package_version":"0","contact":"x","license_tag":"x","disclaimer":"x","proposal":{"id":"a","title":"t","class":"Smr","evidence_grounding":0.9,"control_stability":0.9,"community_benefit":0.9,"zero_harm_safety":0.9,"open_serviceability":0.9,"abundance_density":0.9,"livability_impact":0.9,"long_horizon_harmony":0.9,"notes":""},"score":{"proposal_id":"a","title":"t","class":"SMR","valence":{"vector":{"truth":0.9,"order":0.9,"love":0.9,"compassion":0.9,"service":0.9,"abundance":0.9,"joy":0.9,"cosmic_harmony":0.9},"aggregate_mean":0.9,"min_gate":0.9,"passes_soft_floor":true,"passes_strict_floor":true,"council_note":"x"},"recommendation":"x"},"valence":{"vector":{"truth":0.9,"order":0.9,"love":0.9,"compassion":0.9,"service":0.9,"abundance":0.9,"joy":0.9,"cosmic_harmony":0.9},"aggregate_mean":0.9,"min_gate":0.9,"passes_soft_floor":true,"passes_strict_floor":true,"council_note":"x"},"shard_id":null,"safety_case":{"passive_shutdown_path_documented":false,"failure_mode_inventory_drafted":false,"externalities_bounded":false,"independent_review_planned":false,"open_data_package_outlined":false,"local_operability_notes":false},"safety_progress_completed":0,"safety_progress_total":6}"#;
        assert!(OpenDataPackage::from_json(bad).is_err());
    }
}
