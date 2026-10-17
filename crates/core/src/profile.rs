//! A person's conditions, allergies and doctor's limits. Stays on the device (AGENTS.md rule 4).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::allergen::Allergen;
use crate::nutrient::Nutrient;
use crate::quantity::Quantity;
use crate::rule::{Dimension, RuleBasis};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Condition {
    Diabetes,
    Gout,
    Hypertension,
    KidneyDisease,
    LiverValues,
    HistamineIntolerance,
    OxalateStones,
}

impl Condition {
    pub const ALL: [Condition; 7] = [
        Condition::Diabetes,
        Condition::Gout,
        Condition::Hypertension,
        Condition::KidneyDisease,
        Condition::LiverValues,
        Condition::HistamineIntolerance,
        Condition::OxalateStones,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Condition::Diabetes => "diabetes",
            Condition::Gout => "gout",
            Condition::Hypertension => "hypertension",
            Condition::KidneyDisease => "kidney-disease",
            Condition::LiverValues => "liver-values",
            Condition::HistamineIntolerance => "histamine-intolerance",
            Condition::OxalateStones => "oxalate-stones",
        }
    }

    /// The evaluation dimension this condition makes relevant.
    pub fn dimension(self) -> Dimension {
        match self {
            Condition::Diabetes => Dimension::BloodSugar,
            Condition::Gout => Dimension::Purine,
            Condition::Hypertension => Dimension::BloodPressure,
            Condition::KidneyDisease => Dimension::Kidney,
            Condition::LiverValues => Dimension::Liver,
            Condition::HistamineIntolerance => Dimension::Histamine,
            Condition::OxalateStones => Dimension::Oxalate,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Female,
    Male,
    Diverse,
}

/// Order of the plate overview. Never changes whether a warning is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Focus {
    Conditions,
    Nutrients,
}

/// A limit set by the person's doctor. Replaces the rule's default; the app never sets one itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoctorLimit {
    pub nutrient: Nutrient,
    pub max: Option<Quantity>,
    pub min: Option<Quantity>,
    pub basis: RuleBasis,
    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub age: Option<u8>,
    pub sex: Option<Sex>,
    pub conditions: BTreeSet<Condition>,
    pub allergies: BTreeSet<Allergen>,
    /// Set by the person; otherwise derived from the conditions.
    pub focus_override: Option<Focus>,
    pub doctor_limits: Vec<DoctorLimit>,
}

impl Profile {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            age: None,
            sex: None,
            conditions: BTreeSet::new(),
            allergies: BTreeSet::new(),
            focus_override: None,
            doctor_limits: Vec::new(),
        }
    }

    /// Conditions focus when the profile has at least one condition, nutrients otherwise.
    pub fn focus(&self) -> Focus {
        self.focus_override
            .unwrap_or(if self.conditions.is_empty() {
                Focus::Nutrients
            } else {
                Focus::Conditions
            })
    }

    pub fn doctor_limit(&self, nutrient: Nutrient, basis: RuleBasis) -> Option<&DoctorLimit> {
        self.doctor_limits
            .iter()
            .find(|l| l.nutrient == nutrient && l.basis == basis)
    }

    /// Up to two initials for the avatar.
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .flat_map(char::to_uppercase)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_follows_conditions_unless_overridden() {
        let mut p = Profile::new("1", "Test Person");
        assert_eq!(p.focus(), Focus::Nutrients);
        p.conditions.insert(Condition::Gout);
        assert_eq!(p.focus(), Focus::Conditions);
        p.focus_override = Some(Focus::Nutrients);
        assert_eq!(p.focus(), Focus::Nutrients);
    }

    #[test]
    fn initials() {
        assert_eq!(Profile::new("1", "erika mustermann").initials(), "EM");
        assert_eq!(Profile::new("1", "Max").initials(), "M");
        assert_eq!(Profile::new("1", "").initials(), "");
    }
}
