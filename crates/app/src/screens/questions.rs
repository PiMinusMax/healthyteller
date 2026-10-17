//! 24 Fragen an den Arzt: open questions from the profile, chosen for a printed report.
//! The app collects questions; it never answers them (AGENTS.md rule 7).

use std::collections::BTreeSet;

use dioxus::prelude::*;
use healthyteller_core::{Reason, RuleBasis, evaluate};

use crate::Route;
use crate::components::{Card, Icon, IconName, Notice, NoticeKind, TopBar};
use crate::date;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::screens::profile::doctor_metrics;
use crate::state::use_app_state;

#[derive(Clone, PartialEq)]
struct Question {
    key: String,
    topic: String,
    kind: &'static str,
    context: String,
    question: String,
}

#[component]
pub fn DoctorQuestions() -> Element {
    let state = use_app_state();
    let mut excluded = use_signal(BTreeSet::<String>::new);
    let mut own = use_signal(String::new);
    let s = state.read();
    let Some(profile) = s.active_profile().cloned() else {
        drop(s);
        return rsx! {
            main { class: "screen",
                TopBar { back: Route::Home {}, title: tr("questions-topbar") }
                h1 { class: "title", {tr("questions-title")} }
                p { class: "notice", {tr("card-no-profile")} }
            }
        };
    };

    let mut questions: Vec<Question> = Vec::new();

    // Rules of the profile that pull a metric in opposite directions.
    if let Some(food) = s.foods.values().next().map(|c| c.food.clone()) {
        let e = evaluate(
            &food,
            &crate::state::default_portion(),
            &profile,
            &s.rules,
            &Default::default(),
            s.eval_options(),
        );
        for c in e.conflicts {
            let n = labels::nutrient(c.nutrient);
            questions.push(Question {
                key: format!("conflict-{}", c.nutrient.key()),
                topic: n.clone(),
                kind: "conflict",
                context: tr_args(
                    "questions-conflict-context",
                    &[
                        ("upper", labels::condition(c.upper.0)),
                        ("lower", labels::condition(c.lower.0)),
                    ],
                ),
                question: tr_args("questions-conflict", &[("nutrient", n)]),
            });
        }
    }

    // Conditions whose key figures have no limit from the doctor yet.
    for &condition in &profile.conditions {
        for n in doctor_metrics(&BTreeSet::from([condition])) {
            if profile.doctor_limit(n, RuleBasis::Day).is_some()
                || questions
                    .iter()
                    .any(|q| q.key == format!("limit-{}", n.key()))
            {
                continue;
            }
            questions.push(Question {
                key: format!("limit-{}", n.key()),
                topic: labels::nutrient(n),
                kind: "missing",
                context: tr_args(
                    "questions-missing-context",
                    &[("condition", labels::condition(condition))],
                ),
                question: tr_args("questions-missing", &[("nutrient", labels::nutrient(n))]),
            });
        }
    }

    // Own foods with trace warnings for an allergen of the profile.
    for m in &s.my_foods {
        let Some(food) = s.food(&m.food_id.0) else {
            continue;
        };
        let e = evaluate(
            food,
            &m.portion,
            &profile,
            &s.rules,
            &Default::default(),
            s.eval_options(),
        );
        for r in e.results {
            if let Reason::AllergenTraces(list) = r.reason {
                let names: Vec<String> = list.iter().map(|&a| labels::allergen(a)).collect();
                questions.push(Question {
                    key: format!("traces-{}", food.id.0),
                    topic: food.name.clone(),
                    kind: "traces",
                    context: tr_args(
                        "questions-traces-context",
                        &[("allergens", names.join(", "))],
                    ),
                    question: tr_args("questions-traces", &[("allergens", names.join(", "))]),
                });
            }
        }
    }
    drop(s);

    let chosen = questions
        .iter()
        .filter(|q| !excluded().contains(&q.key))
        .count()
        + usize::from(!own().trim().is_empty());
    let total = questions.len() + usize::from(!own().trim().is_empty());
    let today = date::display(&date::today());

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Profiles {}, title: tr("questions-topbar") }
            div {
                h1 { class: "title", {tr("questions-title")} }
                p { class: "lead", {tr_args("questions-lead", &[("name", profile.name.clone())])} }
                p { class: "small muted print-only", {tr_args("print-date", &[("date", today)])} }
            }
            if questions.is_empty() {
                p { class: "notice", {tr("questions-none")} }
            }
            for q in questions {
                Card {
                    key: "{q.key}",
                    title: q.topic.clone(),
                    id: format!("q-{}", q.key),
                    class: if excluded().contains(&q.key) { "no-print" } else { "" },
                    div { class: "card-body",
                        span { class: "badge badge-draft", style: "align-self: flex-end;", {tr(&format!("questions-kind-{}", q.kind))} }
                        p { "{q.context}" }
                        div { class: "notice-box", style: "padding: 14px;",
                            p { class: "small", style: "font-weight: 800; color: var(--kt-primary);", {tr("questions-question")} }
                            p { style: "font-weight: 800;", "{q.question}" }
                        }
                        label { class: "check-row no-print", style: "padding: 0;",
                            input {
                                r#type: "checkbox",
                                checked: !excluded().contains(&q.key),
                                onchange: {
                                    let key = q.key.clone();
                                    move |e: FormEvent| {
                                        let mut set = excluded();
                                        if e.checked() { set.remove(&key); } else { set.insert(key.clone()); }
                                        excluded.set(set);
                                    }
                                },
                            }
                            span { class: "grow", {tr("questions-include")} }
                        }
                    }
                }
            }
            Card { title: tr("questions-own"), id: "questions-own",
                div { class: "card-body",
                    label { class: "visually-hidden", r#for: "own-question", {tr("questions-own")} }
                    textarea {
                        id: "own-question",
                        class: "input",
                        rows: "3",
                        style: "padding: 12px; min-height: 96px;",
                        value: "{own}",
                        oninput: move |e| own.set(e.value()),
                    }
                }
            }
            Notice { kind: NoticeKind::Doctor }
            button {
                class: "btn btn-primary btn-large no-print",
                onclick: move |_| { document::eval("window.print()"); },
                Icon { name: IconName::Print }
                {tr("questions-print")}
            }
            p { class: "small muted no-print", role: "status", {tr_args("questions-count", &[("chosen", chosen.to_string()), ("total", total.to_string())])} }
        }
    }
}
