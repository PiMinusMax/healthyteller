//! Nutrients and other substances the app tracks.

use serde::{Deserialize, Serialize};

use crate::quantity::Unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NutrientGroup {
    Energy,
    Macronutrients,
    Carbohydrates,
    Fats,
    Minerals,
    TraceElements,
    Vitamins,
    Other,
}

impl NutrientGroup {
    pub const ALL: [NutrientGroup; 8] = [
        NutrientGroup::Energy,
        NutrientGroup::Macronutrients,
        NutrientGroup::Carbohydrates,
        NutrientGroup::Fats,
        NutrientGroup::Minerals,
        NutrientGroup::TraceElements,
        NutrientGroup::Vitamins,
        NutrientGroup::Other,
    ];

    /// Stable key, also used for translation ids.
    pub fn key(self) -> &'static str {
        match self {
            NutrientGroup::Energy => "energy",
            NutrientGroup::Macronutrients => "macronutrients",
            NutrientGroup::Carbohydrates => "carbohydrates",
            NutrientGroup::Fats => "fats",
            NutrientGroup::Minerals => "minerals",
            NutrientGroup::TraceElements => "trace-elements",
            NutrientGroup::Vitamins => "vitamins",
            NutrientGroup::Other => "other",
        }
    }
}

macro_rules! nutrients {
    ($($variant:ident => $key:literal, $group:ident, $unit:ident;)*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum Nutrient {
            $($variant,)*
        }

        impl Nutrient {
            pub const ALL: &'static [Nutrient] = &[$(Nutrient::$variant,)*];

            /// Stable key, used in rules, storage and translation ids.
            pub fn key(self) -> &'static str {
                match self {
                    $(Nutrient::$variant => $key,)*
                }
            }

            pub fn group(self) -> NutrientGroup {
                match self {
                    $(Nutrient::$variant => NutrientGroup::$group,)*
                }
            }

            /// Unit used for display.
            pub fn unit(self) -> Unit {
                match self {
                    $(Nutrient::$variant => Unit::$unit,)*
                }
            }

            pub fn from_key(key: &str) -> Option<Nutrient> {
                match key {
                    $($key => Some(Nutrient::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

nutrients! {
    Energy => "energy", Energy, Kilocalorie;
    Protein => "protein", Macronutrients, Gram;
    Fat => "fat", Macronutrients, Gram;
    Carbohydrate => "carbohydrate", Macronutrients, Gram;
    Fibre => "fibre", Macronutrients, Gram;
    Water => "water", Macronutrients, Gram;
    Alcohol => "alcohol", Macronutrients, Gram;
    Sugars => "sugars", Carbohydrates, Gram;
    Starch => "starch", Carbohydrates, Gram;
    SaturatedFat => "saturated-fat", Fats, Gram;
    MonounsaturatedFat => "monounsaturated-fat", Fats, Gram;
    PolyunsaturatedFat => "polyunsaturated-fat", Fats, Gram;
    Omega3 => "omega-3", Fats, Gram;
    Omega6 => "omega-6", Fats, Gram;
    Cholesterol => "cholesterol", Fats, Milligram;
    Salt => "salt", Minerals, Gram;
    Sodium => "sodium", Minerals, Milligram;
    Potassium => "potassium", Minerals, Milligram;
    Calcium => "calcium", Minerals, Milligram;
    Magnesium => "magnesium", Minerals, Milligram;
    Phosphorus => "phosphorus", Minerals, Milligram;
    Iron => "iron", TraceElements, Milligram;
    Zinc => "zinc", TraceElements, Milligram;
    Iodine => "iodine", TraceElements, Microgram;
    Selenium => "selenium", TraceElements, Microgram;
    VitaminA => "vitamin-a", Vitamins, Microgram;
    VitaminD => "vitamin-d", Vitamins, Microgram;
    VitaminE => "vitamin-e", Vitamins, Milligram;
    VitaminK => "vitamin-k", Vitamins, Microgram;
    VitaminC => "vitamin-c", Vitamins, Milligram;
    VitaminB1 => "vitamin-b1", Vitamins, Milligram;
    VitaminB2 => "vitamin-b2", Vitamins, Milligram;
    Niacin => "niacin", Vitamins, Milligram;
    VitaminB6 => "vitamin-b6", Vitamins, Milligram;
    Folate => "folate", Vitamins, Microgram;
    VitaminB12 => "vitamin-b12", Vitamins, Microgram;
    Purine => "purine", Other, Milligram;
    Oxalate => "oxalate", Other, Milligram;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_round_trip_and_are_unique() {
        let mut keys = std::collections::BTreeSet::new();
        for &n in Nutrient::ALL {
            assert_eq!(Nutrient::from_key(n.key()), Some(n));
            assert!(keys.insert(n.key()), "duplicate key {}", n.key());
        }
    }

    #[test]
    fn every_group_has_nutrients() {
        for group in NutrientGroup::ALL {
            assert!(
                Nutrient::ALL.iter().any(|n| n.group() == group),
                "{group:?}"
            );
        }
    }
}
