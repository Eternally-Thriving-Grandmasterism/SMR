//! TOLC 8 valence vector and floor checks for energy design scoring.
//!
//! Soft floor 0.55 · Strict floor 0.72 (same constants as Ra-Thor Live Valence).
//! AG-SML v1.0 | Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Soft floor: any gate below this fails soft pass (compassion engaged).
pub const THETA_MIN_SOFT: f64 = 0.55;

/// Strict floor for high-stakes designs (required to birth OpenSmrShard).
pub const THETA_MIN_STRICT: f64 = 0.72;

/// Eight-gate valence vector. Each component is in [0, 1].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValenceVector {
    pub truth: f64,
    pub order: f64,
    pub love: f64,
    /// Compassion / Zero-Harm
    pub compassion: f64,
    pub service: f64,
    pub abundance: f64,
    pub joy: f64,
    pub cosmic_harmony: f64,
}

impl ValenceVector {
    pub fn min_gate(&self) -> f64 {
        [
            self.truth,
            self.order,
            self.love,
            self.compassion,
            self.service,
            self.abundance,
            self.joy,
            self.cosmic_harmony,
        ]
        .into_iter()
        .fold(1.0_f64, f64::min)
    }

    pub fn mean(&self) -> f64 {
        (self.truth
            + self.order
            + self.love
            + self.compassion
            + self.service
            + self.abundance
            + self.joy
            + self.cosmic_harmony)
            / 8.0
    }

    pub fn passes_floor(&self, theta: f64) -> bool {
        self.min_gate() + f64::EPSILON >= theta
    }

    pub fn passes_soft_floor(&self) -> bool {
        self.passes_floor(THETA_MIN_SOFT)
    }

    pub fn passes_strict_floor(&self) -> bool {
        self.passes_floor(THETA_MIN_STRICT)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveValenceReport {
    pub vector: ValenceVector,
    pub aggregate_mean: f64,
    pub min_gate: f64,
    pub passes_soft_floor: bool,
    pub passes_strict_floor: bool,
    pub council_note: String,
}

impl LiveValenceReport {
    pub fn from_vector(vector: ValenceVector) -> Self {
        let min_gate = vector.min_gate();
        let aggregate_mean = vector.mean();
        let passes_soft = vector.passes_soft_floor();
        let passes_strict = vector.passes_strict_floor();

        let council_note = if passes_strict {
            format!(
                "Energy design STRICT PASS | min={:.3} mean={:.3} | all TOLC 8 ≥ {:.2}",
                min_gate, aggregate_mean, THETA_MIN_STRICT
            )
        } else if passes_soft {
            format!(
                "Energy design SOFT PASS | min={:.3} mean={:.3} | below strict {:.2} — council review",
                min_gate, aggregate_mean, THETA_MIN_STRICT
            )
        } else {
            format!(
                "Energy design FLOOR FAIL | min={:.3} mean={:.3} | soft floor {:.2} not met — hold",
                min_gate, aggregate_mean, THETA_MIN_SOFT
            )
        };

        Self {
            vector,
            aggregate_mean,
            min_gate,
            passes_soft_floor: passes_soft,
            passes_strict_floor: passes_strict,
            council_note,
        }
    }
}
