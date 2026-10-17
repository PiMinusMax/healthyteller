//! Foods with their values, allergen data and the portion a person eats.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::allergen::Allergen;
use crate::nutrient::Nutrient;
use crate::quantity::Quantity;
use crate::source::{SourceRef, SourcedValue};

/// Source-qualified id, e.g. `off:4000000000000`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FoodId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoodKind {
    Generic,
    Brand,
    Dish,
}

/// Allergen information as the source states it. Nothing found never means "free of".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllergenData {
    /// The source has allergen or ingredient information at all.
    pub known: bool,
    pub contains: BTreeSet<Allergen>,
    pub traces: BTreeSet<Allergen>,
    /// Completeness of the product data between 0 and 1, if the source states it.
    pub completeness: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Food {
    pub id: FoodId,
    pub name: String,
    pub brand: Option<String>,
    pub kind: FoodKind,
    pub nutrients: BTreeMap<Nutrient, SourcedValue>,
    pub allergens: AllergenData,
    pub ingredients: Option<String>,
    /// NOVA group 1 to 4.
    pub nova_group: Option<u8>,
    pub additives: Vec<String>,
    pub source: SourceRef,
}

impl Food {
    /// Amount of a nutrient in the given portion; `None` when the source has no value.
    /// A missing value is never zero (AGENTS.md rule 5).
    pub fn amount_in(&self, nutrient: Nutrient, portion: &Portion) -> Option<Quantity> {
        self.nutrients
            .get(&nutrient)
            .map(|v| v.quantity.scale(portion.grams / 100.0))
    }

    /// Share of the tracked nutrients the source has a value for, between 0 and 1.
    pub fn nutrient_completeness(&self, nutrients: &[Nutrient]) -> f64 {
        if nutrients.is_empty() {
            return 0.0;
        }
        let known = nutrients
            .iter()
            .filter(|n| self.nutrients.contains_key(n))
            .count();
        known as f64 / nutrients.len() as f64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Preparation {
    Raw,
    Boiled,
    Steamed,
    Fried,
    Grilled,
    Smoked,
}

impl Preparation {
    pub const ALL: [Preparation; 6] = [
        Preparation::Raw,
        Preparation::Boiled,
        Preparation::Steamed,
        Preparation::Fried,
        Preparation::Grilled,
        Preparation::Smoked,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Preparation::Raw => "raw",
            Preparation::Boiled => "boiled",
            Preparation::Steamed => "steamed",
            Preparation::Fried => "fried",
            Preparation::Grilled => "grilled",
            Preparation::Smoked => "smoked",
        }
    }

    pub fn from_key(key: &str) -> Option<Preparation> {
        Preparation::ALL.into_iter().find(|p| p.key() == key)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Portion {
    pub grams: f64,
    pub preparation: Preparation,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::quantity::Unit;
    use crate::source::{Derivation, ValueBasis};

    /// Fixture food with invented values, only for tests.
    pub fn fixture_food(values: &[(Nutrient, f64, Unit)]) -> Food {
        let source = SourceRef {
            id: "fixture".into(),
            version: "1".into(),
            retrieved_at: "2026-01-01".into(),
        };
        Food {
            id: FoodId("fixture:1".into()),
            name: "Fixture food".into(),
            brand: None,
            kind: FoodKind::Generic,
            nutrients: values
                .iter()
                .map(|&(n, amount, unit)| {
                    (
                        n,
                        SourcedValue {
                            quantity: Quantity::new(amount, unit),
                            basis: ValueBasis::Per100gRaw,
                            source: source.clone(),
                            derivation: Derivation::Measured,
                            samples: None,
                            min: None,
                            max: None,
                        },
                    )
                })
                .collect(),
            allergens: AllergenData::default(),
            ingredients: None,
            nova_group: None,
            additives: Vec::new(),
            source,
        }
    }

    #[test]
    fn scales_to_portion() {
        let food = fixture_food(&[(Nutrient::Protein, 10.0, Unit::Gram)]);
        let portion = Portion {
            grams: 150.0,
            preparation: Preparation::Raw,
        };
        assert_eq!(
            food.amount_in(Nutrient::Protein, &portion),
            Some(Quantity::new(15.0, Unit::Gram))
        );
    }

    #[test]
    fn missing_value_is_none_not_zero() {
        let food = fixture_food(&[]);
        let portion = Portion {
            grams: 100.0,
            preparation: Preparation::Raw,
        };
        assert_eq!(food.amount_in(Nutrient::Salt, &portion), None);
    }

    #[test]
    fn completeness_counts_known_values() {
        let food = fixture_food(&[(Nutrient::Protein, 1.0, Unit::Gram)]);
        assert_eq!(
            food.nutrient_completeness(&[Nutrient::Protein, Nutrient::Salt]),
            0.5
        );
    }
}
