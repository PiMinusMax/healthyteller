//! 02 Start: two main actions, check food and look at the plate.

use dioxus::prelude::*;

use crate::Route;
use crate::components::{Card, Icon, IconName, Logo};
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{update, use_app_state};

/// Own foods shown on the start page; the rest is one tap away.
const HOME_MY_FOODS: usize = 5;

#[component]
pub fn Home() -> Element {
    let state = use_app_state();
    let s = state.read();
    let profile = s.active_profile().cloned();
    let today_count = if s.day.date == crate::date::today() {
        s.day.entries.len()
    } else {
        0
    };
    let my_foods: Vec<(usize, String, String)> = s
        .my_foods
        .iter()
        .enumerate()
        .take(HOME_MY_FOODS)
        .filter_map(|(i, m)| {
            s.food(&m.food_id.0)
                .map(|f| (i, f.name.clone(), labels::portion(&m.portion)))
        })
        .collect();
    let more = s.my_foods.len().saturating_sub(HOME_MY_FOODS);
    drop(s);
    let mut added = use_signal(|| None::<String>);
    let plate_line = if today_count == 0 {
        tr("home-plate-empty")
    } else {
        tr_args("home-plate-count", &[("count", today_count.to_string())])
    };
    rsx! {
        main { class: "screen",
            header { class: "topbar",
                span { class: "brand", Logo { size: 40 } h1 { style: "font-size: 26px; font-weight: 800;", {tr("app-title")} } }
                Link { class: "icon-btn", to: Route::Settings {},
                    Icon { name: IconName::Menu }
                    span { class: "visually-hidden", {tr("settings-title")} }
                }
            }
            Card { title: tr("home-checked-for"),
                if let Some(p) = profile {
                    div { class: "card-body", style: "flex-direction: row; align-items: center; gap: 14px;",
                        span { class: "avatar big", "{p.initials()}" }
                        div { style: "flex: 1; display: flex; flex-direction: column;",
                            span { style: "font-size: 22px; font-weight: 800;", "{p.name}" }
                            span { class: "muted", {labels::profile_summary(&p)} }
                        }
                        Link { class: "link-btn", to: Route::Profiles {}, {tr("action-switch")} }
                    }
                } else {
                    div { class: "card-body",
                        p { {tr("home-no-profile")} }
                        Link { class: "btn btn-secondary", to: Route::ProfileEdit { id: "neu".into(), step: 1 }, {tr("action-create-profile")} }
                    }
                }
            }
            Link { class: "tile tile-primary", to: Route::Search {},
                span { class: "tile-icon", Icon { name: IconName::Search, size: 36 } }
                span { class: "tile-title", {tr("home-check-title")} }
                span { style: "font-size: 19px;", {tr("home-check-sub")} }
            }
            Link { class: "tile tile-secondary", to: Route::Plate {},
                span { class: "tile-icon", Icon { name: IconName::Plate, size: 36 } }
                span { class: "tile-title", {tr("home-plate-title")} }
                span { class: "muted", style: "font-size: 19px;", "{plate_line}" }
            }
            Card { title: tr("my-foods-title"), id: "home-my-foods",
                if my_foods.is_empty() {
                    div { class: "card-body",
                        p { {tr("home-my-foods-empty")} }
                        Link { class: "btn btn-secondary btn-large", to: Route::MyFoods {}, Icon { name: IconName::Plus } {tr("home-my-foods-setup")} }
                    }
                } else {
                    p { class: "card-body small muted", {tr("my-foods-hint")} }
                    ul { class: "list",
                        for (i, name, portion) in my_foods {
                            li { key: "{i}", class: "food-row",
                                button {
                                    class: "row-btn grow",
                                    onclick: move |_| {
                                        update(state, |s| {
                                            if let Some(m) = s.my_foods.get(i).cloned() {
                                                s.add_to_plate(m.food_id, m.portion);
                                            }
                                        });
                                        added.set(Some(name.clone()));
                                    },
                                    span { class: "plus", "aria-hidden": "true", "+" }
                                    span { span { class: "food-name", "{name}" } ", {portion}" }
                                }
                            }
                        }
                    }
                    if let Some(name) = added() {
                        p { class: "card-row", role: "status", {tr_args("plate-added", &[("name", name)])} }
                    }
                    div { class: "card-row",
                        Link { class: "link-btn", to: Route::MyFoods {},
                            if more > 0 { {tr_args("home-my-foods-more", &[("count", more.to_string())])} } else { {tr("home-my-foods-edit")} }
                        }
                    }
                }
            }
            div { class: "footer-note",
                span { class: "status-line", Icon { name: IconName::Lock, size: 20 } {tr("home-local")} }
                span { {tr("home-no-diagnosis")} }
            }
        }
    }
}
