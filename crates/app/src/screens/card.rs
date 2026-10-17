//! 07/08 Lebensmittelkarte: verdict and reason first, values and sources on request.

use dioxus::prelude::*;
use healthyteller_core::{
    Dimension, DimensionResult, Level, LimitOrigin, Portion, Preparation, Reason, evaluate,
};

use crate::Route;
use crate::components::{
    Badge, Card, Icon, IconName, Notice, NoticeKind, QualityExplanation, TopBar,
};
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{MyFood, default_portion, update, use_app_state};

/// Portion from the route; `prep == "mine"` means the saved portion of Meine Lebensmittel.
pub fn resolve_portion(
    state: &crate::state::AppState,
    id: &str,
    grams: u32,
    prep: &str,
) -> Portion {
    if prep == "mine" {
        return state
            .my_food(id)
            .map_or_else(default_portion, |m| m.portion);
    }
    Portion {
        grams: f64::from(grams.max(1)),
        preparation: Preparation::from_key(prep).unwrap_or(Preparation::Raw),
    }
}

#[component]
pub fn FoodCard(id: String, grams: u32, prep: String) -> Element {
    let state = use_app_state();
    let mut added = use_signal(|| false);
    let s = state.read();
    let Some(food) = s.food(&id).cloned() else {
        drop(s);
        return rsx! {
            main { class: "screen",
                TopBar { back: Route::Search {}, title: tr("card-topbar") }
                h1 { class: "title", {tr("food-missing-title")} }
                Link { class: "btn btn-primary btn-large", to: Route::Search {}, {tr("search-title")} }
            }
        };
    };
    let portion = resolve_portion(&s, &id, grams, &prep);
    let profile = s.active_profile().cloned();
    let in_my_foods = s.my_food(&id).is_some();
    let evaluation = profile.as_ref().map(|p| {
        let day = s.day_totals(false);
        evaluate(&food, &portion, p, &s.rules, &day, s.eval_options())
    });
    drop(s);

    let grams_u = portion.grams.round() as u32;
    let prep_key = portion.preparation.key().to_owned();
    let food_id = food.id.clone();

    rsx! {
        main { class: "screen",
            TopBar { back: Route::PortionScreen { id: id.clone() }, title: tr("card-topbar") }
            div { style: "display: flex; align-items: flex-end; justify-content: space-between; gap: 12px;",
                div { style: "display: flex; flex-direction: column; gap: 4px;",
                    h1 { class: "title", "{food.name}" }
                    p { class: "subtitle", {labels::portion(&portion)} }
                    QualityExplanation { id: id.clone() }
                    if let Some(p) = &profile {
                        p { class: "muted", {tr_args("card-checked-for", &[("name", p.name.clone())])} }
                    }
                }
                Link { class: "btn btn-quiet", to: Route::PortionScreen { id: id.clone() }, {tr("action-change")} }
            }

            if profile.is_none() {
                div { class: "notice",
                    p { {tr("card-no-profile")} }
                    Link { class: "link-btn", to: Route::ProfileEdit { id: "neu".into(), step: 1 }, {tr("action-create-profile")} }
                }
            }

            if let Some(e) = &evaluation {
                if !e.conflicts.is_empty() {
                    Card { title: tr("card-conflicts"), class: "warn",
                        ul { class: "list",
                            for c in e.conflicts.iter() {
                                li { class: "card-row",
                                    {tr_args("card-conflict", &[
                                        ("nutrient", labels::nutrient(c.nutrient)),
                                        ("upper", labels::condition(c.upper.0)),
                                        ("lower", labels::condition(c.lower.0)),
                                    ])}
                                }
                            }
                        }
                    }
                }
                if e.attention().next().is_some() {
                    Card { title: tr("card-attention"), id: "card-attention",
                        for r in e.attention() { ResultItem { result: r.clone(), open: true } }
                    }
                }
                if e.unknown().next().is_some() {
                    Card { title: tr("card-unknown"), id: "card-unknown",
                        for r in e.unknown() { ResultItem { result: r.clone(), open: false } }
                    }
                }
                if e.unremarkable().next().is_some() {
                    Card { title: tr("card-unremarkable"), id: "card-unremarkable",
                        for r in e.unremarkable() { ResultItem { result: r.clone(), open: false } }
                    }
                }
                if e.results.is_empty() {
                    p { class: "notice", {tr("card-nothing-to-check")} }
                }
            }

            if profile.as_ref().is_some_and(|p| !p.allergies.is_empty()) {
                Notice { kind: NoticeKind::Allergens }
            }

            if food.nova_group.is_some() || !food.additives.is_empty() || food.ingredients.is_some() {
                Card { title: tr("card-more"), id: "card-more",
                    dl { class: "card-body facts",
                        if let Some(nova) = food.nova_group {
                            dt { Link { to: Route::Term { term: "nova".into() }, {tr("term-nova-title")} } }
                            dd { {tr_args("card-nova", &[("group", nova.to_string())])} }
                        }
                        if !food.additives.is_empty() {
                            dt { {tr("card-additives")} }
                            dd { {food.additives.join(", ")} }
                        }
                        if let Some(ingredients) = &food.ingredients {
                            dt { {tr("card-ingredients")} }
                            dd { "{ingredients}" }
                        }
                    }
                    p { class: "card-row small muted", {tr("card-more-note")} }
                }
            }

            Link {
                class: "btn btn-secondary btn-large",
                to: Route::NutrientList { id: id.clone(), grams: grams_u, prep: prep_key.clone() },
                {tr("card-all-nutrients")}
            }
            Link {
                class: "btn btn-quiet btn-large",
                to: Route::CompareStart { id: id.clone(), grams: grams_u },
                {tr("card-compare")}
            }
            div { class: "btn-row",
                button {
                    class: "btn btn-primary btn-large",
                    onclick: {
                        let food_id = food_id.clone();
                        move |_| {
                            update(state, |s| s.add_to_plate(food_id.clone(), portion));
                            navigator().push(Route::Plate {});
                        }
                    },
                    {tr("card-to-plate")}
                }
            }
            if in_my_foods || added() {
                p { class: "status-line", role: "status", Icon { name: IconName::Plus, size: 18 } {tr("card-in-my-foods")} }
            } else {
                button {
                    class: "btn btn-quiet btn-large",
                    onclick: move |_| {
                        let food_id = food.id.clone();
                        update(state, |s| {
                            if s.my_food(&food_id.0).is_none() {
                                s.my_foods.push(MyFood { food_id: food_id.clone(), portion });
                            }
                        });
                        added.set(true);
                    },
                    {tr("card-add-my-foods")}
                }
            }
        }
    }
}

