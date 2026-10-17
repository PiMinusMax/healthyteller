//! 09 Fachbegriff: technical terms in plain language.

use dioxus::prelude::*;

use crate::Route;
use crate::components::TopBar;
use crate::i18n::{has_message, tr};

pub const TERMS: [&str; 8] = [
    "purine",
    "glycemic-load",
    "nova",
    "traces",
    "histamine",
    "oxalate",
    "completeness",
    "estimate",
];

#[component]
pub fn Term(term: String) -> Element {
    let key = term;
    let known = TERMS.contains(&key.as_str()) && has_message(&format!("term-{key}-title"));
    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("term-topbar") }
            if known {
                h1 { class: "title", {tr(&format!("term-{key}-title"))} }
                p { style: "font-size: 20px;", {tr(&format!("term-{key}-text"))} }
            } else {
                h1 { class: "title", {tr("not-found-title")} }
            }
            button { class: "btn btn-secondary btn-large", onclick: move |_| navigator().go_back(), {tr("action-understood")} }
        }
    }
}
