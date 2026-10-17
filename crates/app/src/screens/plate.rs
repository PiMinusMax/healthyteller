//! 12/13 Teller: day totals against limits, saved plates pale, current plate strong.

use dioxus::prelude::*;
use healthyteller_core::{
    Allergen, Focus, LimitKind, Nutrient, NutrientGroup, Quantity, RuleBasis, RuleStatus,
};

use crate::Route;
use crate::components::{Badge, Card, Icon, IconName, QualityMark, TopBar};
use crate::date;
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::{SavedPlate, update, use_app_state};

/// Name, amount and whether all foods had a value.
type NutrientRows = Vec<(String, String, bool)>;

#[derive(Clone, PartialEq)]
struct LimitBar {
    nutrient: Nutrient,
    limit: Quantity,
    from_doctor: bool,
    saved: Quantity,
    total: Quantity,
    complete: bool,
}

#[derive(Clone, PartialEq)]
struct Entry {
    index: usize,
    id: String,
    name: String,
    portion: String,
    saved: bool,
    allergens: Vec<Allergen>,
}

#[component]
pub fn Plate() -> Element {
    let state = use_app_state();
    let mut plate_name = use_signal(String::new);

    let s = state.read();
    let profile = s.active_profile().cloned();
    let mut focus = use_signal(|| profile.as_ref().map_or(Focus::Nutrients, |p| p.focus()));
    let all = s.day_totals(false);
    let saved = s.day_totals(true);

    let entries: Vec<Entry> = s
        .day
        .entries
        .iter()
        .enumerate()
        .map(|(index, e)| {
            let food = s.food(&e.item.food_id.0);
            let allergens = match (&profile, food) {
                (Some(p), Some(f)) => p
                    .allergies
                    .intersection(&f.allergens.contains)
                    .copied()
                    .collect(),
                _ => Vec::new(),
            };
            Entry {
                index,
                id: e.item.food_id.0.clone(),
                name: food.map_or_else(|| e.item.food_id.0.clone(), |f| f.name.clone()),
                portion: labels::portion(&e.item.portion),
                saved: e.saved,
                allergens,
            }
        })
        .collect();

    // Limits per day: doctor's limits first, then rules of the profile's conditions.
    let mut bars: Vec<LimitBar> = Vec::new();
    if let Some(p) = &profile {
        let mut add = |nutrient: Nutrient, limit: Quantity, from_doctor: bool| {
            if bars.iter().any(|b| b.nutrient == nutrient) {
                return;
            }
            let unit = nutrient.unit();
            let zero = Quantity::new(0.0, unit);
            let total = all.get(nutrient);
            bars.push(LimitBar {
                nutrient,
                limit: limit.to(unit).unwrap_or(limit),
                from_doctor,
                saved: saved.get(nutrient).and_then(|t| t.amount).unwrap_or(zero),
                total: total.and_then(|t| t.amount).unwrap_or(zero),
                complete: total.is_none_or(|t| t.is_complete()),
            });
        };
        for l in p.doctor_limits.iter().filter(|l| l.basis == RuleBasis::Day) {
            if let Some(max) = l.max {
                add(l.nutrient, max, true);
            }
        }
        for r in s.rules.iter().filter(|r| {
            p.conditions.contains(&r.condition)
                && r.basis == RuleBasis::Day
                && r.limit == LimitKind::Upper
                && (r.status == RuleStatus::Reviewed || s.settings.include_draft_rules)
        }) {
            if let Some(max) = r.avoid() {
                add(r.nutrient, max, false);
            }
        }
    }
    let over: Vec<LimitBar> = bars
        .iter()
        .filter(|b| b.total.ratio(b.limit).is_some_and(|r| r > 1.0))
        .cloned()
        .collect();
    let found_allergens: Vec<(String, Vec<Allergen>)> = entries
        .iter()
        .filter(|e| !e.allergens.is_empty())
        .map(|e| (e.name.clone(), e.allergens.clone()))
        .collect();

    let groups: Vec<(NutrientGroup, NutrientRows)> = NutrientGroup::ALL
        .into_iter()
        .map(|g| {
            let rows = Nutrient::ALL
                .iter()
                .filter(|n| n.group() == g)
                .filter_map(|&n| {
                    let t = all.get(n)?;
                    let amount = t.amount?;
                    Some((
                        labels::nutrient(n),
                        format::quantity(amount),
                        t.is_complete(),
                    ))
                })
                .collect();
            (g, rows)
        })
        .collect();
    let day_label = date::display(&s.day.date);
    let saved_plates = s.day.saved_plates;
    let saved_count = entries.iter().filter(|e| e.saved).count();
    let has_conditions = profile.as_ref().is_some_and(|p| !p.conditions.is_empty());
    let profile_id = profile.as_ref().map(|p| p.id.clone());
    drop(s);

    let current: Vec<Entry> = entries.iter().filter(|e| !e.saved).cloned().collect();
    let conditions_first = focus() == Focus::Conditions;

    let limits_section = rsx! {
        Card { title: tr("plate-limits"), id: "plate-limits",
            if bars.is_empty() {
                div { class: "card-body",
                    p { {if has_conditions { tr("plate-no-limits") } else { tr("plate-no-conditions") }} }
                    if let Some(id) = profile_id.clone() {
                        if has_conditions {
                            Link { class: "link-btn", to: Route::ProfileEdit { id, step: 3 }, {tr("plate-enter-doctor-limits")} }
                        }
                    }
                }
            } else {
                div { class: "legend", style: "padding-top: 14px;",
                    span { i { style: "background: var(--kt-primary); opacity: .35;" } {tr("legend-saved")} }
                    span { i { style: "background: var(--kt-primary);" } {tr("legend-current")} }
                    span { i { style: "background: var(--kt-gut-soft); border: 1px solid var(--kt-line);" } {tr("legend-target")} }
                    span { i { style: "background: var(--kt-meiden-soft); border: 1px solid var(--kt-line);" } {tr("legend-over")} }
                }
                for b in bars.clone() { LimitBarView { key: "{b.nutrient.key()}", bar: b } }
            }
        }
    };
    let nutrients_section = rsx! {
        Card { title: tr("plate-nutrients"), id: "plate-nutrients",
            p { class: "card-body small muted", {tr("plate-nutrients-note")} }
            if entries.is_empty() {
                p { class: "card-row muted", {tr("plate-empty")} }
            }
            for (g, rows) in groups.into_iter().filter(|(_, r)| !r.is_empty()) {
                details { key: "{g.key()}", class: "group", style: "border-top: 1px solid var(--kt-line-soft);",
                    summary { span { style: "font-weight: 800;", {tr(&format!("group-{}", g.key()))} } Icon { name: IconName::ChevronDown, size: 20 } }
                    table { class: "nutrients",
                        tbody {
                            for (name, value, complete) in rows {
                                tr { key: "{name}",
                                    td { "{name}" }
                                    td { class: "r num",
                                        "{value}"
                                        if !complete { span { class: "small muted", " " {tr("plate-incomplete")} } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("plate-topbar") }
            div {
                h1 { class: "title", {tr("plate-today")} }
                p { class: "muted", "{day_label}" }
            }
            if profile.is_some() {
                div { class: "seg", role: "group", "aria-label": tr("plate-focus"),
                    button { "aria-pressed": if conditions_first { "true" } else { "false" }, onclick: move |_| focus.set(Focus::Conditions), {tr("focus-conditions")} }
                    button { "aria-pressed": if conditions_first { "false" } else { "true" }, onclick: move |_| focus.set(Focus::Nutrients), {tr("focus-nutrients")} }
                }
            }
            // Found allergens and exceeded maximums always come first, whatever the focus.
            for (name, list) in found_allergens {
                div { class: "alert", role: "alert",
                    Badge { level: healthyteller_core::Level::Avoid, small: true }
                    div {
                        p { class: "alert-title", {tr_args("plate-allergen-found", &[("allergens", list.iter().map(|&a| labels::allergen(a)).collect::<Vec<_>>().join(", "))])} }
                        p { class: "small", {tr_args("plate-allergen-in", &[("name", name)])} }
                    }
                }
            }
            for b in over {
                div { class: "alert", role: "alert",
                    Badge { level: healthyteller_core::Level::Avoid, small: true }
                    div {
                        p { class: "alert-title", {tr_args("plate-over-max", &[("nutrient", labels::nutrient(b.nutrient))])} }
                        p { class: "small", {tr_args("plate-over-max-detail", &[("total", format::quantity(b.total)), ("limit", format::quantity(b.limit))])} }
                    }
                }
            }
            if conditions_first {
                {limits_section}
                {nutrients_section}
            } else {
                {nutrients_section}
                {limits_section}
            }
            Card { title: tr("plate-current"), id: "plate-current", class: "focus-strong",
                if current.is_empty() {
                    p { class: "card-body muted", {tr("plate-current-empty")} }
                } else {
                    ul { class: "list",
                        for e in current {
                            li { key: "{e.index}", class: "food-row",
                                div { class: "grow",
                                    p { class: "food-name", "{e.name}" }
                                    p { class: "small muted", "{e.portion}" }
                                }
                                QualityMark { id: e.id.clone(), compact: true }
                                if !e.allergens.is_empty() { Badge { level: healthyteller_core::Level::Avoid, small: true } }
                                button {
                                    class: "icon-btn",
                                    onclick: move |_| update(state, |s| { if e.index < s.day.entries.len() { s.day.entries.remove(e.index); } }),
                                    Icon { name: IconName::Trash, size: 20 }
                                    span { class: "visually-hidden", {tr_args("action-remove-named", &[("name", e.name.clone())])} }
                                }
                            }
                        }
                    }
                }
                if saved_plates > 0 {
                    p { class: "card-row small muted", {tr_args("plate-saved-today", &[("plates", saved_plates.to_string()), ("foods", saved_count.to_string())])} }
                }
            }
            Link { class: "btn btn-primary btn-large", to: Route::Search {}, Icon { name: IconName::Plus } {tr("plate-add-food")} }
            if entries.iter().any(|e| !e.saved) {
                Card { title: tr("plate-save"),
                    div { class: "card-body",
                        div { class: "field",
                            label { r#for: "plate-name", {tr("plate-save-name")} }
                            input { id: "plate-name", class: "input", value: "{plate_name}", oninput: move |e| plate_name.set(e.value()) }
                            p { class: "small muted", {tr("plate-save-name-hint")} }
                        }
                        button {
                            class: "btn btn-secondary btn-large",
                            onclick: move |_| {
                                let name = plate_name().trim().to_owned();
                                update(state, |s| {
                                    let items: Vec<_> = s.day.entries.iter().filter(|e| !e.saved).map(|e| e.item.clone()).collect();
                                    if !name.is_empty() && !items.is_empty() {
                                        s.plates.push(SavedPlate { name: name.clone(), items });
                                    }
                                    for e in s.day.entries.iter_mut() { e.saved = true; }
                                    s.day.saved_plates += 1;
                                });
                                plate_name.set(String::new());
                            },
                            {tr("plate-save-action")}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LimitBarView(bar: LimitBar) -> Element {
    let limit = bar.limit.amount.max(f64::EPSILON);
    let total = bar.total.amount;
    let scale = (limit * 1.25).max(total);
    let pct = |v: f64| (v / scale * 100.0).clamp(0.0, 100.0);
    let saved_pct = pct(bar.saved.amount);
    let current_pct = (pct(total) - saved_pct).max(0.0);
    let limit_pct = pct(limit);
    let rest = limit - total;
    let status = if rest >= 0.0 {
        tr_args(
            "bar-left",
            &[(
                "amount",
                format::quantity(Quantity::new(rest, bar.limit.unit)),
            )],
        )
    } else {
        tr_args(
            "bar-over",
            &[(
                "amount",
                format::quantity(Quantity::new(-rest, bar.limit.unit)),
            )],
        )
    };
    let origin = if bar.from_doctor {
        tr("origin-doctor")
    } else {
        tr("origin-rule-short")
    };
    let name = labels::nutrient(bar.nutrient);
    rsx! {
        div { class: "bar-block",
            div { class: "bar-head",
                span { "{name}" }
                span { class: "num", {format::quantity(bar.total)} }
            }
            div {
                class: "bar",
                role: "img",
                "aria-label": tr_args("bar-label", &[
                    ("nutrient", name.clone()),
                    ("total", format::quantity(bar.total)),
                    ("limit", format::quantity(bar.limit)),
                ]),
                div { class: "bar-over", style: "left: {limit_pct}%;" }
                div { class: "bar-saved", style: "width: {saved_pct}%;" }
                div { class: "bar-current", style: "left: {saved_pct}%; width: {current_pct}%;" }
                div { class: "bar-mark", style: "left: {limit_pct}%;", span { {tr("bar-max")} } }
            }
            p { class: if rest < 0.0 { "small error" } else { "small" }, "{status}" }
            p { class: "meta muted", {tr_args("bar-origin", &[("limit", format::quantity(bar.limit)), ("origin", origin)])} }
            if !bar.complete {
                p { class: "meta muted", {tr("plate-incomplete-long")} }
            }
        }
    }
}
