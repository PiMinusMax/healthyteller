//! How good the data of a food is, as one level for the whole food.
//!
//! The level follows the weakest value that matters. What matters is not every
//! nutrient, since no food is described by every nutrient, but:
//!
//! - the seven values every EU nutrition label must show; every food has them,
//!   if need be as a true zero,
//! - every nutrient the active profile needs (rules of its conditions, doctor's limits),
//! - the allergen data when the profile has allergies.
//!
//! A missing value among these makes the food incomplete (AGENTS.md rule 5).

use crate::food::Food;
use crate::nutrient::Nutrient;
use crate::profile::Profile;
use crate::rule::Rule;
use crate::source::{Derivation, EstimateMethod};

/// Data quality of a food, from best to worst.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataQuality {
    /// Every value that matters comes from the source: measured, label or source calculation.
    Source,
    /// At least one value is calculated from the ingredients.
    Interpolated,
    /// At least one value is estimated from a generic food or an estimation recipe.
    Estimated,
    /// At least one value that matters is missing.
    Incomplete,
}

impl DataQuality {
    /// Stable key, also used for translation ids.
    pub fn key(self) -> &'static str {
        match self {
            DataQuality::Source => "source",
            DataQuality::Interpolated => "interpolated",
            DataQuality::Estimated => "estimated",
            DataQuality::Incomplete => "incomplete",
        }
    }

    /// Symbol shown with the level; the fill shrinks with the quality.
    pub fn symbol(self) -> &'static str {
        match self {
            DataQuality::Source => "●",
            DataQuality::Interpolated => "◐",
            DataQuality::Estimated => "◔",
            DataQuality::Incomplete => "○",
        }
    }
}

/// The seven values of the EU nutrition label (Regulation (EU) No 1169/2011, Art. 30).
pub const LABEL_NUTRIENTS: [Nutrient; 7] = [
    Nutrient::Energy,
    Nutrient::Fat,
    Nutrient::SaturatedFat,
    Nutrient::Carbohydrate,
    Nutrient::Sugars,
    Nutrient::Protein,
    Nutrient::Salt,
];

#[derive(Clone, Debug, PartialEq)]
pub struct QualityReport {
    pub level: DataQuality,
    /// Nutrients that matter but have no value.
    pub missing: Vec<Nutrient>,
    /// The profile has allergies, but the food has no allergen data.
    pub allergens_unknown: bool,
}

/// Nutrients that matter for this profile, label values first.
pub fn required_nutrients(profile: Option<&Profile>, rules: &[Rule]) -> Vec<Nutrient> {
    let mut required = LABEL_NUTRIENTS.to_vec();
    if let Some(p) = profile {
        let from_rules = rules
            .iter()
            .filter(|r| p.conditions.contains(&r.condition))
            .map(|r| r.nutrient);
        let from_doctor = p.doctor_limits.iter().map(|l| l.nutrient);
        for n in from_rules.chain(from_doctor) {
            if !required.contains(&n) {
                required.push(n);
            }
        }
    }
    required
}

fn level_of(derivation: &Derivation) -> DataQuality {
    match derivation {
        Derivation::Measured | Derivation::Calculated | Derivation::Declared => DataQuality::Source,
        Derivation::Interpolated
        | Derivation::Estimated {
            method: EstimateMethod::Ingredients,
            ..
        } => DataQuality::Interpolated,
        Derivation::Estimated { .. } => DataQuality::Estimated,
    }
}

