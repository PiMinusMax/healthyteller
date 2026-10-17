//! Encrypted export and import for moving to another device.

#[cfg(target_arch = "wasm32")]
use base64::Engine;
use dioxus::prelude::*;
use healthyteller_storage::export::{self, ExportError, MIN_PASSPHRASE_CHARS};

use crate::components::Card;
use crate::date;
use crate::i18n::{tr, tr_args};
use crate::state::{ExportData, update, use_app_state};

#[derive(Clone, PartialEq)]
enum Exported {
    None,
    /// Web: a link to save the file.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    Link {
        href: String,
        name: String,
    },
    /// Native: written to this path.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    Saved(String),
    Failed(String),
}

fn error_text(e: ExportError) -> String {
    tr(match e {
        ExportError::PassphraseTooShort => "transfer-pass-short",
        ExportError::NotAnExportFile => "transfer-not-export",
        ExportError::WrongPassphraseOrDamaged => "transfer-wrong-pass",
        ExportError::Random | ExportError::Crypto => "transfer-crypto-error",
    })
}

fn deliver(bytes: Vec<u8>) -> Exported {
    let name = format!("healthyteller-{}.htx", date::today());
    #[cfg(target_arch = "wasm32")]
    {
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        Exported::Link {
            href: format!("data:application/octet-stream;base64,{b64}"),
            name,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let dir = dirs::download_dir().or_else(dirs::home_dir);
        match dir {
            Some(dir) => {
                let path = dir.join(&name);
                match std::fs::write(&path, bytes) {
                    Ok(()) => Exported::Saved(path.display().to_string()),
                    Err(e) => Exported::Failed(e.to_string()),
                }
            }
            None => Exported::Failed(tr("transfer-no-folder")),
        }
    }
}

#[component]
pub fn Transfer() -> Element {
    let state = use_app_state();
    let mut pass = use_signal(String::new);
    let mut pass2 = use_signal(String::new);
    let mut exported = use_signal(|| Exported::None);
    let mut file = use_signal(|| None::<Vec<u8>>);
    let mut import_pass = use_signal(String::new);
    let mut import_result = use_signal(|| None::<Result<(), String>>);

    let make_export = move |_| {
        if pass() != pass2() {
            exported.set(Exported::Failed(tr("transfer-pass-mismatch")));
            return;
        }
        let data = state.read().export_data();
        let json = serde_json::to_vec(&data).unwrap_or_default();
        exported.set(match export::encrypt(&pass(), &json) {
            Ok(bytes) => deliver(bytes),
            Err(e) => Exported::Failed(error_text(e)),
        });
        pass.set(String::new());
        pass2.set(String::new());
    };

    let pick_file = move |e: FormEvent| async move {
        import_result.set(None);
        if let Some(f) = e.files().into_iter().next() {
            file.set(f.read_bytes().await.ok().map(|b| b.to_vec()));
        }
    };

    let run_import = move |_| {
        let Some(bytes) = file() else { return };
        let result = export::decrypt(&import_pass(), &bytes)
            .map_err(error_text)
            .and_then(|json| {
                serde_json::from_slice::<ExportData>(&json).map_err(|_| tr("transfer-not-export"))
            });
        match result {
            Ok(data) => {
                update(state, |s| s.import_data(data));
                import_result.set(Some(Ok(())));
            }
            Err(msg) => import_result.set(Some(Err(msg))),
        }
        import_pass.set(String::new());
    };

    rsx! {
        Card { title: tr("transfer-title"), id: "settings-transfer",
            div { class: "card-body",
                p { class: "small", {tr("transfer-lead")} }
                h3 { class: "section-label", {tr("transfer-export")} }
                div { class: "field",
                    label { r#for: "export-pass", {tr("transfer-pass")} }
                    input { id: "export-pass", class: "input", r#type: "password", autocomplete: "new-password", value: "{pass}", oninput: move |e| pass.set(e.value()) }
                    p { class: "small muted", {tr_args("transfer-pass-hint", &[("min", MIN_PASSPHRASE_CHARS.to_string())])} }
                }
                div { class: "field",
                    label { r#for: "export-pass2", {tr("transfer-pass-repeat")} }
                    input { id: "export-pass2", class: "input", r#type: "password", autocomplete: "new-password", value: "{pass2}", oninput: move |e| pass2.set(e.value()) }
                }
                button { class: "btn btn-secondary", disabled: pass().is_empty(), onclick: make_export, {tr("transfer-export-action")} }
                match exported() {
                    Exported::None => rsx! {},
                    Exported::Link { href, name } => rsx! {
                        a { class: "btn btn-primary", href: "{href}", download: "{name}", {tr("transfer-save-file")} }
                    },
                    Exported::Saved(path) => rsx! { p { role: "status", {tr_args("transfer-saved", &[("path", path)])} } },
                    Exported::Failed(msg) => rsx! { p { class: "error", role: "alert", "{msg}" } },
                }

                h3 { class: "section-label", style: "margin-top: 12px;", {tr("transfer-import")} }
                div { class: "field",
                    label { r#for: "import-export-file", {tr("transfer-file")} }
                    input { id: "import-export-file", r#type: "file", accept: ".htx", onchange: pick_file }
                }
                div { class: "field",
                    label { r#for: "import-pass", {tr("transfer-pass")} }
                    input { id: "import-pass", class: "input", r#type: "password", autocomplete: "off", value: "{import_pass}", oninput: move |e| import_pass.set(e.value()) }
                }
                p { class: "small muted", {tr("transfer-import-warning")} }
                button { class: "btn btn-secondary", disabled: file().is_none() || import_pass().is_empty(), onclick: run_import, {tr("transfer-import-action")} }
                match import_result() {
                    Some(Ok(())) => rsx! { p { role: "status", {tr("transfer-imported")} } },
                    Some(Err(msg)) => rsx! { p { class: "error", role: "alert", "{msg}" } },
                    None => rsx! {},
                }
            }
        }
    }
}
