//! 20 Einstellungen und Datenquellen.

use dioxus::prelude::*;
use healthyteller_ingest::sources::{Access, SOURCES};

use crate::Route;
use crate::components::{Card, Icon, IconName, TopBar};
use crate::i18n::{tr, tr_args};
use crate::state::{Reference, Theme, update, use_app_state};

#[component]
pub fn Settings() -> Element {
    let state = use_app_state();
    let mut confirm_delete = use_signal(|| false);
    let s = state.read();
    let settings = s.settings.clone();
    let cached = s.foods.len();
    let rule_count = s.rules.len();
    let draft_count = s
        .rules
        .iter()
        .filter(|r| r.status == healthyteller_core::RuleStatus::Draft)
        .count();
    let rule_errors = s.rule_errors.clone();
    drop(s);

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("settings-title") }
            h1 { class: "title", {tr("settings-title")} }

            Card { title: tr("settings-reference"), id: "settings-reference",
                fieldset { class: "card-body", style: "border: none; margin: 0;",
                    legend { class: "small muted", {tr("settings-reference-lead")} }
                    for (r, key, sub) in [(Reference::DgeOege, "reference-dge", "reference-dge-sub"), (Reference::Efsa, "reference-efsa", "reference-efsa-sub")] {
                        label { key: "{key}", class: "choice", style: "margin-top: 10px;",
                            input { r#type: "radio", name: "reference", checked: settings.reference == r, onchange: move |_| update(state, |s| s.settings.reference = r) }
                            span { class: "choice-text", span { style: "font-weight: 800;", {tr(key)} } span { class: "small muted", {tr(sub)} } }
                        }
                    }
                    p { class: "small muted", {tr("settings-reference-pending")} }
                }
            }

            Card { title: tr("settings-theme"), id: "settings-theme",
                fieldset { class: "card-body", style: "border: none; margin: 0;",
                    legend { class: "visually-hidden", {tr("settings-theme")} }
                    div { class: "choice-grid",
                        for (t, key) in [(Theme::System, "theme-system"), (Theme::Light, "theme-light"), (Theme::Dark, "theme-dark")] {
                            label { key: "{key}", class: "choice",
                                input { r#type: "radio", name: "theme", checked: settings.theme == t, onchange: move |_| update(state, |s| s.settings.theme = t) }
                                span { {tr(key)} }
                            }
                        }
                    }
                }
            }

            Card { title: tr("settings-sources"), id: "settings-sources",
                p { class: "card-body small muted", {tr("settings-sources-lead")} }
                ul { class: "list",
                    for src in SOURCES {
                        li { key: "{src.id}", class: "card-row", style: "flex-direction: column; align-items: stretch; gap: 4px;",
                            div { style: "display: flex; justify-content: space-between; gap: 12px; align-items: center;",
                                span { class: "food-name", "{src.name}" }
                                span { class: "small", style: "font-weight: 700;",
                                    {if src.implemented { tr("source-active") } else { match src.access {
                                        Access::PendingPermission => tr("source-permission"),
                                        _ => tr("source-not-yet"),
                                    }}}
                                }
                            }
                            span { class: "small muted", {tr_args("source-license", &[("license", src.license.to_owned())])} }
                            if src.id == "off" {
                                span { class: "small", {tr_args("source-off-cache", &[("count", cached.to_string())])} }
                                button {
                                    class: "btn btn-quiet",
                                    disabled: cached == 0,
                                    onclick: move |_| update(state, |s| {
                                        let keep: Vec<String> = s.my_foods.iter().map(|m| m.food_id.0.clone())
                                            .chain(s.day.entries.iter().map(|e| e.item.food_id.0.clone()))
                                            .chain(s.plates.iter().flat_map(|p| p.items.iter().map(|i| i.food_id.0.clone())))
                                            .collect();
                                        s.foods.retain(|k, _| keep.contains(k));
                                    }),
                                    {tr("source-off-clear")}
                                }
                            }
                            if !src.url.is_empty() {
                                a { class: "small", href: "{src.url}", target: "_blank", rel: "noopener", {tr_args("source-website", &[("name", src.name.to_owned())])} }
                            }
                        }
                    }
                }
            }

            Card { title: tr("settings-rules"), id: "settings-rules",
                div { class: "card-body",
                    p { {tr_args("settings-rules-count", &[("count", rule_count.to_string()), ("drafts", draft_count.to_string())])} }
                    for e in rule_errors { p { class: "error small", "{e}" } }
                    label { class: "check-row", style: "padding: 0;",
                        input { r#type: "checkbox", checked: settings.include_draft_rules, onchange: move |e| update(state, |s| s.settings.include_draft_rules = e.checked()) }
                        span { class: "grow", {tr("settings-rules-drafts")} }
                    }
                }
            }

            nav { class: "card", style: "margin-top: 0;", "aria-label": tr("settings-more"),
                for (route, key, icon) in [
                    (Route::Profiles {}, "profiles-title", IconName::Settings),
                    (Route::MyFoods {}, "my-foods-title", IconName::Menu),
                    (Route::PrintList {}, "print-create", IconName::Print),
                    (Route::DoctorQuestions {}, "questions-link", IconName::Info),
                    (Route::Estimates {}, "estimates-all", IconName::Info),
                    (Route::About {}, "about-link", IconName::Info),
                ] {
                    Link { key: "{key}", class: "card-row", to: route, style: "text-decoration: none; color: var(--kt-ink); font-weight: 700;",
                        Icon { name: icon }
                        span { style: "flex: 1;", {tr(key)} }
                        Icon { name: IconName::ChevronRight }
                    }
                }
            }

            super::transfer::Transfer {}

            Card { title: tr("settings-device"), id: "settings-device",
                div { class: "card-body",
                    p { class: "small", {tr("settings-device-lead")} }
                    if confirm_delete() {
                        p { class: "error", role: "alert", {tr("settings-delete-confirm")} }
                        div { class: "btn-row",
                            button { class: "btn btn-secondary", onclick: move |_| confirm_delete.set(false), {tr("action-cancel")} }
                            button {
                                class: "btn btn-danger",
                                onclick: move |_| {
                                    let mut st = state;
                                    st.write().clear_all();
                                    confirm_delete.set(false);
                                },
                                {tr("settings-delete-final")}
                            }
                        }
                    } else {
                        button { class: "btn btn-danger", onclick: move |_| confirm_delete.set(true), {tr("settings-delete")} }
                    }
                }
            }
        }
    }
}
