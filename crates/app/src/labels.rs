//! Translated names for the core types.

use healthyteller_core::{
    Allergen, Condition, Dimension, DimensionResult, Evidence, Food, Nutrient, Portion,
    Preparation, Profile, RuleBasis,
};
use healthyteller_ingest::sources;

use crate::format;
use crate::i18n::{tr, tr_args};

pub fn nutrient(n: Nutrient) -> String {
    tr(&format!("nutrient-{}", n.key()))
}

pub fn condition(c: Condition) -> String {
    tr(&format!("condition-{}", c.key()))
}

pub fn allergen(a: Allergen) -> String {
    tr(&format!("allergen-{}", a.key()))
}

pub fn allergy(a: Allergen) -> String {
    tr(&format!("allergy-{}", a.key()))
}

pub fn dimension(d: Dimension) -> String {
    tr(&format!("dimension-{}", d.key()))
}

pub fn preparation(p: Preparation) -> String {
    tr(&format!("preparation-{}", p.key()))
}

pub fn evidence(e: Evidence) -> String {
    tr(match e {
        Evidence::High => "evidence-high",
        Evidence::Medium => "evidence-medium",
        Evidence::Low => "evidence-low",
    })
}

pub fn basis(b: RuleBasis) -> String {
    tr(match b {
        RuleBasis::Portion => "basis-portion",
        RuleBasis::Day => "basis-day",
        RuleBasis::Week => "basis-week",
    })
}

/// "150 g · gegrillt"
pub fn portion(p: &Portion) -> String {
    format!(
        "{} g · {}",
        format::number(p.grams),
        preparation(p.preparation)
    )
}

/// Conditions and allergies, e.g. "Diabetes, Gicht, Ei-Allergie".
pub fn profile_summary(p: &Profile) -> String {
    let mut parts: Vec<String> = p.conditions.iter().map(|&c| condition(c)).collect();
    parts.extend(p.allergies.iter().map(|&a| allergy(a)));
    if parts.is_empty() {
        tr("profile-no-conditions")
    } else {
        parts.join(", ")
    }
}

pub fn source_name(id: &str) -> String {
    sources::by_id(id).map_or_else(|| id.to_owned(), |s| s.name.to_owned())
}

/// Title of a result: the nutrient for rules and doctor's limits, otherwise the dimension.
pub fn result_title(r: &DimensionResult) -> String {
    match r.nutrient {
        Some(n) => nutrient(n),
        None => dimension(r.dimension),
    }
}

/// "wegen Gicht" or "wegen Ihrer Allergie".
pub fn result_why(r: &DimensionResult) -> String {
    if r.dimension == Dimension::Allergens {
        tr("why-allergy")
    } else if r.conditions.is_empty() {
        tr("why-doctor")
    } else {
        let names: Vec<String> = r.conditions.iter().map(|&c| condition(c)).collect();
        tr_args("why-conditions", &[("conditions", names.join(", "))])
    }
}

pub fn food_title(f: &Food) -> String {
    match &f.brand {
        Some(b) => format!("{} · {b}", f.name),
        None => f.name.clone(),
    }
}
