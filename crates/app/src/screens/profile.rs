//! 17/18/19 Profil: person, conditions and allergies with focus, doctor's limits.

use dioxus::prelude::*;
use healthyteller_core::{
    Allergen, Condition, DoctorLimit, Focus, Nutrient, Profile, Quantity, RuleBasis, Sex,
};

use crate::Route;
use crate::components::{Card, Icon, IconName, TopBar};
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{update, use_app_state};

const STEPS: u8 = 3;

/// Profile being edited across the wizard steps.
#[derive(Clone, Copy)]
pub struct ProfileDraft(pub Signal<Option<Profile>>);

/// Allergens shown before "show all".
const COMMON_ALLERGENS: [Allergen; 4] = [
    Allergen::Eggs,
    Allergen::Milk,
    Allergen::Peanuts,
    Allergen::Nuts,
];

/// Metrics a doctor's limit can be entered for, per condition.
pub(crate) fn doctor_metrics(conditions: &std::collections::BTreeSet<Condition>) -> Vec<Nutrient> {
    let mut list = Vec::new();
    for c in conditions {
        let add: &[Nutrient] = match c {
            Condition::Gout => &[Nutrient::Purine],
            Condition::Hypertension => &[Nutrient::Salt],
            Condition::Diabetes => &[Nutrient::Sugars, Nutrient::Carbohydrate],
            Condition::KidneyDisease => &[
                Nutrient::Potassium,
                Nutrient::Phosphorus,
                Nutrient::Protein,
                Nutrient::Salt,
            ],
            Condition::OxalateStones => &[Nutrient::Oxalate],
            Condition::LiverValues | Condition::HistamineIntolerance => &[],
        };
        for n in add {
            if !list.contains(n) {
                list.push(*n);
            }
        }
    }
    list
}

#[component]
pub fn Profiles() -> Element {
    let state = use_app_state();
    let s = state.read();
    let profiles = s.profiles.clone();
    let active = s.active_profile.clone();
    drop(s);
    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("profiles-topbar"), show_profile: false }
            h1 { class: "title", {tr("profiles-title")} }
            Card { title: tr("profiles-list"), id: "profiles-list",
                if profiles.is_empty() {
                    p { class: "card-body muted", {tr("profiles-empty")} }
                }
                ul { class: "list",
                    for p in profiles {
                        li { key: "{p.id}", class: "food-row",
                            label { class: "check-row grow", style: "padding: 0;",
                                input {
                                    r#type: "radio",
                                    name: "active-profile",
                                    checked: active.as_deref() == Some(p.id.as_str()),
                                    onchange: {
                                        let id = p.id.clone();
                                        move |_| update(state, |s| s.active_profile = Some(id.clone()))
                                    },
                                }
                                span { class: "avatar", "{p.initials()}" }
                                span { class: "grow", style: "display: flex; flex-direction: column;",
                                    span { class: "food-name", "{p.name}" }
                                    span { class: "small muted", {labels::profile_summary(&p)} }
                                }
                            }
                            Link { class: "btn btn-quiet", to: Route::ProfileEdit { id: p.id.clone(), step: 1 }, {tr("action-change")} }
                        }
                    }
                }
            }
            Link { class: "btn btn-primary btn-large", to: Route::ProfileEdit { id: "neu".into(), step: 1 }, Icon { name: IconName::Plus } {tr("profiles-new")} }
            if active.is_some() {
                Link { class: "btn btn-secondary btn-large", to: Route::DoctorQuestions {}, {tr("questions-link")} }
            }
            p { class: "status-line", Icon { name: IconName::Lock, size: 20 } {tr("profiles-local")} }
        }
    }
}

