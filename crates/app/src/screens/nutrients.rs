//! 10 Vollständige Nährwertliste: every nutrient per portion with its origin.

use dioxus::prelude::*;
use healthyteller_core::{Derivation, Nutrient, NutrientGroup};

use crate::Route;
use crate::components::{DataStatusNotice, Icon, IconName, Notice, NoticeKind, TopBar};
use crate::date;
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::screens::card::resolve_portion;
use crate::state::use_app_state;

#[derive(Clone, PartialEq)]
struct Row {
    name: String,
    value: String,
    derivation: Option<Derivation>,
}

#[component]
pub fn NutrientList(id: String, grams: u32, prep: String) -> Element {
    let state = use_app_state();
    let s = state.read();
    let Some(food) = s.food(&id).cloned() else {
        drop(s);
        return rsx! {
            main { class: "screen",
                TopBar { back: Route::Search {}, title: tr("nutrients-title") }
                h1 { class: "title", {tr("food-missing-title")} }
            }
        };
    };
    let portion = resolve_portion(&s, &id, grams, &prep);
    let fetched = s
        .foods
        .get(&id)
        .map(|c| c.fetched_at.clone())
        .unwrap_or_default();
    drop(s);

    // Known values in one table; unknown ones named in one line instead of rows of "unbekannt".
    let mut unknown: Vec<String> = Vec::new();
    let groups: Vec<(NutrientGroup, Vec<Row>)> = NutrientGroup::ALL
        .into_iter()
        .map(|g| {
            let rows = Nutrient::ALL
                .iter()
                .filter(|n| n.group() == g)
                .filter_map(|&n| {
                    let Some(value) = food.amount_in(n, &portion) else {
                        unknown.push(labels::nutrient(n));
                        return None;
                    };
                    Some(Row {
                        name: labels::nutrient(n),
                        value: format::quantity(value.to(n.unit()).unwrap_or(value)),
                        derivation: food.nutrients.get(&n).map(|v| v.derivation.clone()),
                    })
                })
                .collect::<Vec<_>>();
            (g, rows)
        })
        .filter(|(_, rows)| !rows.is_empty())
        .collect();
    let has_estimates = food.nutrients.values().any(|v| v.derivation.is_estimate());
    let source = labels::source_name(&food.source.id);

    rsx! {
        main { class: "screen",
            TopBar {
                back: Route::FoodCard { id: id.clone(), grams, prep: prep.clone() },
                title: format!("{} · {}", food.name, labels::portion(&portion)),
            }
            h1 { class: "title", {tr("nutrients-title")} }
            ul { class: "origin-legend compact", "aria-label": tr("nutrients-origin"),
                li { Mark { derivation: Some(Derivation::Measured) } span { "aria-hidden": "true", {tr("origin-measured")} } }
                li { Mark { derivation: Some(Derivation::Calculated) } span { "aria-hidden": "true", {tr("origin-calculated")} } }
                li { Mark { derivation: Some(Derivation::Interpolated) } span { "aria-hidden": "true", {tr("origin-interpolated")} } }
                li { Mark { derivation: Some(Derivation::Estimated { basis: String::new(), method: Default::default() }) } span { "aria-hidden": "true", {tr("origin-estimate")} } }
            }
            if !groups.is_empty() {
                section { class: "card", "aria-label": tr("nutrients-title"),
                    table { class: "nutrients",
                        thead {
                            tr {
                                th { {tr("nutrients-col-name")} }
                                th { class: "r", {tr("nutrients-col-portion")} }
                                th { class: "r", {tr("nutrients-col-origin")} }
                            }
                        }
                        for (group, rows) in groups {
                            tbody { key: "{group.key()}",
                                tr { th { colspan: 3, scope: "colgroup", {tr(&format!("group-{}", group.key()))} } }
                                for row in rows {
                                    tr { key: "{row.name}",
                                        td { "{row.name}" }
                                        td { class: "r num", "{row.value}" }
                                        td { class: "r", Mark { derivation: row.derivation.clone() } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !unknown.is_empty() {
                details { class: "card group", id: "nutrients-unknown",
                    summary {
                        span { {tr_args("nutrients-unknown", &[("count", unknown.len().to_string())])} }
                        Icon { name: IconName::ChevronDown, size: 20 }
                    }
                    p { class: "card-body", {unknown.join(", ")} }
                }
            }
            Notice { kind: NoticeKind::Variation }
            if has_estimates {
                Notice { kind: NoticeKind::Estimates, class: "notice-warn" }
                Link { class: "btn btn-secondary btn-large", to: Route::Estimates {}, {tr("estimates-all")} }
            }
            DataStatusNotice { source: source.clone(), date: date::display(&fetched) }
        }
    }
}

/// Origin mark: ● measured, ∑ calculated or declared, ◐ calculated from ingredients, ≈ estimate, ? unknown.
#[component]
fn Mark(derivation: Option<Derivation>) -> Element {
    let (class, symbol, label) = match &derivation {
        Some(Derivation::Measured) => ("mark-measured", "●", tr("origin-measured")),
        Some(Derivation::Calculated) => ("mark-calculated", "∑", tr("origin-calculated")),
        Some(Derivation::Declared) => ("mark-calculated", "∑", tr("origin-declared")),
        Some(Derivation::Interpolated) => ("mark-interpolated", "◐", tr("origin-interpolated")),
        Some(Derivation::Estimated { .. }) => ("mark-estimate", "≈", tr("origin-estimate")),
        None => ("mark-unknown", "?", tr("origin-unknown")),
    };
    rsx! {
        span { class: "mark {class}", title: "{label}",
            span { "aria-hidden": "true", "{symbol}" }
            span { class: "visually-hidden", "{label}" }
        }
    }
}
