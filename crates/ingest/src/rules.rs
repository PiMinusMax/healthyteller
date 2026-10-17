//! Loads rules from `rules/*.yaml` (format in docs/auftrag.md) and checks them.

use std::collections::BTreeMap;

use healthyteller_core::{
    AllergenData, DayTotals, Derivation, Food, FoodId, FoodKind, Nutrient, Options, Portion,
    Preparation, Profile, Quantity, Rule, RuleStatus, SourceRef, SourcedValue, Unit, ValueBasis,
    evaluate,
};
use serde::Deserialize;

/// Reference case that belongs to a rule (AGENTS.md rule 11).
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RuleTest {
    pub fixture_food: String,
    pub portion_g: f64,
    pub expect: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RuleFile {
    #[serde(flatten)]
    pub rule: Rule,
    #[serde(default)]
    pub tests: Vec<RuleTest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleError {
    pub rule_id: String,
    pub message: String,
}

pub fn parse_rule(yaml: &str) -> Result<RuleFile, String> {
    serde_norway::from_str(yaml).map_err(|e| e.to_string())
}

/// Checks what the format cannot express: sources, thresholds and reference cases.
pub fn validate(file: &RuleFile) -> Vec<RuleError> {
    let rule = &file.rule;
    let mut errors = Vec::new();
    let mut fail = |message: &str| {
        errors.push(RuleError {
            rule_id: rule.id.clone(),
            message: message.to_owned(),
        })
    };
    if rule.sources.is_empty() {
        fail("every rule needs at least one source");
    }
    if rule.message_key.trim().is_empty() {
        fail("message_key is empty");
    }
    match (rule.thresholds.caution, rule.thresholds.avoid) {
        (None, None) => fail("rule has no threshold"),
        (Some(c), Some(a)) if c > a => fail("caution threshold is above avoid threshold"),
        _ => {}
    }
    let expects: Vec<&str> = file.tests.iter().map(|t| t.expect.as_str()).collect();
    if !expects.contains(&"ok") || !expects.iter().any(|e| *e == "caution" || *e == "avoid") {
        fail("needs reference cases inside and outside the threshold");
    }
    if rule.status == RuleStatus::Reviewed && file.tests.is_empty() {
        fail("reviewed rule without reference cases");
    }
    errors
}

/// A citable work a rule threshold comes from (AGENTS.md rule 2): one file per work
/// under `rules/citations/`. Holds the bibliographic data only, never copied tables.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Citation {
    pub id: String,
    pub title: String,
    pub publisher: String,
    pub year: u16,
    /// Edition, version or issue the threshold was taken from.
    pub edition: String,
    pub url: String,
    /// ISO 8601 date the work was retrieved.
    pub retrieved: String,
    /// Page, table or section of the threshold.
    pub locator: String,
}

pub fn parse_citation(yaml: &str) -> Result<Citation, String> {
    serde_norway::from_str(yaml).map_err(|e| e.to_string())
}

/// Every source a rule names must be a known citation with a retrieval date.
pub fn check_citations(file: &RuleFile, citations: &[Citation]) -> Vec<RuleError> {
    let rule = &file.rule;
    let mut errors = Vec::new();
    for id in &rule.sources {
        match citations.iter().find(|c| &c.id == id) {
            None => errors.push(RuleError {
                rule_id: rule.id.clone(),
                message: format!("unknown citation {id}"),
            }),
            Some(c) if c.retrieved.trim().is_empty() || c.url.trim().is_empty() => {
                errors.push(RuleError {
                    rule_id: rule.id.clone(),
                    message: format!("citation {id} needs a url and a retrieval date"),
                })
            }
            Some(_) => {}
        }
    }
    errors
}

/// Food with invented values for the reference cases of a rule (`fixtures/rules/`).
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct FixtureFood {
    pub id: String,
    pub name: String,
    pub per_100g: BTreeMap<Nutrient, FixtureValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct FixtureValue {
    pub amount: f64,
    pub unit: Unit,
}

impl FixtureFood {
    pub fn parse(yaml: &str) -> Result<FixtureFood, String> {
        serde_norway::from_str(yaml).map_err(|e| e.to_string())
    }

    pub fn to_food(&self) -> Food {
        let source = SourceRef {
            id: "fixture".into(),
            version: "1".into(),
            retrieved_at: "2026-01-01".into(),
        };
        Food {
            id: FoodId(format!("fixture:{}", self.id)),
            name: self.name.clone(),
            brand: None,
            kind: FoodKind::Generic,
            nutrients: self
                .per_100g
                .iter()
                .map(|(&n, v)| {
                    (
                        n,
                        SourcedValue {
                            quantity: Quantity::new(v.amount, v.unit),
                            basis: ValueBasis::Per100gAsSold,
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
}

/// Runs the reference cases of a rule against the core evaluation (AGENTS.md rule 11).
/// Each case gets a fresh profile with only the rule's condition and an empty day.
pub fn check_reference_cases(file: &RuleFile, foods: &[FixtureFood]) -> Vec<RuleError> {
    let rule = &file.rule;
    let fail = |message: String| RuleError {
        rule_id: rule.id.clone(),
        message,
    };
    let mut profile = Profile::new("fixture", "Fixture");
    profile.conditions.insert(rule.condition);
    let rules = [rule.clone()];
    let mut errors = Vec::new();
    for case in &file.tests {
        let Some(food) = foods.iter().find(|f| f.id == case.fixture_food) else {
            errors.push(fail(format!("unknown fixture food {}", case.fixture_food)));
            continue;
        };
        let portion = Portion {
            grams: case.portion_g,
            preparation: Preparation::Raw,
        };
        let evaluation = evaluate(
            &food.to_food(),
            &portion,
            &profile,
            &rules,
            &DayTotals::new(),
            Options {
                include_drafts: true,
            },
        );
        let level = evaluation
            .results
            .iter()
            .find(|r| r.rule_id.as_deref() == Some(rule.id.as_str()))
            .map(|r| r.level.key());
        if level != Some(case.expect.as_str()) {
            errors.push(fail(format!(
                "{} at {} g: expected {}, got {}",
                case.fixture_food,
                case.portion_g,
                case.expect,
                level.unwrap_or("no result")
            )));
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use healthyteller_core::{Condition, Nutrient, RuleBasis, Unit};

    // Invented thresholds, format check only.
    const FIXTURE: &str = r#"
id: fixture.purine.daily_budget
version: 1
status: draft
profile: gout
dimension: purine
metric: purine
unit: mg
basis: day
thresholds:
  caution: 10
  avoid: 20
user_overridable: true
evidence: high
sources: [fixture-source]
message_key: rules-fixture
tests:
  - fixture_food: fixture-a
    portion_g: 100
    expect: avoid
  - fixture_food: fixture-b
    portion_g: 100
    expect: ok
"#;

    #[test]
    fn parses_auftrag_format() {
        let file = parse_rule(FIXTURE).unwrap();
        assert_eq!(file.rule.condition, Condition::Gout);
        assert_eq!(file.rule.nutrient, Nutrient::Purine);
        assert_eq!(file.rule.unit, Unit::Milligram);
        assert_eq!(file.rule.basis, RuleBasis::Day);
        assert_eq!(file.tests.len(), 2);
        assert!(validate(&file).is_empty());
    }

    #[test]
    fn rejects_rule_without_sources_and_cases() {
        let yaml = FIXTURE
            .replace("sources: [fixture-source]", "sources: []")
            .replace("expect: ok", "expect: avoid");
        let errors = validate(&parse_rule(&yaml).unwrap());
        assert_eq!(errors.len(), 2, "{errors:?}");
    }

    fn fixture_foods() -> Vec<FixtureFood> {
        [
            "id: fixture-a\nname: Fixture A\nper_100g:\n  purine: { amount: 30, unit: mg }\n",
            "id: fixture-b\nname: Fixture B\nper_100g:\n  purine: { amount: 5, unit: mg }\n",
        ]
        .iter()
        .map(|y| FixtureFood::parse(y).unwrap())
        .collect()
    }

    #[test]
    fn reference_cases_run_against_the_evaluation() {
        let file = parse_rule(FIXTURE).unwrap();
        assert!(check_reference_cases(&file, &fixture_foods()).is_empty());
        let wrong = parse_rule(&FIXTURE.replace("expect: ok", "expect: caution")).unwrap();
        let errors = check_reference_cases(&wrong, &fixture_foods());
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].message.contains("expected caution, got ok"));
        let missing = check_reference_cases(&file, &[]);
        assert_eq!(missing.len(), 2);
    }

    #[test]
    fn rule_sources_must_be_citations() {
        let file = parse_rule(FIXTURE).unwrap();
        assert_eq!(check_citations(&file, &[]).len(), 1);
        let citation = parse_citation(
            "id: fixture-source\ntitle: Fixture\npublisher: Nobody\nyear: 2026\nedition: '1'\nurl: https://example.org\nretrieved: 2026-10-17\nlocator: p. 1\n",
        )
        .unwrap();
        assert!(check_citations(&file, &[citation]).is_empty());
    }

    fn read_dir_yaml(dir: &str) -> Vec<(std::path::PathBuf, String)> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "yaml") {
                let text = std::fs::read_to_string(&path).unwrap();
                out.push((path, text));
            }
        }
        out
    }

    /// Every rule in `rules/` names known citations and passes its reference cases.
    #[test]
    fn rules_pass_citations_and_reference_cases() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let citations: Vec<Citation> = read_dir_yaml(&format!("{root}/rules/citations"))
            .iter()
            .map(|(p, t)| parse_citation(t).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
            .collect();
        for (path, _) in read_dir_yaml(&format!("{root}/rules/citations")) {
            let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
            assert!(
                citations.iter().any(|c| c.id == stem),
                "{}: id must match the file name",
                path.display()
            );
        }
        let foods: Vec<FixtureFood> = read_dir_yaml(&format!("{root}/fixtures/rules"))
            .iter()
            .map(|(p, t)| FixtureFood::parse(t).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
            .collect();
        for (path, text) in read_dir_yaml(&format!("{root}/rules")) {
            let file = parse_rule(&text).unwrap();
            let mut errors = check_citations(&file, &citations);
            errors.extend(check_reference_cases(&file, &foods));
            assert!(errors.is_empty(), "{}: {errors:?}", path.display());
        }
    }

    #[test]
    fn rules_directory_is_valid() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../rules");
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "yaml") {
                let text = std::fs::read_to_string(&path).unwrap();
                let file = parse_rule(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                let errors = validate(&file);
                assert!(errors.is_empty(), "{}: {errors:?}", path.display());
            }
        }
    }
}
