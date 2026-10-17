//! Rules: one threshold for one metric under one condition. Values only from cited sources
//! (AGENTS.md rule 2); only reviewed rules are applied by default (rule 11).

use serde::{Deserialize, Serialize};

use crate::nutrient::Nutrient;
use crate::profile::Condition;
use crate::quantity::{Quantity, Unit};

/// Evaluation dimensions. Each keeps its own result; there is no overall score (rule 7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Dimension {
    Allergens,
    BloodSugar,
    Purine,
    Kidney,
    BloodPressure,
    Liver,
    Histamine,
    Oxalate,
    Processing,
    Cancer,
    Preparation,
    Medication,
    Basics,
}

impl Dimension {
    pub fn key(self) -> &'static str {
        match self {
            Dimension::Allergens => "allergens",
            Dimension::BloodSugar => "blood-sugar",
            Dimension::Purine => "purine",
            Dimension::Kidney => "kidney",
            Dimension::BloodPressure => "blood-pressure",
            Dimension::Liver => "liver",
            Dimension::Histamine => "histamine",
            Dimension::Oxalate => "oxalate",
            Dimension::Processing => "processing",
            Dimension::Cancer => "cancer",
            Dimension::Preparation => "preparation",
            Dimension::Medication => "medication",
            Dimension::Basics => "basics",
        }
    }

    /// Dimension a doctor's limit on this nutrient belongs to.
    pub fn for_nutrient(nutrient: Nutrient) -> Dimension {
        match nutrient {
            Nutrient::Purine => Dimension::Purine,
            Nutrient::Salt | Nutrient::Sodium => Dimension::BloodPressure,
            Nutrient::Sugars | Nutrient::Carbohydrate | Nutrient::Starch => Dimension::BloodSugar,
            Nutrient::Potassium | Nutrient::Phosphorus | Nutrient::Protein => Dimension::Kidney,
            Nutrient::Oxalate => Dimension::Oxalate,
            Nutrient::Alcohol => Dimension::Liver,
            _ => Dimension::Basics,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    Draft,
    Reviewed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleBasis {
    Portion,
    Day,
    Week,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    High,
    Medium,
    Low,
}

/// Whether the threshold is a maximum (less is better) or a minimum.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitKind {
    #[default]
    Upper,
    Lower,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Thresholds {
    pub caution: Option<f64>,
    pub avoid: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub version: u32,
    pub status: RuleStatus,
    #[serde(rename = "profile")]
    pub condition: Condition,
    pub dimension: Dimension,
    #[serde(rename = "metric")]
    pub nutrient: Nutrient,
    /// Unit of the thresholds.
    pub unit: Unit,
    pub basis: RuleBasis,
    #[serde(default)]
    pub limit: LimitKind,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub user_overridable: bool,
    pub evidence: Evidence,
    pub sources: Vec<String>,
    pub message_key: String,
}

impl Rule {
    pub fn caution(&self) -> Option<Quantity> {
        self.thresholds.caution.map(|v| Quantity::new(v, self.unit))
    }

    pub fn avoid(&self) -> Option<Quantity> {
        self.thresholds.avoid.map(|v| Quantity::new(v, self.unit))
    }
}
