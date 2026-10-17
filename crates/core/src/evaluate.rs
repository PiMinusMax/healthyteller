//! Evaluates one food and portion against a profile, one result per dimension.

use serde::{Deserialize, Serialize};

use crate::allergen::Allergen;
use crate::food::{Food, Portion};
use crate::nutrient::Nutrient;
use crate::plate::DayTotals;
use crate::profile::{Condition, DoctorLimit, MedicationGroup, Profile};
use crate::quantity::Quantity;
use crate::rule::{Dimension, Evidence, LimitKind, Rule, RuleBasis, RuleStatus};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Ok,
    Caution,
    Avoid,
    Unknown,
}

impl Level {
    pub fn key(self) -> &'static str {
        match self {
            Level::Ok => "ok",
            Level::Caution => "caution",
            Level::Avoid => "avoid",
            Level::Unknown => "unknown",
        }
    }

    fn priority(self) -> u8 {
        match self {
            Level::Avoid => 0,
            Level::Caution => 1,
            Level::Unknown => 2,
            Level::Ok => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LimitOrigin {
    Rule { id: String },
    Doctor,
}

/// The limit a value was compared with, and how much of it is used.
#[derive(Clone, Debug, PartialEq)]
pub struct Budget {
    pub limit: Quantity,
    pub basis: RuleBasis,
    /// Amount compared with the limit: this portion, plus the day so far for day limits.
    pub used: Quantity,
    pub origin: LimitOrigin,
}

impl Budget {
    pub fn used_fraction(&self) -> Option<f64> {
        self.used.ratio(self.limit)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Reason {
    AllergenFound(Vec<Allergen>),
    AllergenTraces(Vec<Allergen>),
    NoAllergenFound,
    AllergenDataMissing,
    WithinLimit,
    NearLimit,
    OverLimit,
    ValueUnknown,
    NoReviewedRule,
}

impl Reason {
    pub fn key(&self) -> &'static str {
        match self {
            Reason::AllergenFound(_) => "allergen-found",
            Reason::AllergenTraces(_) => "allergen-traces",
            Reason::NoAllergenFound => "no-allergen-found",
            Reason::AllergenDataMissing => "allergen-data-missing",
            Reason::WithinLimit => "within-limit",
            Reason::NearLimit => "near-limit",
            Reason::OverLimit => "over-limit",
            Reason::ValueUnknown => "value-unknown",
            Reason::NoReviewedRule => "no-reviewed-rule",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionResult {
    pub dimension: Dimension,
    /// Conditions that make this dimension relevant; empty for allergies.
    pub conditions: Vec<Condition>,
    /// Drug groups that make the medication dimension relevant.
    pub medications: Vec<MedicationGroup>,
    pub level: Level,
    pub nutrient: Option<Nutrient>,
    pub value_per_portion: Option<Quantity>,
    pub budget: Option<Budget>,
    pub reason: Reason,
    pub evidence: Option<Evidence>,
    pub sources: Vec<String>,
    /// Completeness of the underlying data between 0 and 1, if known.
    pub completeness: Option<f64>,
    pub rule_id: Option<String>,
    /// Based on a rule that has not been reviewed yet.
    pub draft: bool,
}

impl DimensionResult {
    fn new(dimension: Dimension, level: Level, reason: Reason) -> Self {
        Self {
            dimension,
            conditions: Vec::new(),
            medications: Vec::new(),
            level,
            nutrient: None,
            value_per_portion: None,
            budget: None,
            reason,
            evidence: None,
            sources: Vec::new(),
            completeness: None,
            rule_id: None,
            draft: false,
        }
    }
}

/// Two rules of the profile pull the same metric in opposite directions.
/// The app shows the conflict and leaves the decision to the doctor (AGENTS.md rule 7).
#[derive(Clone, Debug, PartialEq)]
pub struct Conflict {
    pub nutrient: Nutrient,
    pub upper: (Condition, String),
    pub lower: (Condition, String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Evaluation {
    /// Sorted: found allergens first, then avoid, caution, unknown, ok.
    pub results: Vec<DimensionResult>,
    pub conflicts: Vec<Conflict>,
}

impl Evaluation {
    /// Results that need attention: avoid and caution.
    pub fn attention(&self) -> impl Iterator<Item = &DimensionResult> {
        self.results
            .iter()
            .filter(|r| matches!(r.level, Level::Avoid | Level::Caution))
    }

    pub fn unknown(&self) -> impl Iterator<Item = &DimensionResult> {
        self.results.iter().filter(|r| r.level == Level::Unknown)
    }

    pub fn unremarkable(&self) -> impl Iterator<Item = &DimensionResult> {
        self.results.iter().filter(|r| r.level == Level::Ok)
    }

    /// Worst level of one dimension, for lists. Not an overall score.
    pub fn level_of(&self, dimension: Dimension) -> Option<Level> {
        self.results
            .iter()
            .filter(|r| r.dimension == dimension)
            .map(|r| r.level)
            .min_by_key(|l| l.priority())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Options {
    /// Also apply rules with status `draft`, marked as such.
    pub include_drafts: bool,
}

pub fn evaluate(
    food: &Food,
    portion: &Portion,
    profile: &Profile,
    rules: &[Rule],
    day: &DayTotals,
    options: Options,
) -> Evaluation {
    let mut results = Vec::new();

    if !profile.allergies.is_empty() {
        results.push(evaluate_allergens(food, profile));
    }

    let active: Vec<&Rule> = rules
        .iter()
        .filter(|r| profile.conditions.contains(&r.condition))
        .filter(|r| r.status == RuleStatus::Reviewed || options.include_drafts)
        .collect();

    let mut covered_by_doctor = Vec::new();
    for limit in &profile.doctor_limits {
        if let Some(result) = evaluate_doctor_limit(food, portion, profile, limit, day) {
            covered_by_doctor.push((limit.nutrient, limit.basis));
            results.push(result);
        }
    }

    for &condition in &profile.conditions {
        let dimension = condition.dimension();
        let mut any = false;
        for rule in active
            .iter()
            .filter(|r| r.condition == condition && r.limit == LimitKind::Upper)
        {
            any = true;
            if rule.user_overridable && covered_by_doctor.contains(&(rule.nutrient, rule.basis)) {
                continue;
            }
            results.push(evaluate_rule(food, portion, rule, day));
        }
        let has_doctor_limit = results
            .iter()
            .any(|r| r.dimension == dimension && r.conditions.contains(&condition));
        if !any && !has_doctor_limit {
            if let Some(existing) = results
                .iter_mut()
                .find(|r| r.dimension == dimension && r.reason == Reason::NoReviewedRule)
            {
                existing.conditions.push(condition);
            } else {
                let mut r = DimensionResult::new(dimension, Level::Unknown, Reason::NoReviewedRule);
                r.conditions.push(condition);
                results.push(r);
            }
        }
    }

    // No food-drug rule is reviewed yet: each group stays unknown instead of looking unremarkable.
    if !profile.medications.is_empty() {
        let mut r = DimensionResult::new(
            Dimension::Medication,
            Level::Unknown,
            Reason::NoReviewedRule,
        );
        r.medications = profile.medications.iter().copied().collect();
        results.push(r);
    }

    results.sort_by_key(|r| {
        (
            !matches!(r.reason, Reason::AllergenFound(_)),
            r.level.priority(),
        )
    });

    Evaluation {
        results,
        conflicts: find_conflicts(&active),
    }
}

fn evaluate_allergens(food: &Food, profile: &Profile) -> DimensionResult {
    let data = &food.allergens;
    let (level, reason) = if !data.known {
        (Level::Unknown, Reason::AllergenDataMissing)
    } else {
        let found: Vec<Allergen> = profile
            .allergies
            .intersection(&data.contains)
            .copied()
            .collect();
        let traces: Vec<Allergen> = profile
            .allergies
            .intersection(&data.traces)
            .copied()
            .collect();
        if !found.is_empty() {
            (Level::Avoid, Reason::AllergenFound(found))
        } else if !traces.is_empty() {
            (Level::Caution, Reason::AllergenTraces(traces))
        } else {
            (Level::Ok, Reason::NoAllergenFound)
        }
    };
    let mut r = DimensionResult::new(Dimension::Allergens, level, reason);
    r.completeness = data.completeness;
    r.sources.push(food.source.id.clone());
    r.evidence = Some(Evidence::High);
    r
}

/// Amount compared with a limit: the portion, plus the day so far for day limits.
fn compared_amount(
    food: &Food,
    portion: &Portion,
    nutrient: Nutrient,
    basis: RuleBasis,
    day: &DayTotals,
) -> Option<(Quantity, Quantity)> {
    let per_portion = food.amount_in(nutrient, portion)?;
    let used = match basis {
        RuleBasis::Portion => per_portion,
        RuleBasis::Day => match day.get(nutrient).and_then(|t| t.amount) {
            Some(before) => before.plus(per_portion)?,
            None => per_portion,
        },
        RuleBasis::Week => return None,
    };
    Some((per_portion, used))
}

fn level_for(
    used: Quantity,
    caution: Option<Quantity>,
    avoid: Option<Quantity>,
) -> (Level, Reason) {
    let reaches = |limit: Option<Quantity>| {
        limit
            .and_then(|l| used.ratio(l))
            .is_some_and(|ratio| ratio >= 1.0)
    };
    if reaches(avoid) {
        (Level::Avoid, Reason::OverLimit)
    } else if reaches(caution) {
        (Level::Caution, Reason::NearLimit)
    } else {
        (Level::Ok, Reason::WithinLimit)
    }
}

fn evaluate_rule(food: &Food, portion: &Portion, rule: &Rule, day: &DayTotals) -> DimensionResult {
    let mut r = match compared_amount(food, portion, rule.nutrient, rule.basis, day) {
        None => DimensionResult::new(rule.dimension, Level::Unknown, Reason::ValueUnknown),
        Some((per_portion, used)) => {
            let (level, reason) = level_for(used, rule.caution(), rule.avoid());
            let mut r = DimensionResult::new(rule.dimension, level, reason);
            r.value_per_portion = Some(per_portion);
            if let Some(limit) = rule.avoid() {
                r.budget = Some(Budget {
                    limit,
                    basis: rule.basis,
                    used,
                    origin: LimitOrigin::Rule {
                        id: rule.id.clone(),
                    },
                });
            }
            r
        }
    };
    r.conditions.push(rule.condition);
    r.nutrient = Some(rule.nutrient);
    r.evidence = Some(rule.evidence);
    r.sources = rule.sources.clone();
    r.rule_id = Some(rule.id.clone());
    r.draft = rule.status == RuleStatus::Draft;
    r.completeness = food.nutrients.contains_key(&rule.nutrient).then_some(1.0);
    r
}

fn evaluate_doctor_limit(
    food: &Food,
    portion: &Portion,
    profile: &Profile,
    limit: &DoctorLimit,
    day: &DayTotals,
) -> Option<DimensionResult> {
    let max = limit.max?;
    let dimension = Dimension::for_nutrient(limit.nutrient);
    let mut r = match compared_amount(food, portion, limit.nutrient, limit.basis, day) {
        None => DimensionResult::new(dimension, Level::Unknown, Reason::ValueUnknown),
        Some((per_portion, used)) => {
            // The app only adopts the doctor's maximum; it never derives a caution level from it.
            let (level, reason) = level_for(used, None, Some(max));
            let mut r = DimensionResult::new(dimension, level, reason);
            r.value_per_portion = Some(per_portion);
            r.budget = Some(Budget {
                limit: max,
                basis: limit.basis,
                used,
                origin: LimitOrigin::Doctor,
            });
            r
        }
    };
    r.nutrient = Some(limit.nutrient);
    r.conditions = profile
        .conditions
        .iter()
        .copied()
        .filter(|c| c.dimension() == dimension)
        .collect();
    r.sources.push(food.source.id.clone());
    Some(r)
}

fn find_conflicts(rules: &[&Rule]) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    for upper in rules.iter().filter(|r| r.limit == LimitKind::Upper) {
        for lower in rules
            .iter()
            .filter(|r| r.limit == LimitKind::Lower && r.nutrient == upper.nutrient)
        {
            if upper.condition != lower.condition {
                conflicts.push(Conflict {
                    nutrient: upper.nutrient,
                    upper: (upper.condition, upper.id.clone()),
                    lower: (lower.condition, lower.id.clone()),
                });
            }
        }
    }
    conflicts
}

#[cfg(test)]
mod tests {
    //! All thresholds and values here are invented fixtures (AGENTS.md rule 1).
    use super::*;
    use crate::food::Preparation;
    use crate::food::tests::fixture_food;
    use crate::quantity::Unit;
    use crate::rule::Thresholds;

    fn portion(grams: f64) -> Portion {
        Portion {
            grams,
            preparation: Preparation::Raw,
        }
    }

    fn rule(
        condition: Condition,
        nutrient: Nutrient,
        basis: RuleBasis,
        caution: f64,
        avoid: f64,
    ) -> Rule {
        Rule {
            id: format!("fixture.{}", nutrient.key()),
            version: 1,
            status: RuleStatus::Reviewed,
            condition,
            dimension: condition.dimension(),
            nutrient,
            unit: Unit::Milligram,
            basis,
            limit: LimitKind::Upper,
            thresholds: Thresholds {
                caution: Some(caution),
                avoid: Some(avoid),
            },
            user_overridable: true,
            evidence: Evidence::High,
            sources: vec!["fixture-source".into()],
            message_key: "fixture".into(),
        }
    }

    fn profile_with(condition: Condition) -> Profile {
        let mut p = Profile::new("t", "Test");
        p.conditions.insert(condition);
        p
    }

    #[test]
    fn portion_rule_levels_inside_and_outside_thresholds() {
        let r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Portion,
            100.0,
            200.0,
        );
        let p = profile_with(Condition::Gout);
        let food = fixture_food(&[(Nutrient::Purine, 100.0, Unit::Milligram)]);
        let day = DayTotals::new();
        let level = |g| {
            evaluate(
                &food,
                &portion(g),
                &p,
                std::slice::from_ref(&r),
                &day,
                Options::default(),
            )
            .results[0]
                .level
        };
        assert_eq!(level(50.0), Level::Ok);
        assert_eq!(level(100.0), Level::Caution);
        assert_eq!(level(250.0), Level::Avoid);
    }

    #[test]
    fn day_rule_adds_what_was_eaten_before() {
        let r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Day,
            100.0,
            200.0,
        );
        let p = profile_with(Condition::Gout);
        let food = fixture_food(&[(Nutrient::Purine, 100.0, Unit::Milligram)]);
        let mut day = DayTotals::new();
        let fresh = evaluate(
            &food,
            &portion(80.0),
            &p,
            std::slice::from_ref(&r),
            &day,
            Options::default(),
        );
        assert_eq!(fresh.results[0].level, Level::Ok);
        day.add(&food, &portion(150.0));
        let later = evaluate(
            &food,
            &portion(80.0),
            &p,
            std::slice::from_ref(&r),
            &day,
            Options::default(),
        );
        assert_eq!(later.results[0].level, Level::Avoid);
        let budget = later.results[0].budget.as_ref().unwrap();
        assert!((budget.used.amount - 230.0).abs() < 1e-9);
    }

    #[test]
    fn missing_value_gives_unknown_not_ok() {
        let r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Portion,
            100.0,
            200.0,
        );
        let p = profile_with(Condition::Gout);
        let food = fixture_food(&[]);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[r],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results[0].level, Level::Unknown);
        assert_eq!(e.results[0].reason, Reason::ValueUnknown);
    }

    #[test]
    fn draft_rules_only_on_request() {
        let mut r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Portion,
            100.0,
            200.0,
        );
        r.status = RuleStatus::Draft;
        let p = profile_with(Condition::Gout);
        let food = fixture_food(&[(Nutrient::Purine, 500.0, Unit::Milligram)]);
        let day = DayTotals::new();
        let rules = std::slice::from_ref(&r);
        let without = evaluate(&food, &portion(100.0), &p, rules, &day, Options::default());
        assert_eq!(without.results[0].reason, Reason::NoReviewedRule);
        let with = evaluate(
            &food,
            &portion(100.0),
            &p,
            rules,
            &day,
            Options {
                include_drafts: true,
            },
        );
        assert_eq!(with.results[0].level, Level::Avoid);
        assert!(with.results[0].draft);
    }

    #[test]
    fn found_allergen_comes_first_and_is_avoid() {
        let r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Portion,
            100.0,
            200.0,
        );
        let mut p = profile_with(Condition::Gout);
        p.allergies.insert(Allergen::Eggs);
        let mut food = fixture_food(&[(Nutrient::Purine, 500.0, Unit::Milligram)]);
        food.allergens.known = true;
        food.allergens.contains.insert(Allergen::Eggs);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[r],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results[0].dimension, Dimension::Allergens);
        assert_eq!(e.results[0].level, Level::Avoid);
        assert_eq!(e.results[1].level, Level::Avoid);
    }

    #[test]
    fn allergen_traces_and_missing_data() {
        let mut p = Profile::new("t", "Test");
        p.allergies.insert(Allergen::Milk);
        let mut food = fixture_food(&[]);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results[0].reason, Reason::AllergenDataMissing);
        food.allergens.known = true;
        food.allergens.traces.insert(Allergen::Milk);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results[0].level, Level::Caution);
        food.allergens.traces.clear();
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results[0].reason, Reason::NoAllergenFound);
    }

    #[test]
    fn condition_without_rule_is_unknown() {
        let mut p = profile_with(Condition::Diabetes);
        p.conditions.insert(Condition::Gout);
        let food = fixture_food(&[]);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results.len(), 2);
        assert!(e.results.iter().all(|r| r.reason == Reason::NoReviewedRule));
    }

    #[test]
    fn medication_group_without_rule_is_unknown() {
        let mut p = Profile::new("p", "Test");
        p.medications.insert(MedicationGroup::Statin);
        p.medications.insert(MedicationGroup::VitaminKAntagonist);
        let e = evaluate(
            &fixture_food(&[]),
            &portion(100.0),
            &p,
            &[],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results.len(), 1);
        let r = &e.results[0];
        assert_eq!(r.dimension, Dimension::Medication);
        assert_eq!(r.level, Level::Unknown);
        assert_eq!(r.reason, Reason::NoReviewedRule);
        assert_eq!(
            r.medications,
            vec![MedicationGroup::VitaminKAntagonist, MedicationGroup::Statin]
        );
    }

    #[test]
    fn doctor_limit_replaces_overridable_rule() {
        let r = rule(
            Condition::Gout,
            Nutrient::Purine,
            RuleBasis::Portion,
            100.0,
            200.0,
        );
        let mut p = profile_with(Condition::Gout);
        p.doctor_limits.push(DoctorLimit {
            nutrient: Nutrient::Purine,
            max: Some(Quantity::new(400.0, Unit::Milligram)),
            min: None,
            basis: RuleBasis::Portion,
            note: None,
        });
        let food = fixture_food(&[(Nutrient::Purine, 300.0, Unit::Milligram)]);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[r],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.results.len(), 1);
        assert_eq!(e.results[0].level, Level::Ok);
        assert_eq!(
            e.results[0].budget.as_ref().unwrap().origin,
            LimitOrigin::Doctor
        );
        assert_eq!(e.results[0].conditions, vec![Condition::Gout]);
    }

    #[test]
    fn opposite_rules_on_same_metric_are_a_conflict() {
        let upper = rule(
            Condition::KidneyDisease,
            Nutrient::Potassium,
            RuleBasis::Day,
            1.0,
            2.0,
        );
        let mut lower = rule(
            Condition::Hypertension,
            Nutrient::Potassium,
            RuleBasis::Day,
            1.0,
            2.0,
        );
        lower.limit = LimitKind::Lower;
        lower.id = "fixture.lower".into();
        let mut p = profile_with(Condition::KidneyDisease);
        p.conditions.insert(Condition::Hypertension);
        let food = fixture_food(&[]);
        let e = evaluate(
            &food,
            &portion(100.0),
            &p,
            &[upper, lower],
            &DayTotals::new(),
            Options::default(),
        );
        assert_eq!(e.conflicts.len(), 1);
        assert_eq!(e.conflicts[0].upper.0, Condition::KidneyDisease);
        assert_eq!(e.conflicts[0].lower.0, Condition::Hypertension);
    }
}