/// Rates the data of a food for a profile; without a profile only the label values count.
pub fn assess(food: &Food, profile: Option<&Profile>, rules: &[Rule]) -> QualityReport {
    let mut level = DataQuality::Source;
    let mut missing = Vec::new();
    for nutrient in required_nutrients(profile, rules) {
        // Salt and sodium describe the same thing; either is enough.
        let value = food.nutrients.get(&nutrient).or(match nutrient {
            Nutrient::Salt => food.nutrients.get(&Nutrient::Sodium),
            Nutrient::Sodium => food.nutrients.get(&Nutrient::Salt),
            _ => None,
        });
        match value {
            Some(v) => level = level.max(level_of(&v.derivation)),
            None => missing.push(nutrient),
        }
    }
    let allergens_unknown =
        profile.is_some_and(|p| !p.allergies.is_empty()) && !food.allergens.known;
    if !missing.is_empty() || allergens_unknown {
        level = DataQuality::Incomplete;
    }
    QualityReport {
        level,
        missing,
        allergens_unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allergen::Allergen;
    use crate::food::{AllergenData, FoodId, FoodKind};
    use crate::quantity::{Quantity, Unit};
    use crate::source::{SourceRef, SourcedValue, ValueBasis};

    fn value(derivation: Derivation) -> SourcedValue {
        SourcedValue {
            quantity: Quantity::new(1.0, Unit::Gram),
            basis: ValueBasis::Per100gAsSold,
            source: SourceRef {
                id: "test".into(),
                version: "1".into(),
                retrieved_at: "2026-01-01".into(),
            },
            derivation,
            samples: None,
            min: None,
            max: None,
        }
    }

    /// Invented food with all label values declared.
    fn labelled() -> Food {
        Food {
            id: FoodId("test:1".into()),
            name: "Testapfel".into(),
            brand: None,
            kind: FoodKind::Brand,
            nutrients: LABEL_NUTRIENTS
                .iter()
                .map(|&n| (n, value(Derivation::Declared)))
                .collect(),
            allergens: AllergenData {
                known: true,
                ..Default::default()
            },
            ingredients: None,
            nova_group: None,
            additives: Vec::new(),
            portions: Vec::new(),
            source: SourceRef {
                id: "test".into(),
                version: "1".into(),
                retrieved_at: "2026-01-01".into(),
            },
        }
    }

    #[test]
    fn label_values_from_the_source_are_source_quality() {
        let report = assess(&labelled(), None, &[]);
        assert_eq!(report.level, DataQuality::Source);
        assert!(report.missing.is_empty());
    }

    #[test]
    fn nutrients_beyond_the_label_do_not_matter_without_a_profile() {
        // An apple without vitamin B12 is not incomplete.
        let food = labelled();
        assert!(!food.nutrients.contains_key(&Nutrient::VitaminB12));
        assert_eq!(assess(&food, None, &[]).level, DataQuality::Source);
    }

    #[test]
    fn the_weakest_value_decides() {
        let mut food = labelled();
        food.nutrients
            .insert(Nutrient::Sugars, value(Derivation::Interpolated));
        assert_eq!(assess(&food, None, &[]).level, DataQuality::Interpolated);
        food.nutrients.insert(
            Nutrient::Fat,
            value(Derivation::Estimated {
                basis: "test".into(),
                method: EstimateMethod::Generic,
            }),
        );
        assert_eq!(assess(&food, None, &[]).level, DataQuality::Estimated);
    }

    #[test]
    fn a_missing_label_value_makes_the_food_incomplete() {
        let mut food = labelled();
        food.nutrients.remove(&Nutrient::SaturatedFat);
        let report = assess(&food, None, &[]);
        assert_eq!(report.level, DataQuality::Incomplete);
        assert_eq!(report.missing, vec![Nutrient::SaturatedFat]);
    }

    #[test]
    fn sodium_stands_in_for_salt() {
        let mut food = labelled();
        food.nutrients.remove(&Nutrient::Salt);
        food.nutrients
            .insert(Nutrient::Sodium, value(Derivation::Declared));
        assert_eq!(assess(&food, None, &[]).level, DataQuality::Source);
    }

    #[test]
    fn the_profile_adds_doctor_limits_and_allergens() {
        let food = labelled();
        let mut profile = Profile::new("p", "Test Person");
        profile.doctor_limits.push(crate::profile::DoctorLimit {
            nutrient: Nutrient::Potassium,
            max: None,
            min: None,
            basis: crate::rule::RuleBasis::Day,
            note: None,
        });
        let report = assess(&food, Some(&profile), &[]);
        assert_eq!(report.missing, vec![Nutrient::Potassium]);

        let mut food = labelled();
        food.allergens.known = false;
        let mut profile = Profile::new("p", "Test Person");
        profile.allergies.insert(Allergen::Eggs);
        let report = assess(&food, Some(&profile), &[]);
        assert!(report.allergens_unknown);
        assert_eq!(report.level, DataQuality::Incomplete);
    }
}
