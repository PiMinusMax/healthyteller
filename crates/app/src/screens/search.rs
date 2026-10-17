//! 03/04 Suche: search field, barcode, saved plates and the person's own foods.

use dioxus::prelude::*;
use healthyteller_core::Food;

use crate::Route;
use crate::components::{Card, Icon, IconName, TopBar};
use crate::fetch;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{update, use_app_state};

#[derive(Clone, PartialEq)]
enum Online {
    Idle,
    Loading,
    Results(Vec<Food>),
    Failed(String),
}

#[derive(Clone, PartialEq)]
enum Lookup {
    Idle,
    Loading,
    Failed(String),
}

#[component]
pub fn Search() -> Element {
    let state = use_app_state();
    let mut query = use_signal(String::new);
    let mut barcode = use_signal(String::new);
    let mut lookup = use_signal(|| Lookup::Idle);
    let mut last_added = use_signal(|| None::<(String, usize)>);
    let mut online = use_signal(|| Online::Idle);
    let mut import_error = use_signal(|| None::<String>);

    let s = state.read();
    let q = query().trim().to_lowercase();
    let my_foods: Vec<(String, String, String)> = s
        .my_foods
        .iter()
        .filter_map(|m| {
            let food = s.food(&m.food_id.0)?;
            Some((
                m.food_id.0.clone(),
                labels::food_title(food),
                labels::portion(&m.portion),
            ))
        })
        .collect();
    let hits: Vec<(String, String, bool)> = if q.is_empty() {
        Vec::new()
    } else {
        let mut hits: Vec<(String, String, bool)> = s
            .foods
            .values()
            .map(|c| &c.food)
            .filter(|f| {
                f.name.to_lowercase().contains(&q)
                    || f.brand
                        .as_ref()
                        .is_some_and(|b| b.to_lowercase().contains(&q))
                    || f.id.0.ends_with(&q)
            })
            .map(|f| {
                (
                    f.id.0.clone(),
                    labels::food_title(f),
                    s.my_food(&f.id.0).is_some(),
                )
            })
            .collect();
        // Own foods first (concept: "Meine Lebensmittel stehen in der Suche zuerst").
        hits.sort_by_key(|(_, name, mine)| (!*mine, name.clone()));
        hits
    };
    let plates: Vec<(usize, String, String)> = s
        .plates
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let names: Vec<String> = p
                .items
                .iter()
                .filter_map(|it| s.food(&it.food_id.0).map(|f| f.name.clone()))
                .collect();
            (i, p.name.clone(), names.join(", "))
        })
        .collect();
    drop(s);

    let search_online = move |_| {
        let terms = query();
        online.set(Online::Loading);
        spawn(async move {
            match fetch::search_products(&terms).await {
                Ok(foods) => online.set(Online::Results(foods)),
                Err(e) => online.set(Online::Failed(tr(e.message_key()))),
            }
        });
    };

    let import_file = move |e: FormEvent| async move {
        let Some(file) = e.files().into_iter().next() else {
            return;
        };
        match file.read_bytes().await {
            Ok(bytes) => match fetch::parse_product(&bytes) {
                Ok(food) => {
                    let id = food.id.0.clone();
                    update(state, |s| s.cache_food(food));
                    import_error.set(None);
                    navigator().push(Route::PortionScreen { id });
                }
                Err(err) => import_error.set(Some(tr(err.message_key()))),
            },
            Err(_) => import_error.set(Some(tr("import-read-error"))),
        }
    };

    let check_barcode = move |_| {
        let code = barcode();
        lookup.set(Lookup::Loading);
        spawn(async move {
            match fetch::product_by_barcode(&code).await {
                Ok(food) => {
                    let id = food.id.0.clone();
                    update(state, |s| s.cache_food(food));
                    lookup.set(Lookup::Idle);
                    navigator().push(Route::PortionScreen { id });
                }
                Err(e) => lookup.set(Lookup::Failed(tr(e.message_key()))),
            }
        });
    };

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("app-title") }
            h1 { class: "title", {tr("search-title")} }
            div { class: "field",
                label { r#for: "search", {tr("search-label")} }
                div { class: "search",
                    Icon { name: IconName::Search }
                    input {
                        id: "search",
                        class: "input",
                        r#type: "search",
                        placeholder: tr("search-placeholder"),
                        value: "{query}",
                        oninput: move |e| query.set(e.value()),
                    }
                }
            }
            if !q.is_empty() {
                Card { title: tr("search-results"),
                    if hits.is_empty() {
                        p { class: "card-body muted", {tr("search-no-hits")} }
                    } else {
                        ul { class: "list",
                            for (id, name, mine) in hits {
                                li { key: "{id}",
                                    Link { class: "food-row", to: Route::PortionScreen { id: id.clone() },
                                        span { class: "grow food-name", "{name}" }
                                        if mine { span { class: "small muted", {tr("search-mine")} } }
                                    }
                                }
                            }
                        }
                    }
                    p { class: "card-body small muted", {tr("search-local-only")} }
                }
                Card { title: tr("search-online-title"), id: "search-online",
                    div { class: "card-body",
                        button {
                            class: "btn btn-secondary",
                            disabled: q.chars().count() < 2 || online() == Online::Loading,
                            onclick: search_online,
                            Icon { name: IconName::Search }
                            if online() == Online::Loading { {tr("barcode-loading")} } else { {tr("search-online-action")} }
                        }
                        p { class: "small muted", {tr("search-online-privacy")} }
                        if let Online::Failed(msg) = online() {
                            p { class: "error", role: "alert", "{msg}" }
                        }
                    }
                    if let Online::Results(foods) = online() {
                        if foods.is_empty() {
                            p { class: "card-row muted", {tr("search-online-none")} }
                        }
                        ul { class: "list",
                            for food in foods {
                                li { key: "{food.id.0}",
                                    button {
                                        class: "row-btn food-row",
                                        onclick: {
                                            let food = food.clone();
                                            move |_| {
                                                let id = food.id.0.clone();
                                                let food = food.clone();
                                                update(state, |s| s.cache_food(food));
                                                navigator().push(Route::PortionScreen { id });
                                            }
                                        },
                                        span { class: "grow food-name", {labels::food_title(&food)} }
                                        Icon { name: IconName::ChevronRight }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Card { title: tr("barcode-title"),
                div { class: "card-body",
                    div { class: "field",
                        label { r#for: "barcode", {tr("barcode-label")} }
                        input {
                            id: "barcode",
                            class: "input num",
                            inputmode: "numeric",
                            autocomplete: "off",
                            value: "{barcode}",
                            oninput: move |e| barcode.set(e.value()),
                        }
                    }
                    button {
                        class: "btn btn-secondary btn-large",
                        disabled: barcode().trim().is_empty() || lookup() == Lookup::Loading,
                        onclick: check_barcode,
                        Icon { name: IconName::Barcode }
                        if lookup() == Lookup::Loading { {tr("barcode-loading")} } else { {tr("barcode-check")} }
                    }
                    if let Lookup::Failed(msg) = lookup() {
                        p { class: "error", role: "alert", "{msg}" }
                    }
                    p { class: "small muted", {tr("barcode-privacy")} }
                    p { class: "small muted", {tr("barcode-camera-later")} }
                    div { class: "field",
                        label { r#for: "import-file", {tr("import-label")} }
                        input {
                            id: "import-file",
                            r#type: "file",
                            accept: ".json,application/json",
                            onchange: import_file,
                        }
                        p { class: "small muted", {tr("import-hint")} }
                        if let Some(msg) = import_error() {
                            p { class: "error", role: "alert", "{msg}" }
                        }
                    }
                }
            }
            if !plates.is_empty() {
                Card { title: tr("plates-saved"),
                    ul { class: "list",
                        for (i, name, foods) in plates {
                            li { key: "{i}", class: "food-row",
                                div { class: "grow",
                                    p { class: "food-name", "{name}" }
                                    p { class: "small muted", "{foods}" }
                                }
                                button {
                                    class: "btn btn-primary",
                                    onclick: move |_| {
                                        update(state, |s| {
                                            if let Some(p) = s.plates.get(i).cloned() {
                                                for item in p.items {
                                                    s.add_to_plate(item.food_id, item.portion);
                                                }
                                            }
                                        });
                                        navigator().push(Route::Plate {});
                                    },
                                    {tr("action-take-over")}
                                }
                            }
                        }
                    }
                }
            }
            Card { title: tr("my-foods-title"),
                div { class: "card-body", style: "flex-direction: row; align-items: center; justify-content: space-between;",
                    p { class: "small muted", {tr("my-foods-hint")} }
                    Link { class: "link-btn", to: Route::MyFoods {}, {tr("action-edit")} }
                }
                if my_foods.is_empty() {
                    p { class: "card-row muted", {tr("my-foods-empty")} }
                } else {
                    ul { class: "list",
                        for (idx, (id, name, portion)) in my_foods.into_iter().enumerate() {
                            li { key: "{id}", class: "food-row",
                                button {
                                    class: "row-btn grow",
                                    onclick: move |_| {
                                        update(state, |s| {
                                            if let Some(m) = s.my_foods.get(idx).cloned() {
                                                s.add_to_plate(m.food_id, m.portion);
                                            }
                                        });
                                        let n = state.read().day.entries.len();
                                        last_added.set(Some((name.clone(), n)));
                                    },
                                    span { class: "plus", "aria-hidden": "true", "+" }
                                    span { span { class: "food-name", "{name}" } ", {portion}" }
                                }
                                Link { class: "icon-btn", to: Route::FoodCard { id: id.clone(), grams: 0, prep: "mine".into() },
                                    Icon { name: IconName::Info }
                                    span { class: "visually-hidden", {tr_args("my-foods-details", &[("name", name.clone())])} }
                                }
                            }
                        }
                    }
                }
            }
            if let Some((name, count)) = last_added() {
                div { class: "notice", role: "status", style: "display: flex; align-items: center; gap: 12px;",
                    span { style: "flex: 1;", {tr_args("plate-added", &[("name", name)])} }
                    button {
                        class: "btn btn-quiet",
                        onclick: move |_| {
                            update(state, |s| {
                                if s.day.entries.len() == count {
                                    s.day.entries.pop();
                                }
                            });
                            last_added.set(None);
                        },
                        {tr("action-undo")}
                    }
                }
            }
        }
    }
}