#[component]
pub fn ProfileEdit(id: String, step: u8) -> Element {
    let state = use_app_state();
    let ProfileDraft(mut draft) = use_context::<ProfileDraft>();
    let step = step.clamp(1, STEPS);

    // Start a draft from the stored profile, or a new one.
    let needs_init = draft
        .read()
        .as_ref()
        .is_none_or(|d| d.id != id && !(id == "neu" && d.id.starts_with("new:")));
    if needs_init {
        let s = state.read();
        let profile = s
            .profiles
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .unwrap_or_else(|| Profile::new(format!("new:{}", s.new_profile_id()), ""));
        drop(s);
        draft.set(Some(profile));
    }
    let Some(p) = draft() else { return rsx! {} };
    let is_new = p.id.starts_with("new:");
    let title = if is_new {
        tr("profile-new-title")
    } else {
        tr_args("profile-edit-title", &[("name", p.name.clone())])
    };
    let show_all = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let next = {
        let id = id.clone();
        move |_| {
            if step == 1 && draft().is_some_and(|d| d.name.trim().is_empty()) {
                error.set(Some(tr("profile-name-missing")));
                return;
            }
            error.set(None);
            navigator().push(Route::ProfileEdit {
                id: id.clone(),
                step: step + 1,
            });
        }
    };
    let save = move |_| {
        let Some(mut p) = draft() else { return };
        if let Some(stripped) = p.id.strip_prefix("new:") {
            p.id = stripped.to_owned();
        }
        let id = p.id.clone();
        update(state, |s| {
            match s.profiles.iter_mut().find(|x| x.id == id) {
                Some(existing) => *existing = p,
                None => s.profiles.push(p),
            }
            s.active_profile = Some(id);
        });
        draft.set(None);
        navigator().push(Route::Home {});
    };

    rsx! {
        main { class: "screen",
            header { class: "topbar",
                button {
                    class: "icon-btn",
                    onclick: move |_| {
                        draft.set(None);
                        navigator().push(Route::Profiles {});
                    },
                    Icon { name: IconName::Close }
                    span { class: "visually-hidden", {tr("action-cancel")} }
                }
                p { class: "topbar-title", "{title}" }
            }
            div { class: "steps", "aria-hidden": "true",
                for i in 1..=STEPS { span { key: "{i}", class: if i <= step { "done" } else { "" } } }
            }
            p { class: "small muted", {tr_args("profile-step", &[("step", step.to_string()), ("total", STEPS.to_string()), ("name", tr(&format!("profile-step-{step}")))])} }
            match step {
                1 => rsx! { StepPerson { draft } },
                2 => rsx! { StepConditions { draft, show_all } },
                _ => rsx! { StepDoctor { draft } },
            }
            if let Some(e) = error() {
                p { class: "error", role: "alert", "{e}" }
            }
            div { class: "btn-row",
                if step > 1 {
                    Link { class: "btn btn-secondary btn-large", to: Route::ProfileEdit { id: id.clone(), step: step - 1 }, {tr("action-back")} }
                }
                if step < STEPS {
                    button { class: "btn btn-primary btn-large", onclick: next, {tr("action-continue")} }
                } else {
                    button { class: "btn btn-primary btn-large", onclick: save, {tr("action-save")} }
                }
            }
            if !is_new && step == 1 {
                button {
                    class: "btn btn-danger btn-large",
                    onclick: move |_| {
                        let pid = id.clone();
                        update(state, |s| {
                            s.profiles.retain(|x| x.id != pid);
                            if s.active_profile.as_deref() == Some(pid.as_str()) {
                                s.active_profile = s.profiles.first().map(|x| x.id.clone());
                            }
                        });
                        draft.set(None);
                        navigator().push(Route::Profiles {});
                    },
                    {tr("profile-delete")}
                }
            }
        }
    }
}

