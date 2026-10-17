//! 03/04 Suche: one search field, the camera scan and the number as fallback.

use dioxus::prelude::*;
use healthyteller_ingest::search::SearchHit;

use crate::Route;
use crate::components::{Icon, IconName, QualityMark, TopBar};
use crate::fetch;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{update, use_app_state};

#[derive(Clone, PartialEq)]
enum Online {
    Idle,
    Loading,
    Results(Vec<SearchHit>),
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
    let mut online = use_signal(|| Online::Idle);
    let scanning = use_signal(|| false);

    let s = state.read();
    let q = query().trim().to_lowercase();
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
    drop(s);

    let mut search_online = move || {
        let terms = query();
        if terms.trim().chars().count() < 2 || online() == Online::Loading {
            return;
        }
        online.set(Online::Loading);
        spawn(async move {
            match fetch::search_products(&terms).await {
                Ok(foods) => online.set(Online::Results(foods)),
                Err(e) => online.set(Online::Failed(tr(e.message_key()))),
            }
        });
    };

    // Search hits carry only name and brand; the full product is loaded by barcode.
    let pick_result = move |id: String| {
        let code = id.trim_start_matches("off:").to_owned();
        spawn(async move {
            match fetch::product_by_barcode(&code).await {
                Ok(food) => {
                    let id = food.id.0.clone();
                    update(state, |s| s.cache_food(food));
                    navigator().push(Route::PortionScreen { id });
                }
                Err(e) => online.set(Online::Failed(tr(e.message_key()))),
            }
        });
    };

    let mut check_code = move |code: String| {
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

    let local_ids: Vec<String> = hits.iter().map(|(id, _, _)| id.clone()).collect();
    let online_hits: Vec<SearchHit> = match online() {
        Online::Results(hits) => hits
            .into_iter()
            .filter(|h| !local_ids.contains(&h.food.id.0))
            .collect(),
        _ => Vec::new(),
    };
    let searched = matches!(online(), Online::Results(_));
    let nothing = searched && hits.is_empty() && online_hits.is_empty();

    let number_entry = rsx! {
        div { class: "field",
            label { r#for: "barcode", {tr("barcode-label")} }
            div { class: "input-row",
                input {
                    id: "barcode",
                    class: "input num",
                    inputmode: "numeric",
                    autocomplete: "off",
                    value: "{barcode}",
                    oninput: move |e| barcode.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !barcode().trim().is_empty() {
                            check_code(barcode());
                        }
                    },
                }
                button {
                    class: "btn btn-secondary",
                    disabled: barcode().trim().is_empty() || lookup() == Lookup::Loading,
                    onclick: move |_| check_code(barcode()),
                    Icon { name: IconName::Barcode }
                    if lookup() == Lookup::Loading { {tr("barcode-loading")} } else { {tr("barcode-check")} }
                }
            }
        }
        if let Lookup::Failed(msg) = lookup() {
            p { class: "error", role: "alert", "{msg}" }
        }
    };

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("app-title") }
            h1 { class: "title", {tr("search-title")} }
            div { class: "field",
                label { r#for: "search", {tr("search-label")} }
                div { class: "input-row",
                    div { class: "search grow",
                        Icon { name: IconName::Search }
                        input {
                            id: "search",
                            class: "input",
                            r#type: "search",
                            placeholder: tr("search-placeholder"),
                            value: "{query}",
                            oninput: move |e| {
                                query.set(e.value());
                                online.set(Online::Idle);
                            },
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    search_online();
                                }
                            },
                        }
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: q.chars().count() < 2 || online() == Online::Loading,
                        onclick: move |_| search_online(),
                        {tr("search-action")}
                    }
                }
            }
            if !q.is_empty() {
                section { class: "card", id: "search-results", "aria-label": tr("search-results"),
                    ul { class: "list",
                        for (id, name, mine) in hits {
                            li { key: "{id}",
                                Link { class: "food-row", to: Route::PortionScreen { id: id.clone() },
                                    span { class: "grow food-name", "{name}" }
                                    if mine { span { class: "small muted", {tr("search-mine")} } }
                                    QualityMark { id: id.clone(), compact: true }
                                    Icon { name: IconName::ChevronRight }
                                }
                            }
                        }
                        for hit in online_hits {
                            li { key: "{hit.food.id.0}",
                                button {
                                    class: "row-btn food-row",
                                    onclick: {
                                        let id = hit.food.id.0.clone();
                                        move |_| pick_result(id.clone())
                                    },
                                    span { class: "grow food-name", {labels::food_title(&hit.food)} }
                                    if hit.merged > 1 {
                                        span { class: "small muted", {tr_args("search-merged", &[("count", hit.merged.to_string())])} }
                                    }
                                    Icon { name: IconName::ChevronRight }
                                }
                            }
                        }
                    }
                    if let Online::Failed(msg) = online() {
                        p { class: "card-row error", role: "alert", "{msg}" }
                    }
                    div { role: "status",
                        match online() {
                            Online::Loading => rsx! { p { class: "card-row muted", {tr("barcode-loading")} } },
                            Online::Failed(_) => rsx! {},
                            Online::Idle => rsx! { p { class: "card-row small muted", {tr("search-press-enter")} } },
                            Online::Results(_) if nothing => rsx! { p { class: "card-row muted", {tr("search-nothing")} } },
                            Online::Results(_) => rsx! {},
                        }
                    }
                }
            }
            {camera_section(scanning, barcode, check_code, number_entry)}
        }
    }
}

/// Camera scan where the platform offers it (web app); the number entry stays one tap away.
#[cfg(target_arch = "wasm32")]
fn camera_section(
    mut scanning: Signal<bool>,
    mut barcode: Signal<String>,
    mut check_code: impl FnMut(String) + Copy + 'static,
    number_entry: Element,
) -> Element {
    rsx! {
        if scanning() {
            crate::scanner::CameraScanner {
                on_code: move |code: String| {
                    scanning.set(false);
                    barcode.set(code.clone());
                    check_code(code);
                },
                on_close: move |_| scanning.set(false),
            }
        } else {
            button {
                class: "btn btn-primary btn-large",
                onclick: move |_| scanning.set(true),
                Icon { name: IconName::Barcode }
                {tr("scan-start")}
            }
        }
        details { class: "card group",
            summary { span { {tr("barcode-type")} } Icon { name: IconName::ChevronDown, size: 20 } }
            div { class: "card-body", {number_entry} }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn camera_section(
    _scanning: Signal<bool>,
    _barcode: Signal<String>,
    _check_code: impl FnMut(String) + Copy + 'static,
    number_entry: Element,
) -> Element {
    rsx! {
        section { class: "card", "aria-label": tr("barcode-title"),
            div { class: "card-body", {number_entry} }
        }
    }
}
