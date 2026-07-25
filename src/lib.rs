//! # open-smr
//!
//! Open Passive-Safety SMR design lattice under **AG-SML v1.0**.
//!
//! - Score energy designs on **TOLC 8** gates (soft 0.55 / strict 0.72)
//! - Birth an **`OpenSmrShard`** only when class is SMR **and** strict floor clears
//! - Design-time safety-case checklist + open protocol surface
//!
//! **Not** a licensed reactor, hardware authorization, or physical readiness claim.
//!
//! Companion lattice: [Ra-Thor](https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor)
//! Contact: **info@Rathor.ai**

mod valence;
mod energy_design;
mod open_smr_shard;

pub use valence::{LiveValenceReport, ValenceVector, THETA_MIN_SOFT, THETA_MIN_STRICT};

pub use energy_design::{
    score_energy_design, EnergyDesignClass, EnergyDesignProposal, EnergyDesignScore,
    example_experimental_fail, example_geothermal_marginal, example_open_smr_high,
};

pub use open_smr_shard::{
    birth_reference_open_smr_shard, DesignEnvelope, OpenProtocolSurface, OpenSmrShard,
    SafetyCaseChecklist, SafetyCaseItem,
};

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Canonical contact.
pub const CONTACT: &str = "info@Rathor.ai";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_to_end_reference_shard() {
        let score = score_energy_design(&example_open_smr_high()).unwrap();
        assert!(score.valence.passes_strict_floor);
        let shard = OpenSmrShard::try_from_score(&score).unwrap();
        assert!(shard.status_line().contains("shard-smr-energy-open-smr-001"));
        assert_eq!(CONTACT, "info@Rathor.ai");
    }
}
