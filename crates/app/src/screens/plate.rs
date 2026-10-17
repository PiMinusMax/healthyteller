//! 12/13 Teller: what was eaten today and the key figures of the person's conditions.

use dioxus::prelude::*;
use healthyteller_core::{
    Allergen, Focus, LimitKind, Nutrient, NutrientGroup, Quantity, RuleBasis, RuleStatus, Total,
};

use crate::Route;
use crate::components::{Badge, Card, Icon, IconName, TopBar};
use crate::date;
use crate::format;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::screens::profile::doctor_metrics;
use crate::state::{SavedPlate, update, use_app_state};

/// A day limit and where it comes from.
#[derive(Clone, Copy, PartialEq)]
struct Limit {
    max: Quantity,
    from_doctor: bool,
}

/// One key figure of the person's conditions, with or without a day limit.
#[derive(Clone, PartialEq)]
struct Metric {
    nutrient: Nutrient,
    total: Option<Quantity>,
    complete: bool,
    limit: Option<Limit>,
}

#[derive(Clone, PartialEq)]
struct Entry {
    index: usize,
    name: String,
    portion: String,
    allergens: Vec<Allergen>,
}

/// Day sum as text: "unbekannt" without any value, "mind. …" when some foods lack it.
fn sum_text(total: Option<Quantity>, complete: bool) -> String {
    match total {
        None => tr("value-unknown"),
        Some(q) if complete => format::quantity(q),
        Some(q) => tr_args("plate-at-least", &[("amount", format::quantity(q))]),
    }
}

/// Day sum of one nutrient. Nothing eaten counts as zero; a food without the value never does.
fn day_sum(total: Option<&Total>, nutrient: Nutrient) -> (Option<Quantity>, bool) {
    match total {
        None => (Some(Quantity::new(0.0, nutrient.unit())), true),
        Some(t) => (t.amount, t.is_complete()),
    }
}

