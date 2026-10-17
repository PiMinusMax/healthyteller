//! 23 Alternativen: two foods side by side, values from the source and verdicts per dimension.

use dioxus::prelude::*;
use healthyteller_core::{Dimension, Food, Level, Nutrient, Portion, Preparation, evaluate};

use crate::Route;
use crate::components::{Badge, Card, Icon, IconName, TopBar};
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{update, use_app_state};

/// Values shown in the comparison; the full lists are one tap away.
const COMPARED: [Nutrient; 7] = [
    Nutrient::Energy,
    Nutrient::Sugars,
    Nutrient::Protein,
    Nutrient::Fat,
    Nutrient::SaturatedFat,
    Nutrient::Fibre,
    Nutrient::Salt,
];

fn portion(grams: u32) -> Portion {
    Portion {
        grams: f64::from(grams.max(1)),
        preparation: Preparation::Raw,
    }
}

/// Picks the second food: own foods first, then everything stored on the device.
#[component]
pub fn CompareStart(id: String, grams: u32) -> Element {
    let state = use_app_state();
    let s = state.read();
    let name = s.food(&id).map(|f| f.name.clone()).unwrap_or_default();
    let mut options: Vec<(String, String, bool)> = s
        .foods
        .values()
        .map(|c| &c.food)
        .filter(|f| f.id.0 != id)
        .map(|f| {
            (
                f.id.0.clone(),
                labels::food_title(f),
                s.my_food(&f.id.0).is_some(),
            )
        })
        .collect();
    drop(s);
    options.sort_by_key(|(_, n, mine)| (!*mine, n.clone()));
    rsx! {
        main { class: "screen",
            TopBar { back: Route::FoodCard { id: id.clone(), grams, prep: "raw".into() }, title: tr("compare-topbar") }
            div {
                h1 { class: "title", {tr_args("compare-instead-of", &[("name", name)])} }
                p { class: "lead", {tr("compare-pick-lead")} }
            }
            Card { title: tr("compare-pick"), id: "compare-pick",
                if options.is_empty() {
                    p { class: "card-body muted", {tr("compare-none")} }
                }
                ul { class: "list",
                    for (other, title, mine) in options {
                        li { key: "{other}",
                            Link { class: "food-row", to: Route::Compare { id: id.clone(), other: other.clone(), grams },
                                span { class: "grow food-name", "{title}" }
                                if mine { span { class: "small muted", {tr("search-mine")} } }
                                Icon { name: IconName::ChevronRight }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Column {
    food: Food,
    verdicts: Vec<(Dimension, Level)>,
}

#[component]
pub fn Compare(id: String, other: String, grams: u32) -> Element {
    let state = use_app_state();
    let mut added = use_signal(|| None::<String>);
    let s = state.read();
    let (Some(a), Some(b)) = (s.food(&id).cloned(), s.food(&other).cloned()) else {
        drop(s);
        return rsx! {
            main { class: "screen",
                TopBar { back: Route::Search {}, title: tr("compare-topbar") }
                h1 { class: "title", {tr("food-missing-title")} }
            }
        };
    };
    let profile = s.active_profile().cloned();
    let p = portion(grams);
    let column = |food: Food| {
        let verdicts = profile
            .as_ref()
            .map(|pr| {
                let e = evaluate(
                    &food,
                    &p,
                    pr,
                    &s.rules,
                    &s.day_totals(false),
                    s.eval_options(),
                );
                let mut dims: Vec<Dimension> = e.results.iter().map(|r| r.dimension).collect();
                dims.dedup();
                dims.into_iter()
                    .filter_map(|d| e.level_of(d).map(|l| (d, l)))
                    .collect()
            })
            .unwrap_or_default();
        Column { food, verdicts }
    };
    let cols = [column(a), column(b)];
    drop(s);
    let dims: Vec<Dimension> = {
        let mut d: Vec<Dimension> = cols
            .iter()
            .flat_map(|c| c.verdicts.iter().map(|v| v.0))
            .collect();
        d.sort();
        d.dedup();
        d
    };
    let checked_for = profile
        .as_ref()
        .map(|pr| tr_args("card-checked-for", &[("name", pr.name.clone())]))
        .unwrap_or_default();

    rsx! {
        main { class: "screen",
            TopBar { back: Route::CompareStart { id: id.clone(), grams }, title: tr("compare-topbar") }
            div {
                h1 { class: "title", {tr_args("compare-instead-of", &[("name", cols[0].food.name.clone())])} }
                p { class: "lead", {tr_args("compare-per-portion", &[("grams", grams.to_string())])} " " "{checked_for}" }
            }
            div { class: "card", style: "margin-top: 0; overflow-x: auto;",
                table { class: "nutrients compare",
                    thead {
                        tr {
                            th { span { class: "visually-hidden", {tr("nutrients-col-name")} } }
                            for c in cols.iter() { th { class: "r", scope: "col", style: "font-weight: 800; color: var(--kt-ink);", "{c.food.name}" } }
                        }
                    }
                    tbody {
                        for n in COMPARED {
                            tr { key: "{n.key()}",
                                th { scope: "row", style: "font-weight: 400; color: var(--kt-ink-2);", {labels::nutrient(n)} }
                                for c in cols.iter() {
                                    td { class: "r num",
                                        match c.food.amount_in(n, &p) {
                                            Some(q) => rsx! { {format::quantity(q.to(n.unit()).unwrap_or(q))} },
                                            None => rsx! { span { class: "muted", {tr("value-unknown")} } },
                                        }
                                    }
                                }
                            }
                        }
                        for d in dims {
                            tr { key: "{d.key()}",
                                th { scope: "row", style: "font-weight: 400; color: var(--kt-ink-2);", {labels::dimension(d)} }
                                for c in cols.iter() {
                                    td { class: "r",
                                        match c.verdicts.iter().find(|v| v.0 == d) {
                                            Some((_, level)) => rsx! { Badge { level: *level, small: true } },
                                            None => rsx! { span { class: "muted", "–" } },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            p { class: "small muted", {tr("compare-note")} }
            for c in cols.iter() {
                button {
                    key: "{c.food.id.0}",
                    class: "btn btn-secondary btn-large",
                    onclick: {
                        let food_id = c.food.id.clone();
                        let name = c.food.name.clone();
                        move |_| {
                            update(state, |s| s.add_to_plate(food_id.clone(), p));
                            added.set(Some(name.clone()));
                        }
                    },
                    {tr_args("compare-add", &[("name", c.food.name.clone())])}
                }
            }
            if let Some(name) = added() {
                p { role: "status", {tr_args("plate-added", &[("name", name)])} }
            }
            Link { class: "btn btn-quiet", to: Route::FoodCard { id: other.clone(), grams, prep: "raw".into() }, {tr_args("compare-open", &[("name", cols[1].food.name.clone())])} }
        }
    }
}
