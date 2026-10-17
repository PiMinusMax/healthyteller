use dioxus::prelude::*;

mod i18n;

use i18n::tr;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { {tr("app-title")} }
        main {
            h1 { {tr("app-title")} }
            p { {tr("app-status")} }
        }
    }
}
