//! 11 Alle Schätzungen: every value the app estimated, filterable, as CSV.

use dioxus::prelude::*;
use healthyteller_core::Derivation;

use crate::Route;
use crate::components::{Card, Notice, NoticeKind, TopBar};
use crate::format;
use crate::i18n::tr;
use crate::labels;
use crate::state::use_app_state;

#[component]
pub fn Estimates() -> Element {
    let state = use_app_state();
    let mut filter = use_signal(String::new);
    let s = state.read();
    let rows: Vec<(String, String, String, String)> = s
        .foods
        .values()
        .flat_map(|c| {
            c.food
                .nutrients
                .iter()
                .filter_map(move |(n, v)| match &v.derivation {
                    Derivation::Estimated { basis, .. } => Some((
                        c.food.name.clone(),
                        labels::nutrient(*n),
                        format::quantity(v.quantity),
                        basis.clone(),
                    )),
                    _ => None,
                })
        })
        .collect();
    drop(s);
    let f = filter().to_lowercase();
    let shown: Vec<_> = rows
        .iter()
        .filter(|r| {
            f.is_empty() || r.0.to_lowercase().contains(&f) || r.1.to_lowercase().contains(&f)
        })
        .cloned()
        .collect();
    let csv = {
        let mut out = String::from("food;nutrient;value_per_100g;basis\n");
        for (food, n, v, b) in &rows {
            out.push_str(&format!("{food};{n};{v};{b}\n").replace('"', "'"));
        }
        out
    };
    let href = format!("data:text/csv;charset=utf-8,{}", encode(&csv));

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Settings {}, title: tr("estimates-all") }
            h1 { class: "title", {tr("estimates-title")} }
            Notice { kind: NoticeKind::Estimates, class: "notice-warn" }
            div { class: "field",
                label { r#for: "estimate-filter", {tr("estimates-filter")} }
                input { id: "estimate-filter", class: "input", value: "{filter}", oninput: move |e| filter.set(e.value()) }
            }
            Card { title: tr("estimates-list"),
                if shown.is_empty() {
                    p { class: "card-body muted", {tr("estimates-empty")} }
                }
                ul { class: "list",
                    for (i, (food, n, v, b)) in shown.into_iter().enumerate() {
                        li { key: "{i}", class: "card-row", style: "flex-direction: column; align-items: flex-start;",
                            span { class: "food-name", "{food}: {n} ≈ {v}" }
                            span { class: "small muted", "{b}" }
                        }
                    }
                }
            }
            if rows.is_empty() {
                button { class: "btn btn-secondary btn-large", disabled: true, {tr("estimates-csv")} }
            } else {
                a { class: "btn btn-secondary btn-large", href: "{href}", download: "schaetzungen.csv", {tr("estimates-csv")} }
            }
        }
    }
}

/// Percent-encoding for a data URL.
fn encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~;".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
