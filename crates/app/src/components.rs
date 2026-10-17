//! Shared building blocks of the UI.

use dioxus::prelude::*;
use healthyteller_core::{DataQuality, Level, QualityReport, assess};

use crate::Route;
use crate::i18n::{tr, tr_args};
use crate::state::use_app_state;

#[derive(Clone, Copy, PartialEq)]
pub enum IconName {
    Back,
    Close,
    Search,
    Barcode,
    Plate,
    Lock,
    Info,
    Plus,
    Trash,
    Up,
    Down,
    ChevronDown,
    ChevronRight,
    Print,
    Menu,
    Settings,
}

/// Decorative icon; the surrounding control carries the accessible name.
#[component]
pub fn Icon(name: IconName, #[props(default = 24)] size: u32) -> Element {
    let paths: &[&str] = match name {
        IconName::Back => &["M15 18l-6-6 6-6"],
        IconName::Close => &["M6 6l12 12M18 6L6 18"],
        IconName::Search => &["M10.5 17a6.5 6.5 0 1 0 0-13 6.5 6.5 0 0 0 0 13zM15.5 15.5L20 20"],
        IconName::Barcode => &["M4 7V4h3M17 4h3v3M20 17v3h-3M7 20H4v-3M8 8v8M11 8v8M14 8v8M17 8v8"],
        IconName::Plate => {
            &["M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM12 16a4 4 0 1 0 0-8 4 4 0 0 0 0 8z"]
        }
        IconName::Lock => &["M6 11h12v9H6zM8.5 11V8a3.5 3.5 0 0 1 7 0v3"],
        IconName::Info => &["M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18zM12 11v6M12 7.5v.01"],
        IconName::Plus => &["M12 5v14M5 12h14"],
        IconName::Trash => &["M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"],
        IconName::Up => &["M6 15l6-6 6 6"],
        IconName::Down => &["M6 9l6 6 6-6"],
        IconName::ChevronDown => &["M6 9l6 6 6-6"],
        IconName::ChevronRight => &["M9 6l6 6-6 6"],
        IconName::Print => &["M7 9V3h10v6M7 17H4v-7h16v7h-3M7 14h10v7H7z"],
        IconName::Menu => &["M4 7h16M4 12h16M4 17h16"],
        IconName::Settings => &[
            "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19 12l2-1-1-3-2 .2-1.2-1.6.6-2L15 3l-1.5 1.5h-3L9 3 6.6 4.6l.6 2L6 8.2 4 8l-1 3 2 1-2 1 1 3 2-.2 1.2 1.6-.6 2L9 21l1.5-1.5h3L15 21l2.4-1.6-.6-2 1.2-1.6 2 .2 1-3z",
        ],
    };
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            for d in paths {
                path { d: "{d}" }
            }
        }
    }
}

