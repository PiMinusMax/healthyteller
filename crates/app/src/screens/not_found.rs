use dioxus::prelude::*;

use crate::Route;
use crate::components::TopBar;
use crate::i18n::tr;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("app-title") }
            h1 { class: "title", {tr("not-found-title")} }
            Link { class: "btn btn-primary btn-large", to: Route::Home {}, {tr("action-home")} }
        }
    }
}
