//! 20 Einstellungen: language and theme on top, everything else one level down.

use dioxus::prelude::*;
use healthyteller_ingest::sources::{Access, SOURCES, SourceInfo};

use crate::Route;
use crate::components::{Card, Icon, IconName, TopBar};
use crate::i18n::{Language, tr, tr_args};
use crate::state::{Reference, Theme, update, use_app_state};

#[component]
pub fn Settings() -> Element {
    let state = use_app_state();
    let settings = state.read().settings.clone();

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("settings-title") }
            h1 { class: "title", {tr("settings-title")} }

            Card { title: tr("settings-language"), id: "settings-language",
                fieldset { class: "card-body", style: "border: none; margin: 0;",
                    legend { class: "visually-hidden", {tr("settings-language")} }
                    div { class: "choice-grid",
                        for (l, label) in [(Language::De, "Deutsch"), (Language::En, "English")] {
                            label { key: "{label}", class: "choice", lang: l.code(),
                                input { r#type: "radio", name: "language", checked: settings.language == l, onchange: move |_| update(state, |s| s.settings.language = l) }
                                span { "{label}" }
                            }
                        }
                    }
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

            nav { class: "card", style: "margin-top: 0;", "aria-label": tr("settings-more"),
                for (route, key, icon) in [
                    (Route::Profiles {}, "profiles-title", IconName::Settings),
                    (Route::MyFoods {}, "my-foods-title", IconName::Menu),
                    (Route::DataSources {}, "settings-sources", IconName::Info),
                    (Route::DeviceTransfer {}, "transfer-title", IconName::Lock),
                    (Route::PrintList {}, "print-create", IconName::Print),
                    (Route::DoctorQuestions {}, "questions-link", IconName::Info),
                    (Route::Estimates {}, "estimates-all", IconName::Info),
                    (Route::SettingsAdvanced {}, "settings-advanced", IconName::Settings),
                    (Route::About {}, "about-link", IconName::Info),
                ] {
                    Link { key: "{key}", class: "card-row", to: route, style: "text-decoration: none; color: var(--kt-ink); font-weight: 700;",
                        Icon { name: icon }
                        span { style: "flex: 1;", {tr(key)} }
                        Icon { name: IconName::ChevronRight }
                    }
                }
            }
        }
    }
}

/// Data sources: the active ones with their cache and file import, planned ones folded away.
#[component]
pub fn DataSources() -> Element {
    let state = use_app_state();
    let mut import_error = use_signal(|| None::<String>);
    let cached = state.read().foods.len();
    let (active, planned): (Vec<&SourceInfo>, Vec<&SourceInfo>) =
        SOURCES.iter().partition(|s| s.implemented);

    // Fallback route of every source (project rule 12): a product file the person downloaded.
    let import_file = move |e: FormEvent| async move {
        let Some(file) = e.files().into_iter().next() else {
            return;
        };
        match file.read_bytes().await {
            Ok(bytes) => match crate::fetch::parse_product(&bytes) {
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

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Settings {}, title: tr("settings-title") }
            div {
                h1 { class: "title", {tr("settings-sources")} }
                p { class: "muted", {tr("settings-sources-lead")} }
            }
            for src in active {
                Card { key: "{src.id}", title: src.name.to_owned(), id: "source-{src.id}",
                    div { class: "card-body",
                        p { class: "small muted", {tr_args("source-license", &[("license", src.license.to_owned())])} }
                        if src.id == "off" {
                            div { class: "input-row", style: "align-items: center;",
                                span { class: "grow", {tr_args("source-off-cache", &[("count", cached.to_string())])} }
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
                        if !src.url.is_empty() {
                            a { class: "small", href: "{src.url}", target: "_blank", rel: "noopener", {tr_args("source-website", &[("name", src.name.to_owned())])} }
                        }
                    }
                }
            }
            details { class: "card group", id: "sources-planned",
                summary {
                    span { {tr_args("sources-planned", &[("count", planned.len().to_string())])} }
                    Icon { name: IconName::ChevronDown, size: 20 }
                }
                ul { class: "list",
                    for src in planned {
                        li { key: "{src.id}", class: "card-row", style: "flex-direction: column; align-items: stretch; gap: 4px;",
                            span { class: "food-name", "{src.name}" }
                            span { class: "small muted",
                                {match src.access {
                                    Access::PendingPermission => tr("source-permission"),
                                    _ => tr("source-not-yet"),
                                }}
                                " · "
                                {tr_args("source-license", &[("license", src.license.to_owned())])}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DeviceTransfer() -> Element {
    rsx! {
        main { class: "screen",
            TopBar { back: Route::Settings {}, title: tr("settings-title") }
            h1 { class: "title", {tr("transfer-title")} }
            super::transfer::Transfer {}
        }
    }
}

/// Reference values, rule drafts and deleting all data: rarely needed, so one level down.
#[component]
pub fn SettingsAdvanced() -> Element {
    let state = use_app_state();
    let mut confirm_delete = use_signal(|| false);
    let s = state.read();
    let settings = s.settings.clone();
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
            TopBar { back: Route::Settings {}, title: tr("settings-title") }
            h1 { class: "title", {tr("settings-advanced")} }

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
