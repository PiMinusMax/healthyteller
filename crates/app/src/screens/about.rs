//! Über die App: the basic notice permanently, license and sources.

use dioxus::prelude::*;

use crate::Route;
use crate::components::{Card, Logo, Notice, NoticeKind, TopBar};
use crate::i18n::{tr, tr_args};

#[component]
pub fn About() -> Element {
    rsx! {
        main { class: "screen",
            TopBar { back: Route::Settings {}, title: tr("about-link") }
            div { style: "display: flex; align-items: center; gap: 14px;",
                Logo { size: 56 }
                h1 { class: "title", {tr("app-title")} }
            }
            p { class: "muted", {tr_args("about-version", &[("version", env!("CARGO_PKG_VERSION").to_owned())])} }
            Notice { kind: NoticeKind::Basic, class: "notice-box" }
            Notice { kind: NoticeKind::Doctor }
            Card { title: tr("about-license"),
                div { class: "card-body",
                    p { {tr("about-license-text")} }
                    a { href: "https://github.com/PiMinusMax/healthyteller", target: "_blank", rel: "noopener", {tr("about-source-code")} }
                }
            }
            Card { title: tr("about-credits"),
                div { class: "card-body",
                    p { {tr("about-credits-off")} }
                    p { {tr("about-credits-fonts")} }
                }
            }
        }
    }
}
