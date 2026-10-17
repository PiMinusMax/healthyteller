//! 10 Vollständige Nährwertliste: every nutrient per portion with its origin.

use dioxus::prelude::*;
use healthyteller_core::{Derivation, Nutrient, NutrientGroup};

use crate::Route;
use crate::components::{Card, DataStatusNotice, Notice, NoticeKind, TopBar};
use crate::date;
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::screens::card::resolve_portion;
use crate::state::use_app_state;

#[derive(Clone, PartialEq)]
struct Row {
    name: String,
    value: Option<String>,
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

    let groups: Vec<(NutrientGroup, Vec<Row>)> = NutrientGroup::ALL
        .into_iter()
        .map(|g| {
            let rows = Nutrient::ALL
                .iter()
                .filter(|n| n.group() == g)
                .map(|&n| {
                    let v = food.nutrients.get(&n);
                    Row {
                        name: labels::nutrient(n),
                        value: food
                            .amount_in(n, &portion)
                            .map(|q| format::quantity(q.to(n.unit()).unwrap_or(q))),
                        derivation: v.map(|v| v.derivation.clone()),
                    }
                })
                .collect();
            (g, rows)
        })
        .collect();
    let has_estimates = food.nutrients.values().any(|v| v.derivation.is_estimate());
    let source = labels::source_name(&food.source.id);

    rsx! {
        main { class: "screen",
            TopBar {
                back: Route::FoodCard { id: id.clone(), grams, prep: prep.clone() },
                title: format!("{} · {}", food.name, labels::portion(&portion)),
            }
            div {
                h1 { class: "title", {tr("nutrients-title")} }
                p { class: "lead", {tr("nutrients-lead")} }
            }
            Card { title: tr("nutrients-origin"),
                ul { class: "card-body list", style: "display: grid; grid-template-columns: 1fr 1fr; gap: 10px;",
                    li { style: "border: none;", Mark { derivation: Some(Derivation::Measured) } " " {tr("origin-measured")} }
                    li { style: "border: none;", Mark { derivation: Some(Derivation::Calculated) } " " {tr("origin-calculated")} }
                    li { style: "border: none;", Mark { derivation: Some(Derivation::Estimated { basis: String::new() }) } " " {tr("origin-estimate")} }
                    li { style: "border: none;", Mark { derivation: None } " " {tr("origin-unknown")} }
                }
            }
            Notice { kind: NoticeKind::Variation }
            if has_estimates {
                Notice { kind: NoticeKind::Estimates, class: "notice-warn" }
            }
            for (group, rows) in groups {
                Card { key: "{group.key()}", title: tr(&format!("group-{}", group.key())), id: format!("group-{}", group.key()),
                    details { class: "group", open: matches!(group, NutrientGroup::Energy | NutrientGroup::Macronutrients),
                        summary {
                            span { class: "small muted",
                                {tr_args("nutrients-group-summary", &[
                                    ("count", rows.len().to_string()),
                                    ("unknown", rows.iter().filter(|r| r.value.is_none()).count().to_string()),
                                ])}
                            }
                            crate::components::Icon { name: crate::components::IconName::ChevronDown, size: 20 }
                        }
                        table { class: "nutrients",
                            thead {
                                tr {
                                    th { {tr("nutrients-col-name")} }
                                    th { class: "r", {tr("nutrients-col-portion")} }
                                    th { class: "r", {tr("nutrients-col-origin")} }
                                }
                            }
                            tbody {
                                for row in rows {
                                    tr { key: "{row.name}",
                                        td { "{row.name}" }
                                        td { class: "r num",
                                            if let Some(v) = &row.value { "{v}" } else { span { class: "muted", {tr("value-unknown")} } }
                                        }
                                        td { class: "r", Mark { derivation: row.derivation.clone() } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            DataStatusNotice { source: source.clone(), date: date::display(&fetched) }
            Link { class: "btn btn-secondary btn-large", to: Route::Estimates {}, {tr("estimates-all")} }
        }
    }
}

/// Origin mark: ● measured, ∑ calculated or declared, ≈ estimate, ? unknown.
#[component]
fn Mark(derivation: Option<Derivation>) -> Element {
    let (class, symbol, label) = match &derivation {
        Some(Derivation::Measured) => ("mark-measured", "●", tr("origin-measured")),
        Some(Derivation::Calculated) => ("mark-calculated", "∑", tr("origin-calculated")),
        Some(Derivation::Declared) => ("mark-calculated", "∑", tr("origin-declared")),
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
