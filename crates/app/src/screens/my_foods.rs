//! 15 Meine Lebensmittel bearbeiten: portion, order, remove, saved plates.

use dioxus::prelude::*;
use healthyteller_core::{Level, Options, evaluate};

use crate::Route;
use crate::components::{Badge, Card, Icon, IconName, TopBar};
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::state::{update, use_app_state};

#[derive(Clone, PartialEq)]
struct Row {
    index: usize,
    id: String,
    name: String,
    grams: f64,
    worst: Option<(Level, String)>,
}

#[component]
pub fn MyFoods() -> Element {
    let state = use_app_state();
    let s = state.read();
    let profile = s.active_profile().cloned();
    let rows: Vec<Row> = s
        .my_foods
        .iter()
        .enumerate()
        .map(|(index, m)| {
            let food = s.food(&m.food_id.0);
            // Shows the most important result, so problems are visible in the list.
            let worst = match (&profile, food) {
                (Some(p), Some(f)) => {
                    let e = evaluate(
                        f,
                        &m.portion,
                        p,
                        &s.rules,
                        &Default::default(),
                        Options {
                            include_drafts: s.settings.include_draft_rules,
                        },
                    );
                    e.attention()
                        .next()
                        .map(|r| (r.level, crate::labels::result_title(r)))
                }
                _ => None,
            };
            Row {
                index,
                id: m.food_id.0.clone(),
                name: food.map_or_else(|| m.food_id.0.clone(), |f| f.name.clone()),
                grams: m.portion.grams,
                worst,
            }
        })
        .collect();
    let plates: Vec<(usize, String, usize)> = s
        .plates
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.name.clone(), p.items.len()))
        .collect();
    let count = rows.len();
    drop(s);

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Search {}, title: tr("action-edit") }
            div {
                h1 { class: "title", {tr("my-foods-title")} }
                p { class: "lead", {tr("my-foods-edit-lead")} }
            }
            Link { class: "btn btn-primary btn-large", to: Route::Search {}, Icon { name: IconName::Plus } {tr("action-add")} }
            Card { title: tr_args("my-foods-count", &[("count", count.to_string())]), id: "my-foods-list",
                if rows.is_empty() {
                    p { class: "card-body muted", {tr("my-foods-empty")} }
                }
                ul { class: "list",
                    for row in rows {
                        li { key: "{row.id}", class: "food-row", style: "align-items: flex-start;",
                            div { style: "display: flex; flex-direction: column; gap: 2px;",
                                button {
                                    class: "icon-btn",
                                    disabled: row.index == 0,
                                    onclick: move |_| update(state, |s| { if row.index > 0 { s.my_foods.swap(row.index, row.index - 1); } }),
                                    Icon { name: IconName::Up, size: 20 }
                                    span { class: "visually-hidden", {tr_args("action-move-up", &[("name", row.name.clone())])} }
                                }
                                button {
                                    class: "icon-btn",
                                    disabled: row.index + 1 == count,
                                    onclick: move |_| update(state, |s| { if row.index + 1 < s.my_foods.len() { s.my_foods.swap(row.index, row.index + 1); } }),
                                    Icon { name: IconName::Down, size: 20 }
                                    span { class: "visually-hidden", {tr_args("action-move-down", &[("name", row.name.clone())])} }
                                }
                            }
                            div { class: "grow", style: "display: flex; flex-direction: column; gap: 8px;",
                                Link { class: "food-name", to: Route::FoodCard { id: row.id.clone(), grams: 0, prep: "mine".into() }, "{row.name}" }
                                label { class: "input-unit",
                                    span { class: "muted", {tr("my-foods-portion")} }
                                    input {
                                        class: "input num",
                                        style: "max-width: 140px; min-height: 48px;",
                                        inputmode: "decimal",
                                        value: format::number(row.grams),
                                        "aria-label": tr_args("my-foods-portion-of", &[("name", row.name.clone())]),
                                        onchange: move |e| {
                                            if let Some(g) = format::parse_number(&e.value()).filter(|g| *g > 0.0) {
                                                update(state, |s| { if let Some(m) = s.my_foods.get_mut(row.index) { m.portion.grams = g; } });
                                            }
                                        },
                                    }
                                    span { "g" }
                                }
                                if let Some((level, what)) = row.worst.clone() {
                                    span { style: "display: flex; gap: 8px; align-items: center;", Badge { level, small: true } span { class: "small", "{what}" } }
                                }
                            }
                            button {
                                class: "icon-btn",
                                onclick: move |_| update(state, |s| { if row.index < s.my_foods.len() { s.my_foods.remove(row.index); } }),
                                Icon { name: IconName::Trash, size: 20 }
                                span { class: "visually-hidden", {tr_args("action-remove-named", &[("name", row.name.clone())])} }
                            }
                        }
                    }
                }
            }
            Card { title: tr("plates-saved"), id: "plates-saved",
                if plates.is_empty() {
                    p { class: "card-body muted", {tr("plates-saved-empty")} }
                }
                ul { class: "list",
                    for (i, name, n) in plates {
                        li { key: "{i}", class: "food-row",
                            div { class: "grow",
                                p { class: "food-name", "{name}" }
                                p { class: "small muted", {tr_args("plates-food-count", &[("count", n.to_string())])} }
                            }
                            button {
                                class: "icon-btn",
                                onclick: move |_| update(state, |s| { if i < s.plates.len() { s.plates.remove(i); } }),
                                Icon { name: IconName::Trash, size: 20 }
                                span { class: "visually-hidden", {tr_args("action-remove-named", &[("name", name.clone())])} }
                            }
                        }
                    }
                }
            }
            Link { class: "btn btn-secondary btn-large", to: Route::PrintList {}, Icon { name: IconName::Print } {tr("print-create")} }
        }
    }
}
