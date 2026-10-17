//! UI texts come only from the Fluent files under `locales/` (AGENTS.md rule 16).

use std::cell::Cell;

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use serde::{Deserialize, Serialize};
use unic_langid::{LanguageIdentifier, langid};

const DE: &str = include_str!("../../../locales/de/app.ftl");
const EN: &str = include_str!("../../../locales/en/app.ftl");

/// UI language. German comes first (AGENTS.md rule 16).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    De,
    En,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Language::De => "de",
            Language::En => "en",
        }
    }
}

fn bundle(lang: LanguageIdentifier, source: &str) -> FluentBundle<FluentResource> {
    let resource = FluentResource::try_new(source.to_owned())
        .unwrap_or_else(|(_, errors)| panic!("invalid Fluent file for {lang}: {errors:?}"));
    let mut bundle = FluentBundle::new(vec![lang]);
    // Fluent wraps placeables in Unicode isolation marks by default; they show up
    // as stray characters in plain text and in screen readers.
    bundle.set_use_isolating(false);
    bundle
        .add_resource(resource)
        .expect("duplicate message id in Fluent file");
    bundle
}

thread_local! {
    static BUNDLE_DE: FluentBundle<FluentResource> = bundle(langid!("de"), DE);
    static BUNDLE_EN: FluentBundle<FluentResource> = bundle(langid!("en"), EN);
    static CURRENT: Cell<Language> = const { Cell::new(Language::De) };
}

pub fn set_language(language: Language) {
    CURRENT.with(|c| c.set(language));
}

fn format(bundle: &FluentBundle<FluentResource>, id: &str, args: &FluentArgs) -> Option<String> {
    let pattern = bundle.get_message(id)?.value()?;
    let mut errors = Vec::new();
    Some(
        bundle
            .format_pattern(pattern, Some(args), &mut errors)
            .into_owned(),
    )
}

/// Looks up a message by id. A missing id is a bug, so it shows up visibly as the id itself.
pub fn tr(id: &str) -> String {
    tr_args(id, &[])
}

/// Looks up a message and fills in its variables, e.g. `{ $name }`.
/// Falls back to German when the current language lacks the id.
pub fn tr_args(id: &str, args: &[(&str, String)]) -> String {
    let mut fluent_args = FluentArgs::new();
    for (name, value) in args {
        fluent_args.set(*name, FluentValue::from(value.clone()));
    }
    let current = if CURRENT.with(Cell::get) == Language::En {
        BUNDLE_EN.with(|b| format(b, id, &fluent_args))
    } else {
        None
    };
    current
        .or_else(|| BUNDLE_DE.with(|b| format(b, id, &fluent_args)))
        .unwrap_or_else(|| id.to_owned())
}

/// True if the German texts define this id.
pub fn has_message(id: &str) -> bool {
    has(id)
}

