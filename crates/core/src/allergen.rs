//! The 14 allergens that must be declared in the EU (Regulation (EU) No 1169/2011, Annex II).

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Allergen {
    Gluten,
    Crustaceans,
    Eggs,
    Fish,
    Peanuts,
    Soybeans,
    Milk,
    Nuts,
    Celery,
    Mustard,
    Sesame,
    Sulphites,
    Lupin,
    Molluscs,
}

impl Allergen {
    pub const ALL: [Allergen; 14] = [
        Allergen::Gluten,
        Allergen::Crustaceans,
        Allergen::Eggs,
        Allergen::Fish,
        Allergen::Peanuts,
        Allergen::Soybeans,
        Allergen::Milk,
        Allergen::Nuts,
        Allergen::Celery,
        Allergen::Mustard,
        Allergen::Sesame,
        Allergen::Sulphites,
        Allergen::Lupin,
        Allergen::Molluscs,
    ];

    /// Stable key, also used for translation ids.
    pub fn key(self) -> &'static str {
        match self {
            Allergen::Gluten => "gluten",
            Allergen::Crustaceans => "crustaceans",
            Allergen::Eggs => "eggs",
            Allergen::Fish => "fish",
            Allergen::Peanuts => "peanuts",
            Allergen::Soybeans => "soybeans",
            Allergen::Milk => "milk",
            Allergen::Nuts => "nuts",
            Allergen::Celery => "celery",
            Allergen::Mustard => "mustard",
            Allergen::Sesame => "sesame",
            Allergen::Sulphites => "sulphites",
            Allergen::Lupin => "lupin",
            Allergen::Molluscs => "molluscs",
        }
    }
}
