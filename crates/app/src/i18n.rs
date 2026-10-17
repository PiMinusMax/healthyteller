//! UI texts come only from the Fluent files under `locales/` (AGENTS.md rule 16).

use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::{LanguageIdentifier, langid};

const DE: &str = include_str!("../../../locales/de/app.ftl");
#[cfg(test)]
const EN: &str = include_str!("../../../locales/en/app.ftl");

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
    static BUNDLE: FluentBundle<FluentResource> = bundle(langid!("de"), DE);
}

/// Looks up a message by id. A missing id is a bug, so it shows up visibly as the id itself.
pub fn tr(id: &str) -> String {
    BUNDLE.with(|bundle| {
        let Some(pattern) = bundle.get_message(id).and_then(|message| message.value()) else {
            return id.to_owned();
        };
        let mut errors = Vec::new();
        bundle
            .format_pattern(pattern, None, &mut errors)
            .into_owned()
    })
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

    #[test]
    fn unknown_id_falls_back_to_the_id() {
        assert_eq!(tr("does-not-exist"), "does-not-exist");
    }
}