fn has(id: &str) -> bool {
    BUNDLE_DE.with(|bundle| bundle.has_message(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message_ids(source: &str) -> Vec<String> {
        let resource = FluentResource::try_new(source.to_owned()).expect("valid Fluent file");
        let mut ids: Vec<String> = resource
            .entries()
            .filter_map(|entry| match entry {
                fluent_syntax::ast::Entry::Message(message) => Some(message.id.name.to_owned()),
                _ => None,
            })
            .collect();
        ids.sort();
        ids
    }

    #[test]
    fn german_and_english_define_the_same_messages() {
        assert_eq!(message_ids(DE), message_ids(EN));
    }

    #[test]
    fn known_id_is_translated() {
        assert_eq!(tr("app-title"), "Healthy Teller");
    }

    /// Literal ids passed to `tr`/`tr_args` anywhere in the app.
    fn literal_ids_in_sources() -> Vec<String> {
        fn walk(dir: &std::path::Path, out: &mut Vec<String>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let text = std::fs::read_to_string(&path).unwrap();
                    for start in ["tr(\"", "tr_args(\"", "(\""] {
                        for part in text.split(start).skip(1) {
                            let id: String = part.chars().take_while(|c| *c != '"').collect();
                            let looks_like_id = !id.is_empty()
                                && id.contains('-')
                                && id.chars().all(|c| {
                                    c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'
                                });
                            if looks_like_id && (start != "(\"" || has(&id)) {
                                out.push(id);
                            }
                        }
                    }
                }
            }
        }
        let mut ids = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut ids,
        );
        ids.retain(|id| id != "does-not-exist");
        ids.sort();
        ids.dedup();
        ids
    }

    #[test]
    fn every_literal_id_is_translated() {
        let ids = literal_ids_in_sources();
        assert!(ids.len() > 100, "scanner found only {} ids", ids.len());
        let missing: Vec<&String> = ids.iter().filter(|id| !has(id)).collect();
        assert!(
            missing.is_empty(),
            "missing in locales/de/app.ftl: {missing:?}"
        );
    }

    #[test]
    fn every_generated_id_is_translated() {
        use healthyteller_core::*;
        let mut ids: Vec<String> = Vec::new();
        ids.extend(
            Nutrient::ALL
                .iter()
                .map(|n| format!("nutrient-{}", n.key())),
        );
        ids.extend(
            NutrientGroup::ALL
                .iter()
                .map(|g| format!("group-{}", g.key())),
        );
        ids.extend(Allergen::ALL.iter().flat_map(|a| {
            [
                format!("allergen-{}", a.key()),
                format!("allergy-{}", a.key()),
            ]
        }));
        ids.extend(
            Condition::ALL
                .iter()
                .map(|c| format!("condition-{}", c.key())),
        );
        ids.extend(
            MedicationGroup::ALL
                .iter()
                .map(|m| format!("medication-{}", m.key())),
        );
        ids.extend(
            Preparation::ALL
                .iter()
                .map(|p| format!("preparation-{}", p.key())),
        );
        for level in [Level::Ok, Level::Caution, Level::Avoid, Level::Unknown] {
            ids.push(format!("level-{}", level.key()));
            ids.push(format!("print-group-{}", level.key()));
        }
        for d in [
            Dimension::Allergens,
            Dimension::BloodSugar,
            Dimension::Purine,
            Dimension::Kidney,
            Dimension::BloodPressure,
            Dimension::Liver,
            Dimension::Histamine,
            Dimension::Oxalate,
            Dimension::Processing,
            Dimension::Cancer,
            Dimension::Preparation,
            Dimension::Medication,
            Dimension::Basics,
        ] {
            ids.push(format!("dimension-{}", d.key()));
        }
        for r in [
            Reason::AllergenFound(vec![]),
            Reason::AllergenTraces(vec![]),
            Reason::NoAllergenFound,
            Reason::AllergenDataMissing,
            Reason::WithinLimit,
            Reason::NearLimit,
            Reason::OverLimit,
            Reason::ValueUnknown,
            Reason::NoReviewedRule,
        ] {
            ids.push(format!("reason-{}", r.key()));
        }
        for key in [
            "basic",
            "doctor",
            "variation",
            "estimates",
            "allergens",
            "data-status",
        ] {
            ids.push(format!("notice-{key}"));
            ids.push(format!("notice-{key}-title"));
        }
        for t in crate::screens::TERMS {
            ids.push(format!("term-{t}-title"));
            ids.push(format!("term-{t}-text"));
        }
        ids.extend((1..=3).map(|s| format!("profile-step-{s}")));
        ids.extend(["conflict", "missing", "traces"].map(|k| format!("questions-kind-{k}")));
        let missing: Vec<&String> = ids.iter().filter(|id| !has(id)).collect();
        assert!(
            missing.is_empty(),
            "missing in locales/de/app.ftl: {missing:?}"
        );
    }

    #[test]
    fn mandatory_notices_keep_their_wording() {
        // Fixed wording from docs/konzept.md, section "Hinweise in der App".
        assert!(
            tr("notice-basic")
                .starts_with("Healthy Teller hilft bei der Suche nach passenden Lebensmitteln")
        );
        assert!(tr("notice-allergens").ends_with("wählen Sie sofort den Notruf 112."));
        assert_eq!(
            tr_args(
                "notice-data-status",
                &[
                    ("source", "Open Food Facts".into()),
                    ("date", "01.01.2026".into())
                ]
            )
            .split(". ")
            .next(),
            Some("Die Werte stammen von Open Food Facts, Stand 01.01.2026")
        );
    }

    #[test]
    fn switches_language() {
        set_language(Language::En);
        assert_eq!(tr("settings-title"), "Settings");
        set_language(Language::De);
        assert_eq!(tr("settings-title"), "Einstellungen");
    }

    #[test]
    fn unknown_id_falls_back_to_the_id() {
        assert_eq!(tr("does-not-exist"), "does-not-exist");
    }
}
