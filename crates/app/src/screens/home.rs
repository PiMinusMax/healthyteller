//! 02 Start: two main actions, check food and look at the plate.

use dioxus::prelude::*;

use crate::Route;
use crate::components::{Card, Icon, IconName, Logo};
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::use_app_state;

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
    drop(s);
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
            div { class: "footer-note",
                span { class: "status-line", Icon { name: IconName::Lock, size: 20 } {tr("home-local")} }
                span { {tr("home-no-diagnosis")} }
            }
        }
    }
}
