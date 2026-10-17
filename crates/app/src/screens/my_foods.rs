//! 15 Meine Lebensmittel bearbeiten: portion, order, remove, saved plates.

use dioxus::prelude::*;
use healthyteller_core::{Food, Level, Options, evaluate};

use crate::Route;
use crate::components::{Badge, Card, Icon, IconName, TopBar};
use crate::fetch;
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{MyFood, default_portion, update, use_app_state};
use healthyteller_ingest::search::SearchHit;

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
            AddMyFood {}
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

#[derive(Clone, PartialEq)]
enum Lookup {
    Idle,
    Loading,
    Failed(String),
}

/// Adds foods to the list directly, with a usual portion of 100 g that can be changed below.
#[component]
fn AddMyFood() -> Element {
    let state = use_app_state();
    let mut query = use_signal(String::new);
    let mut barcode = use_signal(String::new);
    let mut results = use_signal(Vec::<SearchHit>::new);
    let mut lookup = use_signal(|| Lookup::Idle);
    let mut last = use_signal(|| None::<String>);

    let mut add = move |food: Food| {
        let name = food.name.clone();
        update(state, |s| {
            if s.my_food(&food.id.0).is_none() {
                s.my_foods.push(MyFood {
                    food_id: food.id.clone(),
                    portion: default_portion(),
                });
            }
            s.cache_food(food);
        });
        last.set(Some(name));
    };

    // Search hits carry only name and brand; the full product is loaded by barcode.
    let mut add_hit = move |id: String| {
        let code = id.trim_start_matches("off:").to_owned();
        lookup.set(Lookup::Loading);
        spawn(async move {
            match fetch::product_by_barcode(&code).await {
                Ok(food) => {
                    add(food);
                    lookup.set(Lookup::Idle);
                }
                Err(e) => lookup.set(Lookup::Failed(tr(e.message_key()))),
            }
        });
    };

    let search = move |_| {
        let terms = query();
        lookup.set(Lookup::Loading);
        spawn(async move {
            match fetch::search_products(&terms).await {
                Ok(foods) => {
                    lookup.set(if foods.is_empty() {
                        Lookup::Failed(tr("search-online-none"))
                    } else {
                        Lookup::Idle
                    });
                    results.set(foods);
                }
                Err(e) => lookup.set(Lookup::Failed(tr(e.message_key()))),
            }
        });
    };

    let by_barcode = move |_| {
        let code = barcode();
        lookup.set(Lookup::Loading);
        spawn(async move {
            match fetch::product_by_barcode(&code).await {
                Ok(food) => {
                    add(food);
                    barcode.set(String::new());
                    lookup.set(Lookup::Idle);
                }
                Err(e) => lookup.set(Lookup::Failed(tr(e.message_key()))),
            }
        });
    };

    // Foods already on this device come first, without a network request.
    let q = query().trim().to_lowercase();
    let s = state.read();
    // (food, already in my foods, search hit that still has to be loaded)
    let mut shown: Vec<(Food, bool, bool)> = if q.chars().count() < 2 {
        Vec::new()
    } else {
        s.foods
            .values()
            .map(|c| c.food.clone())
            .filter(|f| {
                f.name.to_lowercase().contains(&q)
                    || f.brand
                        .as_ref()
                        .is_some_and(|b| b.to_lowercase().contains(&q))
            })
            .map(|f| {
                let mine = s.my_food(&f.id.0).is_some();
                (f, mine, false)
            })
            .collect()
    };
    for hit in results() {
        let f = hit.food;
        if !shown.iter().any(|(x, _, _)| x.id == f.id) {
            let mine = s.my_food(&f.id.0).is_some();
            shown.push((f, mine, true));
        }
    }
    drop(s);
    let loading = lookup() == Lookup::Loading;

    rsx! {
        Card { title: tr("my-foods-add-title"), id: "my-foods-add",
            div { class: "card-body",
                div { class: "field",
                    label { r#for: "my-foods-query", {tr("search-label")} }
                    div { style: "display: flex; gap: 10px;",
                        input {
                            id: "my-foods-query",
                            class: "input",
                            r#type: "search",
                            placeholder: tr("search-placeholder"),
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                        }
                        button { class: "btn btn-primary", disabled: query().trim().chars().count() < 2 || loading, onclick: search, {tr("my-foods-search")} }
                    }
                    p { class: "small muted", {tr("search-online-privacy")} }
                }
                div { class: "field",
                    label { r#for: "my-foods-barcode", {tr("barcode-label")} }
                    div { style: "display: flex; gap: 10px;",
                        input {
                            id: "my-foods-barcode",
                            class: "input num",
                            inputmode: "numeric",
                            autocomplete: "off",
                            value: "{barcode}",
                            oninput: move |e| barcode.set(e.value()),
                        }
                        button { class: "btn btn-primary", disabled: barcode().trim().is_empty() || loading, onclick: by_barcode, {tr("action-add")} }
                    }
                }
                if loading {
                    p { role: "status", {tr("barcode-loading")} }
                }
                if let Lookup::Failed(msg) = lookup() {
                    p { class: "error", role: "alert", "{msg}" }
                }
                if let Some(name) = last() {
                    p { role: "status", {tr_args("my-foods-added", &[("name", name)])} }
                }
            }
            if !shown.is_empty() {
                ul { class: "list",
                    for (food, mine, online) in shown {
                        li { key: "{food.id.0}", class: "food-row",
                            span { class: "grow food-name", {labels::food_title(&food)} }
                            if mine {
                                span { class: "small muted", {tr("my-foods-already")} }
                            } else {
                                button {
                                    class: "btn btn-secondary",
                                    "aria-label": tr_args("my-foods-add-named", &[("name", food.name.clone())]),
                                    onclick: {
                                        let food = food.clone();
                                        move |_| if online { add_hit(food.id.0.clone()) } else { add(food.clone()) }
                                    },
                                    Icon { name: IconName::Plus, size: 20 }
                                    {tr("action-add")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