/// Logo: plate with a check mark.
#[component]
pub fn Logo(#[props(default = 40)] size: u32) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 40 40",
            fill: "none",
            "aria-hidden": "true",
            circle { cx: "20", cy: "20", r: "17.5", stroke: "currentColor", stroke_width: "3" }
            circle { cx: "20", cy: "20", r: "11", stroke: "currentColor", stroke_width: "3" }
            path {
                d: "M15 20.5l3.5 3.5 7-7.5",
                stroke: "var(--kt-gut)",
                stroke_width: "3.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

/// Verdict as shape, icon and word, never colour alone.
#[component]
pub fn Badge(level: Level, #[props(default = false)] small: bool) -> Element {
    let class = format!(
        "badge badge-{}{}",
        level.key(),
        if small { " small" } else { "" }
    );
    let size = if small { 18 } else { 20 };
    let word = tr(&format!("level-{}", level.key()));
    rsx! {
        span { class: "{class}",
            svg {
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.4",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                    match level {
                    Level::Ok => rsx! {
                        circle { cx: "12", cy: "12", r: "9" }
                        path { d: "M8 12.3l2.8 2.8L16.2 9.5" }
                    },
                    Level::Caution => rsx! {
                        path { d: "M12 3.5L2.5 20h19z" }
                        path { d: "M12 10v4.5M12 17.5v.01" }
                    },
                    Level::Avoid => rsx! {
                        path { d: "M8.2 2.5h7.6l5.7 5.7v7.6l-5.7 5.7H8.2l-5.7-5.7V8.2z" }
                        path { d: "M9 9l6 6M15 9l-6 6" }
                    },
                    Level::Unknown => rsx! {
                        circle { cx: "12", cy: "12", r: "9", stroke_dasharray: "3 3" }
                        path { d: "M9.5 9.5a2.5 2.5 0 1 1 3.5 2.3c-.6.3-1 .9-1 1.6v.6M12 17.5v.01" }
                    },
                }
            }
            "{word}"
        }
    }
}

/// Data quality of a stored food for the active profile, or `None` if the food is not stored.
pub fn quality_of(id: &str) -> Option<QualityReport> {
    let state = use_app_state();
    let s = state.read();
    let food = s.food(id)?;
    Some(assess(food, s.active_profile(), &s.rules))
}

/// Data quality as fill symbol and word, never colour alone. Compact rows show
/// the symbol and keep the word for screen readers and the tooltip.
#[component]
pub fn QualityMark(id: String, #[props(default = false)] compact: bool) -> Element {
    let Some(report) = quality_of(&id) else {
        return rsx! {};
    };
    let level = report.level;
    let word = tr(&format!("quality-{}", level.key()));
    let label = tr_args("quality-label", &[("level", word.clone())]);
    rsx! {
        span { class: "quality quality-{level.key()}", title: "{label}",
            span { class: "quality-symbol", "aria-hidden": "true", "{level.symbol()}" }
            if compact {
                span { class: "visually-hidden", "{label}" }
            } else {
                span { "{word}" }
            }
        }
    }
}

/// What the data quality means for this food: which values are missing or how they came about.
#[component]
pub fn QualityExplanation(id: String) -> Element {
    let Some(report) = quality_of(&id) else {
        return rsx! {};
    };
    let mut lines = Vec::new();
    if !report.missing.is_empty() {
        let names: Vec<String> = report
            .missing
            .iter()
            .map(|n| crate::labels::nutrient(*n))
            .collect();
        lines.push(tr_args("quality-missing", &[("list", names.join(", "))]));
    }
    if report.allergens_unknown {
        lines.push(tr("quality-allergens-unknown"));
    }
    if report.level != DataQuality::Incomplete {
        lines.push(tr(&format!("quality-{}-hint", report.level.key())));
    }
    rsx! {
        div { class: "quality-explanation small",
            QualityMark { id: id.clone() }
            for line in lines { p { class: "muted", "{line}" } }
        }
    }
}

/// Card with its heading on the top edge, like a fieldset.
#[component]
pub fn Card(
    title: String,
    #[props(default)] class: String,
    #[props(default)] id: String,
    children: Element,
) -> Element {
    let heading_id = if id.is_empty() {
        format!(
            "card-{}",
            title
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric(), "-")
        )
    } else {
        id
    };
    rsx! {
        section { class: "card {class}", "aria-labelledby": "{heading_id}",
            h2 { class: "card-title", id: "{heading_id}", "{title}" }
            {children}
        }
    }
}

/// Top bar with back link, title and the active profile.
#[component]
pub fn TopBar(
    back: Option<Route>,
    title: String,
    #[props(default = true)] show_profile: bool,
) -> Element {
    let state = use_app_state();
    let profile = state.read().active_profile().cloned();
    rsx! {
        header { class: "topbar no-print",
            if let Some(back) = back {
                Link { class: "icon-btn", to: back, "aria-label": tr("nav-back"),
                    Icon { name: IconName::Back }
                }
            }
            p { class: "topbar-title", "{title}" }
            if show_profile {
                if let Some(p) = profile {
                    Link { class: "profile-chip", to: Route::Profiles {},
                        span { class: "avatar", "aria-hidden": "true", "{p.initials()}" }
                        // Visible first name stays part of the accessible name (WCAG 2.5.3).
                        span { {p.name.split_whitespace().next().unwrap_or_default().to_owned()} }
                        span { class: "visually-hidden", {tr_args("nav-profile-switch", &[("name", p.name.clone())])} }
                    }
                }
            }
        }
    }
}

/// The mandatory notices in their fixed wording (AGENTS.md rule 19).
#[derive(Clone, Copy, PartialEq)]
pub enum NoticeKind {
    Basic,
    Doctor,
    Variation,
    Estimates,
    Allergens,
}

impl NoticeKind {
    fn key(self) -> &'static str {
        match self {
            NoticeKind::Basic => "basic",
            NoticeKind::Doctor => "doctor",
            NoticeKind::Variation => "variation",
            NoticeKind::Estimates => "estimates",
            NoticeKind::Allergens => "allergens",
        }
    }
}

#[component]
pub fn Notice(kind: NoticeKind, #[props(default)] class: String) -> Element {
    let key = kind.key();
    rsx! {
        p { class: "notice {class}", "data-notice": "{key}",
            strong { {tr(&format!("notice-{key}-title"))} }
            " "
            {tr(&format!("notice-{key}"))}
        }
    }
}

/// Data status notice with source and date.
#[component]
pub fn DataStatusNotice(source: String, date: String) -> Element {
    rsx! {
        p { class: "notice", "data-notice": "data-status",
            strong { {tr("notice-data-status-title")} }
            " "
            {tr_args("notice-data-status", &[("source", source), ("date", date)])}
        }
    }
}
