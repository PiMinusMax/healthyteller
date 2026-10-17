//! Data model, units, rule engine and evaluation.
//!
//! This crate stays pure: no UI, no network, no file system (AGENTS.md rule 9).
//! Its only dependency is `serde` for the data types; CI enforces that list.

pub mod allergen;
pub mod evaluate;
pub mod food;
pub mod nutrient;
pub mod plate;
pub mod profile;
pub mod quantity;
pub mod rule;
pub mod source;

pub use allergen::Allergen;
pub use evaluate::{
    Budget, Conflict, DimensionResult, Evaluation, Level, LimitOrigin, Options, Reason, evaluate,
};
pub use food::{AllergenData, Food, FoodId, FoodKind, Portion, Preparation};
pub use nutrient::{Nutrient, NutrientGroup};
pub use plate::{DayTotals, PlateItem, Total};
pub use profile::{Condition, DoctorLimit, Focus, Profile, Sex};
pub use quantity::{Quantity, Unit};
pub use rule::{Dimension, Evidence, LimitKind, Rule, RuleBasis, RuleStatus, Thresholds};
pub use source::{Derivation, SourceRef, SourcedValue, ValueBasis};