fn reason_text(r: &DimensionResult) -> String {
    let value = r
        .value_per_portion
        .map(format::quantity)
        .unwrap_or_default();
    let limit = r
        .budget
        .as_ref()
        .map(|b| format::quantity(b.limit))
        .unwrap_or_default();
    let completeness = r
        .completeness
        .map_or_else(|| tr("completeness-unknown"), format::percent);
    let args: Vec<(&str, String)> = match &r.reason {
        Reason::AllergenFound(list) | Reason::AllergenTraces(list) => {
            let names: Vec<String> = list.iter().map(|&a| labels::allergen(a)).collect();
            vec![
                ("allergens", names.join(", ")),
                ("completeness", completeness),
            ]
        }
        Reason::ValueUnknown => vec![(
            "nutrient",
            r.nutrient.map(labels::nutrient).unwrap_or_default(),
        )],
        _ => vec![
            ("value", value),
            ("limit", limit),
            ("completeness", completeness),
        ],
    };
    tr_args(&format!("reason-{}", r.reason.key()), &args)
}

#[component]
fn ResultItem(result: DimensionResult, open: bool) -> Element {
    let r = result;
    let title = labels::result_title(&r);
    let why = labels::result_why(&r);
    let reason = reason_text(&r);
    let small = r.level == Level::Ok;
    let sources: Vec<String> = r.sources.iter().map(|s| labels::source_name(s)).collect();
    let term = match r.dimension {
        Dimension::Purine => Some("purine"),
        Dimension::BloodSugar => Some("glycemic-load"),
        Dimension::Histamine => Some("histamine"),
        Dimension::Oxalate => Some("oxalate"),
        Dimension::Allergens => Some("traces"),
        _ => None,
    };
    rsx! {
        details { class: "result", open: open,
            summary {
                span { class: "result-head",
                    span { style: "display: flex; flex-direction: column;",
                        span { class: "result-name", "{title}" }
                        span { class: "result-why", "{why}" }
                    }
                    Badge { level: r.level, small }
                }
                span { "{reason}" }
                if r.draft {
                    span { class: "badge badge-draft", {tr("card-draft")} }
                }
                span { class: "result-more", {tr("card-values-and-source")} Icon { name: IconName::ChevronDown, size: 18 } }
            }
            div { class: "result-details",
                dl { class: "facts",
                    if let Some(v) = r.value_per_portion {
                        dt { {tr("fact-per-portion")} }
                        dd { class: "num", style: "font-weight: 700;", {format::quantity(v)} }
                    }
                    if let Some(b) = &r.budget {
                        dt { {tr("fact-limit")} }
                        dd {
                            {format::quantity(b.limit)}
                            " " {labels::basis(b.basis)} ", "
                            {match &b.origin {
                                LimitOrigin::Doctor => tr("origin-doctor"),
                                LimitOrigin::Rule { id } => tr_args("origin-rule", &[("id", id.clone())]),
                            }}
                        }
                        if b.basis == healthyteller_core::RuleBasis::Day {
                            dt { {tr("fact-today")} }
                            dd { class: "num", {format::quantity(b.used)} }
                        }
                    }
                    if !sources.is_empty() {
                        dt { {tr("fact-source")} }
                        dd { {sources.join(", ")} }
                    }
                    if let Some(e) = r.evidence {
                        dt { {tr("fact-evidence")} }
                        dd { {labels::evidence(e)} }
                    }
                    if let Some(c) = r.completeness {
                        dt { {tr("fact-completeness")} }
                        dd { {format::percent(c)} }
                    }
                }
                if let Some(t) = term {
                    Link { class: "link-btn", to: Route::Term { term: t.to_owned() }, {tr(&format!("term-{t}-title"))} " – " {tr("term-explain")} }
                }
            }
        }
    }
}