#[component]
pub fn Plate() -> Element {
    let state = use_app_state();
    let mut plate_name = use_signal(String::new);

    let s = state.read();
    let profile = s.active_profile().cloned();
    let all = s.day_totals(false);

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
                name: food.map_or_else(|| e.item.food_id.0.clone(), |f| f.name.clone()),
                portion: labels::portion(&e.item.portion),
                allergens,
            }
        })
        .collect();

    // Key figures: doctor's limits first, then reviewed rules, then the figures the
    // conditions make relevant even without any limit.
    let mut metrics: Vec<Metric> = Vec::new();
    if let Some(p) = &profile {
        let mut add = |nutrient: Nutrient, limit: Option<Limit>| {
            if let Some(m) = metrics.iter_mut().find(|m| m.nutrient == nutrient) {
                if m.limit.is_none() {
                    m.limit = limit;
                }
                return;
            }
            let (total, complete) = day_sum(all.get(nutrient), nutrient);
            metrics.push(Metric {
                nutrient,
                total,
                complete,
                limit: limit.map(|l| Limit {
                    max: l.max.to(nutrient.unit()).unwrap_or(l.max),
                    ..l
                }),
            });
        };
        for l in p.doctor_limits.iter().filter(|l| l.basis == RuleBasis::Day) {
            if let Some(max) = l.max {
                add(
                    l.nutrient,
                    Some(Limit {
                        max,
                        from_doctor: true,
                    }),
                );
            }
        }
        for r in s.rules.iter().filter(|r| {
            p.conditions.contains(&r.condition)
                && r.basis == RuleBasis::Day
                && r.limit == LimitKind::Upper
                && (r.status == RuleStatus::Reviewed || s.settings.include_draft_rules)
        }) {
            if let Some(max) = r.avoid() {
                add(
                    r.nutrient,
                    Some(Limit {
                        max,
                        from_doctor: false,
                    }),
                );
            }
        }
        for n in doctor_metrics(&p.conditions) {
            add(n, None);
        }
    }
    let over: Vec<(Nutrient, Quantity, Quantity)> = metrics
        .iter()
        .filter_map(|m| {
            let (total, limit) = (m.total?, m.limit?);
            total
                .ratio(limit.max)
                .is_some_and(|r| r > 1.0)
                .then_some((m.nutrient, total, limit.max))
        })
        .collect();
    let found_allergens: Vec<(String, Vec<Allergen>)> = entries
        .iter()
        .filter(|e| !e.allergens.is_empty())
        .map(|e| (e.name.clone(), e.allergens.clone()))
        .collect();

    // Full list: only nutrients with at least one value; "mind." marks incomplete sums.
    let groups: Vec<(NutrientGroup, Vec<(String, String)>)> = NutrientGroup::ALL
        .into_iter()
        .map(|g| {
            let rows = Nutrient::ALL
                .iter()
                .filter(|n| n.group() == g)
                .filter_map(|&n| {
                    let t = all.get(n)?;
                    t.amount?;
                    Some((labels::nutrient(n), sum_text(t.amount, t.is_complete())))
                })
                .collect::<Vec<_>>();
            (g, rows)
        })
        .filter(|(_, rows)| !rows.is_empty())
        .collect();
    let any_incomplete = Nutrient::ALL
        .iter()
        .filter_map(|&n| all.get(n))
        .any(|t| t.amount.is_some() && !t.is_complete());
    let meals: Vec<(usize, String, String)> = s
        .plates
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let names: Vec<String> = p
                .items
                .iter()
                .filter_map(|it| s.food(&it.food_id.0).map(|f| f.name.clone()))
                .collect();
            (i, p.name.clone(), names.join(", "))
        })
        .collect();
    let can_save = s.day.entries.iter().any(|e| !e.saved);
    let day_label = date::display(&s.day.date);
    let nutrients_open = profile
        .as_ref()
        .is_some_and(|p| p.focus() == Focus::Nutrients);
    let missing_limit = metrics.iter().any(|m| m.limit.is_none());
    let profile_id = profile.as_ref().map(|p| p.id.clone());
    drop(s);

    rsx! {
        main { class: "screen",
            TopBar { back: Route::Home {}, title: tr("plate-topbar") }
            div {
                h1 { class: "title", {tr("plate-today")} }
                p { class: "muted", "{day_label}" }
            }
            // Found allergens and exceeded maximums always come first.
            for (name, list) in found_allergens {
                div { class: "alert", role: "alert",
                    Badge { level: healthyteller_core::Level::Avoid, small: true }
                    div {
                        p { class: "alert-title", {tr_args("plate-allergen-found", &[("allergens", list.iter().map(|&a| labels::allergen(a)).collect::<Vec<_>>().join(", "))])} }
                        p { class: "small", {tr_args("plate-allergen-in", &[("name", name)])} }
                    }
                }
            }
            for (nutrient, total, limit) in over {
                div { class: "alert", role: "alert",
                    Badge { level: healthyteller_core::Level::Avoid, small: true }
                    div {
                        p { class: "alert-title", {tr_args("plate-over-max", &[("nutrient", labels::nutrient(nutrient))])} }
                        p { class: "small", {tr_args("plate-over-max-detail", &[("total", format::quantity(total)), ("limit", format::quantity(limit))])} }
                    }
                }
            }
            if !metrics.is_empty() {
                Card { title: tr("plate-metrics"), id: "plate-metrics",
                    for m in metrics { MetricView { key: "{m.nutrient.key()}", metric: m } }
                    if missing_limit {
                        div { class: "card-row small muted", style: "flex-direction: column; align-items: flex-start; gap: 6px;",
                            span { {tr("plate-no-default-limit")} }
                            if let Some(id) = profile_id {
                                Link { class: "link-btn", to: Route::ProfileEdit { id, step: 3 }, {tr("plate-enter-doctor-limits")} }
                            }
                        }
                    }
                }
            }
            Card { title: tr("plate-eaten"), id: "plate-eaten",
                if entries.is_empty() {
                    p { class: "card-body muted", {tr("plate-empty")} }
                } else {
                    ul { class: "list",
                        for e in entries {
                            li { key: "{e.index}", class: "food-row",
                                div { class: "grow",
                                    p { class: "food-name", "{e.name}" }
                                    p { class: "small muted", "{e.portion}" }
                                }
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
            }
            Link { class: "btn btn-primary btn-large", to: Route::Search {}, Icon { name: IconName::Plus } {tr("plate-add-food")} }
            if !meals.is_empty() {
                Card { title: tr("plates-saved"), id: "plate-meals",
                    ul { class: "list",
                        for (i, name, foods) in meals {
                            li { key: "{i}", class: "food-row",
                                div { class: "grow",
                                    p { class: "food-name", "{name}" }
                                    p { class: "small muted", "{foods}" }
                                }
                                button {
                                    class: "btn btn-secondary",
                                    onclick: move |_| update(state, |s| {
                                        if let Some(p) = s.plates.get(i).cloned() {
                                            for item in p.items {
                                                s.add_to_plate(item.food_id, item.portion);
                                            }
                                        }
                                    }),
                                    {tr("action-take-over")}
                                    span { class: "visually-hidden", " {name}" }
                                }
                            }
                        }
                    }
                }
            }
            if !groups.is_empty() {
                details { class: "card group", id: "plate-nutrients", open: nutrients_open,
                    summary { span { {tr("plate-nutrients")} } Icon { name: IconName::ChevronDown, size: 20 } }
                    table { class: "nutrients",
                        for (g, rows) in groups {
                            tbody { key: "{g.key()}",
                                tr { th { colspan: 2, scope: "colgroup", {tr(&format!("group-{}", g.key()))} } }
                                for (name, value) in rows {
                                    tr { key: "{name}",
                                        td { "{name}" }
                                        td { class: "r num", "{value}" }
                                    }
                                }
                            }
                        }
                    }
                    if any_incomplete {
                        p { class: "card-row small muted", {tr("plate-at-least-note")} }
                    }
                }
            }
            if can_save {
                details { class: "card group", id: "plate-save",
                    summary { span { {tr("plate-save")} } Icon { name: IconName::ChevronDown, size: 20 } }
                    div { class: "card-body",
                        div { class: "field",
                            label { r#for: "plate-name", {tr("plate-save-name")} }
                            input { id: "plate-name", class: "input", value: "{plate_name}", oninput: move |e| plate_name.set(e.value()) }
                        }
                        button {
                            class: "btn btn-secondary btn-large",
                            disabled: plate_name().trim().is_empty(),
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
fn MetricView(metric: Metric) -> Element {
    let name = labels::nutrient(metric.nutrient);
    let value = sum_text(metric.total, metric.complete);
    let Some(limit) = metric.limit else {
        return rsx! {
            div { class: "bar-block",
                div { class: "bar-head",
                    span { "{name}" }
                    span { class: "num", "{value}" }
                }
                p { class: "small muted", {tr("plate-no-limit")} }
            }
        };
    };
    let max = limit.max.amount.max(f64::EPSILON);
    let total = metric.total.map_or(0.0, |q| q.amount);
    let scale = (max * 1.25).max(total);
    let pct = |v: f64| (v / scale * 100.0).clamp(0.0, 100.0);
    let total_pct = pct(total);
    let limit_pct = pct(max);
    let rest = max - total;
    let amount = |v: f64| format::quantity(Quantity::new(v, limit.max.unit));
    let status = match metric.total {
        None => tr("value-unknown"),
        Some(_) if rest >= 0.0 => tr_args("bar-left", &[("amount", amount(rest))]),
        Some(_) => tr_args("bar-over", &[("amount", amount(-rest))]),
    };
    let origin = if limit.from_doctor {
        tr("origin-doctor")
    } else {
        tr("origin-rule-short")
    };
    rsx! {
        div { class: "bar-block",
            div { class: "bar-head",
                span { "{name}" }
                span { class: "num", "{value}" }
            }
            div {
                class: "bar",
                role: "img",
                "aria-label": tr_args("bar-label", &[
                    ("nutrient", name.clone()),
                    ("total", value.clone()),
                    ("limit", format::quantity(limit.max)),
                ]),
                div { class: "bar-over", style: "left: {limit_pct}%;" }
                div { class: "bar-current", style: "left: 0; width: {total_pct}%;" }
                div { class: "bar-mark", style: "left: {limit_pct}%;", span { {tr("bar-max")} } }
            }
            p { class: if rest < 0.0 { "small error" } else { "small" },
                "{status} · "
                span { class: "muted", {tr_args("bar-origin", &[("limit", format::quantity(limit.max)), ("origin", origin)])} }
            }
        }
    }
}
