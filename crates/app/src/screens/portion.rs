//! 06 Portion und Zubereitung.

use dioxus::prelude::*;
use healthyteller_core::Preparation;

use crate::Route;
use crate::components::{Card, TopBar};
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{default_portion, use_app_state};

const QUICK: [u32; 4] = [50, 100, 150, 200];

#[component]
pub fn PortionScreen(id: String) -> Element {
    let state = use_app_state();
    let s = state.read();
    let food = s.food(&id).cloned();
    let start = food
        .as_ref()
        .map_or_else(default_portion, |f| s.usual_portion(f));
    drop(s);
    let mut grams = use_signal(|| start.grams.round() as u32);
    let mut prep = use_signal(|| start.preparation);

    let Some(food) = food else {
        return rsx! {
            main { class: "screen",
                TopBar { back: Route::Search {}, title: tr("app-title") }
                h1 { class: "title", {tr("food-missing-title")} }
                p { {tr("food-missing-text")} }
                Link { class: "btn btn-primary btn-large", to: Route::Search {}, {tr("search-title")} }
            }
        };
    };
    let source = labels::source_name(&food.source.id);
    // The slider reaches at least the whole package.
    let max_grams = food
        .portions
        .iter()
        .map(|p| p.grams.round() as u32)
        .fold(1000, u32::max);
    let kind = tr(match food.kind {
        healthyteller_core::FoodKind::Generic => "food-kind-generic",
        healthyteller_core::FoodKind::Brand => "food-kind-brand",
        healthyteller_core::FoodKind::Dish => "food-kind-dish",
    });

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Search {}, title: tr("portion-topbar") }
            div {
                h1 { class: "title", "{food.name}" }
                p { class: "muted", style: "font-size: 20px;",
                    "{kind} · {source}"
                    if let Some(b) = &food.brand { " · {b}" }
                }
            }
            Card { title: tr("portion-how-much"),
                div { class: "card-body",
                    div { style: "display: flex; justify-content: space-between; align-items: baseline;",
                        label { r#for: "grams", class: "muted", style: "font-size: 20px;", {tr("portion-grams")} }
                        span { class: "big-value num", "aria-live": "polite", {format!("{} g", grams())} }
                    }
                    input {
                        id: "grams",
                        class: "range",
                        r#type: "range",
                        min: "5",
                        max: "{max_grams}",
                        step: "5",
                        value: "{grams}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<u32>() {
                                grams.set(v);
                            }
                        },
                    }
                    if !food.portions.is_empty() {
                        div { class: "quick source-portions", role: "group", "aria-label": tr("portion-from-source"),
                            for p in food.portions.clone() {
                                {
                                    let g = p.grams.round() as u32;
                                    let name = tr(&format!("portion-kind-{}", p.kind.key()));
                                    rsx! {
                                        button {
                                            key: "{p.kind.key()}",
                                            class: "btn",
                                            "aria-pressed": if grams() == g { "true" } else { "false" },
                                            onclick: move |_| grams.set(g),
                                            span { style: "display: flex; flex-direction: column; align-items: center;",
                                                span { "{name}" }
                                                span { class: "num small", "{g} g" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        p { class: "small muted", {tr_args("portion-source-note", &[("source", source.clone())])} }
                    }
                    div { class: "quick", role: "group", "aria-label": tr("portion-quick"),
                        for g in QUICK {
                            button {
                                key: "{g}",
                                class: "btn num",
                                "aria-pressed": if grams() == g { "true" } else { "false" },
                                onclick: move |_| grams.set(g),
                                "{g} g"
                            }
                        }
                    }
                }
            }
            Card { title: tr("portion-how-prepared"),
                fieldset { class: "card-body", style: "border: none; margin: 0;",
                    legend { class: "visually-hidden", {tr("portion-how-prepared")} }
                    div { class: "choice-grid",
                        for p in Preparation::ALL {
                            label { key: "{p.key()}", class: "choice",
                                input {
                                    r#type: "radio",
                                    name: "preparation",
                                    checked: prep() == p,
                                    onchange: move |_| prep.set(p),
                                }
                                span { {labels::preparation(p)} }
                            }
                        }
                    }
                }
            }
            Link {
                class: "btn btn-primary btn-large",
                to: Route::FoodCard { id: id.clone(), grams: grams(), prep: prep().key().to_owned() },
                {tr("action-evaluate")}
            }
        }
    }
}
