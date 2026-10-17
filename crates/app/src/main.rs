use dioxus::prelude::*;

mod components;
mod date;
mod fetch;
mod format;
mod i18n;
mod labels;
#[cfg(target_arch = "wasm32")]
mod scanner;
mod screens;
mod state;

use i18n::tr;
use screens::*;
use state::{AppState, Theme, use_app_state};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const NEXT_400: Asset = asset!("/assets/fonts/atkinson-hyperlegible-next-latin-400-normal.woff2");
const NEXT_600: Asset = asset!("/assets/fonts/atkinson-hyperlegible-next-latin-600-normal.woff2");
const NEXT_700: Asset = asset!("/assets/fonts/atkinson-hyperlegible-next-latin-700-normal.woff2");
const NEXT_800: Asset = asset!("/assets/fonts/atkinson-hyperlegible-next-latin-800-normal.woff2");
const MONO_400: Asset = asset!("/assets/fonts/atkinson-hyperlegible-mono-latin-400-normal.woff2");
const MONO_600: Asset = asset!("/assets/fonts/atkinson-hyperlegible-mono-latin-600-normal.woff2");

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/pruefen")]
        Search {},
        #[route("/portion/:id")]
        PortionScreen { id: String },
        #[route("/karte/:id/:grams/:prep")]
        FoodCard { id: String, grams: u32, prep: String },
        #[route("/naehrwerte/:id/:grams/:prep")]
        NutrientList { id: String, grams: u32, prep: String },
        #[route("/teller")]
        Plate {},
        #[route("/meine-lebensmittel")]
        MyFoods {},
        #[route("/profile")]
        Profiles {},
        #[route("/profil/:id/:step")]
        ProfileEdit { id: String, step: u8 },
        #[route("/einstellungen")]
        Settings {},
        #[route("/ueber")]
        About {},
        #[route("/begriff/:term")]
        Term { term: String },
        #[route("/druckliste")]
        PrintList {},
        #[route("/schaetzungen")]
        Estimates {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

fn font_faces() -> String {
    let face = |family: &str, weight: u32, url: Asset| {
        format!(
            "@font-face{{font-family:'{family}';font-style:normal;font-weight:{weight};font-display:swap;src:url('{url}') format('woff2');}}"
        )
    };
    [
        face("Atkinson Hyperlegible Next", 400, NEXT_400),
        face("Atkinson Hyperlegible Next", 600, NEXT_600),
        face("Atkinson Hyperlegible Next", 700, NEXT_700),
        face("Atkinson Hyperlegible Next", 800, NEXT_800),
        face("Atkinson Hyperlegible Mono", 400, MONO_400),
        face("Atkinson Hyperlegible Mono", 600, MONO_600),
    ]
    .concat()
}

/// Path the web app is served under, e.g. `/healthyteller` on GitHub Pages, or empty.
fn web_base() -> String {
    dioxus::cli_config::base_path()
        .map(|b| b.trim_matches('/').to_owned())
        .filter(|b| !b.is_empty())
        .map(|b| format!("/{b}"))
        .unwrap_or_default()
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::load()));
    use_context_provider(|| screens::ProfileDraft(Signal::new(None)));
    let base = web_base();
    // Installable web app that also works offline (files in public/).
    if cfg!(target_arch = "wasm32") {
        let sw = format!("{base}/sw.js");
        use_hook(move || {
            document::eval(&format!(
                "if ('serviceWorker' in navigator) {{ navigator.serviceWorker.register('{sw}').catch(() => {{}}); }}"
            ));
        });
    }
    rsx! {
        document::Title { {tr("app-title")} }
        document::Stylesheet { href: MAIN_CSS }
        document::Style { {font_faces()} }
        if cfg!(target_arch = "wasm32") {
            document::Link { rel: "manifest", href: "{base}/manifest.webmanifest" }
            document::Link { rel: "apple-touch-icon", href: "{base}/icons/apple-touch-icon.png" }
            document::Link { rel: "icon", r#type: "image/png", href: "{base}/icons/icon-192.png" }
            document::Meta { name: "theme-color", content: "#1D3B8F" }
            document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
            document::Meta { name: "apple-mobile-web-app-title", content: "Healthy Teller" }
        }
        Router::<Route> {}
    }
}

/// Wraps every screen: theme, and the basic notice until it has been confirmed.
#[component]
fn Shell() -> Element {
    let state = use_app_state();
    let route = use_route::<Route>();
    let (accepted, theme, language) = {
        let s = state.read();
        (
            s.settings.accepted_notice,
            s.settings.theme,
            s.settings.language,
        )
    };
    i18n::set_language(language);
    let lang = language.code();
    use_effect(use_reactive!(|lang| {
        document::eval(&format!("document.documentElement.lang = '{lang}';"));
    }));
    let theme = match theme {
        Theme::System => "system",
        Theme::Light => "light",
        Theme::Dark => "dark",
    };
    let show_welcome = !accepted && !matches!(route, Route::About {});
    rsx! {
        // Keyed by language, so every screen renders again with the new texts.
        div { key: "{lang}", class: "app", "data-theme": "{theme}", lang: "{lang}",
            if show_welcome {
                Welcome {}
            } else {
                Outlet::<Route> {}
            }
        }
    }
}
