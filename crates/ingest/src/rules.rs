//! Loads rules from `rules/*.yaml` (format in docs/auftrag.md) and checks them.

use healthyteller_core::{Rule, RuleStatus};
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
