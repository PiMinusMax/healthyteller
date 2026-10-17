//! 01 Grundhinweis: first start, the basic notice has to be confirmed.

use dioxus::prelude::*;

use crate::components::{Logo, Notice, NoticeKind};
use crate::i18n::tr;
use crate::state::{update, use_app_state};

#[component]
pub fn Welcome() -> Element {
    let state = use_app_state();
    let mut confirmed = use_signal(|| false);
    rsx! {
        main { class: "screen", style: "padding-top: 32px;",
            Logo { size: 64 }
            h1 { class: "title", {tr("welcome-title")} }
            p { class: "lead", {tr("welcome-lead")} }
            Notice { kind: NoticeKind::Basic, class: "notice-box" }
            p { class: "small muted", {tr("welcome-where")} }
            div { class: "card", style: "margin-top: 0;",
                label { class: "check-row",
                    input {
                        r#type: "checkbox",
                        checked: confirmed(),
                        onchange: move |e| confirmed.set(e.checked()),
                    }
                    span { class: "grow", style: "font-weight: 800;", {tr("welcome-confirm")} }
                }
            }
            button {
                class: "btn btn-primary btn-large",
                disabled: !confirmed(),
                onclick: move |_| update(state, |s| s.settings.accepted_notice = true),
                {tr("action-continue")}
            }
        }
    }
}