#[component]
fn StepPerson(draft: Signal<Option<Profile>>) -> Element {
    let p = draft().unwrap_or_else(|| Profile::new("", ""));
    let today = crate::date::today();
    let mut edit = move |f: Box<dyn FnOnce(&mut Profile)>| {
        if let Some(p) = draft.write().as_mut() {
            f(p);
        }
    };
    rsx! {
        div {
            h1 { class: "title", {tr("profile-person-title")} }
            p { class: "lead", {tr("profile-person-lead")} }
        }
        Card { title: tr("profile-person"),
            div { class: "card-body",
                div { class: "field",
                    label { r#for: "profile-name", {tr("profile-name")} }
                    input {
                        id: "profile-name",
                        class: "input",
                        autocomplete: "off",
                        value: "{p.name}",
                        oninput: move |e| { let v = e.value(); edit(Box::new(move |p| p.name = v)) },
                    }
                    p { class: "small muted", {tr("profile-name-hint")} }
                }
                div { class: "field",
                    label { r#for: "profile-birth-date", {tr("profile-birth-date")} }
                    input {
                        id: "profile-birth-date",
                        class: "input num",
                        r#type: "date",
                        style: "max-width: 240px;",
                        max: "{today}",
                        value: p.birth_date.clone().unwrap_or_default(),
                        oninput: move |e| {
                            let v = e.value();
                            let v = (!v.trim().is_empty()).then_some(v);
                            edit(Box::new(move |p| p.birth_date = v))
                        },
                    }
                    if let Some(age) = p.age_on(&today) {
                        p { class: "small muted", {tr_args("profile-age-today", &[("age", age.to_string())])} }
                    }
                }
                fieldset { class: "field", style: "border: none; padding: 0; margin: 0;",
                    legend { class: "label", style: "margin-bottom: 6px;", {tr("profile-sex")} }
                    div { class: "choice-grid",
                        for (sex, key) in [(Sex::Female, "sex-female"), (Sex::Male, "sex-male"), (Sex::Diverse, "sex-diverse")] {
                            label { key: "{key}", class: "choice",
                                input { r#type: "radio", name: "sex", checked: p.sex == Some(sex), onchange: move |_| edit(Box::new(move |p| p.sex = Some(sex))) }
                                span { {tr(key)} }
                            }
                        }
                    }
                    p { class: "small muted", {tr("profile-reference-hint")} }
                }
            }
        }
    }
}

#[component]
fn StepConditions(draft: Signal<Option<Profile>>, show_all: Signal<bool>) -> Element {
    let p = draft().unwrap_or_else(|| Profile::new("", ""));
    let allergens: Vec<Allergen> = if show_all() {
        Allergen::ALL.to_vec()
    } else {
        let mut list = COMMON_ALLERGENS.to_vec();
        list.extend(p.allergies.iter().filter(|a| !COMMON_ALLERGENS.contains(a)));
        list
    };
    let focus = p.focus();
    rsx! {
        div {
            h1 { class: "title", {tr("profile-conditions-title")} }
            p { class: "lead", {tr("profile-conditions-lead")} }
        }
        Card { title: tr("profile-conditions"), id: "profile-conditions",
            for c in Condition::ALL {
                label { key: "{c.key()}", class: "check-row",
                    input {
                        r#type: "checkbox",
                        checked: p.conditions.contains(&c),
                        onchange: move |e| {
                            if let Some(p) = draft.write().as_mut() {
                                if e.checked() { p.conditions.insert(c); } else { p.conditions.remove(&c); }
                            }
                        },
                    }
                    span { class: "grow", {labels::condition(c)} }
                }
            }
        }
        Card { title: tr("profile-allergies"), id: "profile-allergies",
            for a in allergens {
                label { key: "{a.key()}", class: "check-row",
                    input {
                        r#type: "checkbox",
                        checked: p.allergies.contains(&a),
                        onchange: move |e| {
                            if let Some(p) = draft.write().as_mut() {
                                if e.checked() { p.allergies.insert(a); } else { p.allergies.remove(&a); }
                            }
                        },
                    }
                    span { class: "grow", {labels::allergen(a)} }
                }
            }
            if !show_all() {
                div { class: "card-row",
                    button { class: "link-btn", onclick: move |_| show_all.set(true), {tr("profile-allergies-all")} }
                }
            }
        }
        Card { title: tr("profile-focus"), id: "profile-focus",
            div { class: "card-body",
                p { class: "small muted", {if p.conditions.is_empty() { tr("profile-focus-auto-nutrients") } else { tr("profile-focus-auto-conditions") }} }
                for (f, key, sub) in [(Focus::Conditions, "focus-conditions", "focus-conditions-sub"), (Focus::Nutrients, "focus-nutrients", "focus-nutrients-sub")] {
                    label { key: "{key}", class: "choice",
                        input {
                            r#type: "radio",
                            name: "focus",
                            checked: focus == f,
                            onchange: move |_| { if let Some(p) = draft.write().as_mut() { p.focus_override = Some(f); } },
                        }
                        span { class: "choice-text",
                            span { style: "font-weight: 800;", {tr(key)} }
                            span { class: "small muted", {tr(sub)} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StepDoctor(draft: Signal<Option<Profile>>) -> Element {
    let p = draft().unwrap_or_else(|| Profile::new("", ""));
    let metrics = doctor_metrics(&p.conditions);
    let note = p
        .doctor_limits
        .iter()
        .find_map(|l| l.note.clone())
        .unwrap_or_default();
    rsx! {
        div {
            h1 { class: "title", {tr("profile-doctor-title")} }
            p { class: "lead", {tr("profile-doctor-lead")} }
        }
        if metrics.is_empty() {
            p { class: "notice", {tr("profile-doctor-none")} }
        } else {
            Card { title: tr("profile-doctor-per-day"), id: "profile-doctor",
                div { class: "card-body",
                    for n in metrics {
                        DoctorField { key: "{n.key()}", draft, nutrient: n }
                    }
                }
            }
            Card { title: tr("profile-doctor-origin"),
                div { class: "card-body",
                    div { class: "field",
                        label { r#for: "doctor-note", {tr("profile-doctor-note")} }
                        input {
                            id: "doctor-note",
                            class: "input",
                            value: "{note}",
                            oninput: move |e| {
                                let v = e.value();
                                if let Some(p) = draft.write().as_mut() {
                                    for l in p.doctor_limits.iter_mut() { l.note = (!v.trim().is_empty()).then(|| v.clone()); }
                                }
                            },
                        }
                        p { class: "small muted", {tr("profile-doctor-note-hint")} }
                    }
                }
            }
        }
        div { class: "notice",
            p { style: "display: flex; gap: 10px;", Icon { name: IconName::Info } {tr("profile-doctor-no-dose")} }
            p { style: "display: flex; gap: 10px; margin-top: 10px;", Icon { name: IconName::Lock } {tr("profiles-local")} }
        }
    }
}

#[component]
fn DoctorField(draft: Signal<Option<Profile>>, nutrient: Nutrient) -> Element {
    let p = draft().unwrap_or_else(|| Profile::new("", ""));
    let unit = nutrient.unit();
    let current = p
        .doctor_limit(nutrient, RuleBasis::Day)
        .and_then(|l| l.max)
        .and_then(|q| q.to(unit))
        .map(|q| format::number(q.amount))
        .unwrap_or_default();
    let id = format!("doctor-{}", nutrient.key());
    rsx! {
        div { class: "field",
            label { r#for: "{id}", {labels::nutrient(nutrient)} }
            div { class: "input-unit",
                input {
                    id: "{id}",
                    class: "input num",
                    inputmode: "decimal",
                    value: "{current}",
                    onchange: move |e| {
                        let value = format::parse_number(&e.value());
                        if let Some(p) = draft.write().as_mut() {
                            let note = p.doctor_limits.iter().find_map(|l| l.note.clone());
                            p.doctor_limits.retain(|l| !(l.nutrient == nutrient && l.basis == RuleBasis::Day));
                            if let Some(v) = value {
                                p.doctor_limits.push(DoctorLimit {
                                    nutrient,
                                    max: Some(Quantity::new(v, unit)),
                                    min: None,
                                    basis: RuleBasis::Day,
                                    note,
                                });
                            }
                        }
                    },
                }
                span { class: "num", style: "font-size: 20px; min-width: 40px;", {unit.symbol()} }
            }
            p { class: "small muted", {if current.is_empty() { tr("profile-doctor-empty") } else { tr("profile-doctor-set") }} }
        }
    }
}
