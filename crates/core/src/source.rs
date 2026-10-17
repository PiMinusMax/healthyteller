//! Where a value comes from. Every stored value carries its source (AGENTS.md rule 2).

use serde::{Deserialize, Serialize};

use crate::quantity::Quantity;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    /// Id of the source, e.g. `off` or `bls`.
    pub id: String,
    pub version: String,
    /// ISO 8601 date the data was retrieved.
    pub retrieved_at: String,
}

/// How the source arrived at a value. Shown next to every value in the nutrient list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Derivation {
    /// Laboratory analysis.
    Measured,
    /// Derived by the source itself, e.g. recipe calculation or a borrowed value.
    Calculated,
    /// Declared by the manufacturer on the label.
    Declared,
    /// Calculated by the source from the ingredients and their shares, e.g.
    /// `nutriments_estimated` in Open Food Facts. Less exact than the label.
    Interpolated,
    /// Estimated by the app following a documented method (AGENTS.md rule 3).
    Estimated {
        basis: String,
        #[serde(default)]
        method: EstimateMethod,
    },
}

/// The documented estimation methods of the app (docs/adr/0004-datenlage-und-schaetzung.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstimateMethod {
    /// Method 1: taken from the matching generic food.
    #[default]
    Generic,
    /// Method 2: calculated from the ingredients and their shares.
    Ingredients,
    /// Method 3: an estimation recipe of the project.
    Recipe,
}

impl Derivation {
    /// Symbol shown next to the value: ● measured, ∑ calculated or declared,
    /// ◐ calculated by the source from ingredients, ≈ estimate.
    pub fn mark(&self) -> &'static str {
        match self {
            Derivation::Measured => "●",
            Derivation::Calculated | Derivation::Declared => "∑",
            Derivation::Interpolated => "◐",
            Derivation::Estimated { .. } => "≈",
        }
    }

    pub fn is_estimate(&self) -> bool {
        matches!(self, Derivation::Estimated { .. })
    }
}

/// What a stored amount refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueBasis {
    /// Per 100 g edible portion, raw.
    Per100gRaw,
    /// Per 100 g edible portion, prepared.
    Per100gPrepared,
    /// Per 100 g as sold (packaged products).
    Per100gAsSold,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourcedValue {
    pub quantity: Quantity,
    pub basis: ValueBasis,
    pub source: SourceRef,
    pub derivation: Derivation,
    /// Number of samples, if the source states it.
    pub samples: Option<u32>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}
