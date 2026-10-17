//! 22 Druckliste: positive and avoid list from the profile and Meine Lebensmittel.

use dioxus::prelude::*;
use healthyteller_core::{Level, evaluate};

use crate::Route;
use crate::components::{
    Badge, Card, DataStatusNotice, Icon, IconName, Notice, NoticeKind, TopBar,
};
use crate::date;
use crate::i18n::{tr, tr_args};
use crate::labels;
use crate::state::use_app_state;

#[derive(Clone, PartialEq)]
struct Line {
    name: String,
    portion: String,
    reasons: Vec<String>,
}

#[component]
pub fn PrintList() -> Element {
    let state = use_app_state();
    let s = state.read();
    let profile = s.active_profile().cloned();
    let mut groups: [(Level, Vec<Line>); 4] = [
        (Level::Ok, Vec::new()),
        (Level::Caution, Vec::new()),
        (Level::Avoid, Vec::new()),
        (Level::Unknown, Vec::new()),
    ];
    let mut sources: Vec<(String, String)> = Vec::new();
    if let Some(p) = &profile {
        for m in &s.my_foods {
            let Some(food) = s.food(&m.food_id.0) else {
                continue;
            };
            let e = evaluate(
                food,
                &m.portion,
                p,
                &s.rules,
                &Default::default(),
                s.eval_options(),
            );
            // The list groups by the most serious result; each line still names its reasons.
            let level = [Level::Avoid, Level::Caution, Level::Unknown]
                .into_iter()
                .find(|l| e.results.iter().any(|r| r.level == *l))
                .unwrap_or(Level::Ok);
            let reasons = e
                .results
                .iter()
                .filter(|r| r.level == level && level != Level::Ok)
                .map(|r| {
                    format!(
                        "{} ({})",
                        labels::result_title(r),
                        tr(&format!("level-{}", r.level.key()))
                    )
                })
                .collect();
            if let Some(g) = groups.iter_mut().find(|(l, _)| *l == level) {
                g.1.push(Line {
                    name: food.name.clone(),
                    portion: labels::portion(&m.portion),
                    reasons,
                });
            }
            let fetched = s
                .foods
                .get(&m.food_id.0)
                .map(|c| c.fetched_at.clone())
                .unwrap_or_default();
            let src = labels::source_name(&food.source.id);
            if !sources.iter().any(|(n, d)| n == &src && d == &fetched) {
                sources.push((src, fetched));
            }
        }
    }
    let today = date::display(&date::today());
    drop(s);

    rsx! {
        main { class: "screen",
            TopBar { back: Route::MyFoods {}, title: tr("print-create") }
            if let Some(p) = profile {
                div {
                    h1 { class: "title", {tr_args("print-title", &[("name", p.name.clone())])} }
                    p { class: "muted", {labels::profile_summary(&p)} " · " {tr_args("print-date", &[("date", today)])} }
                }
                Notice { kind: NoticeKind::Basic }
                if !p.conditions.is_empty() {
                    Notice { kind: NoticeKind::Doctor }
                }
                if !p.allergies.is_empty() {
                    Notice { kind: NoticeKind::Allergens }
                }
                for (level, lines) in groups {
                    if !lines.is_empty() {
                        Card { key: "{level.key()}", title: tr(&format!("print-group-{}", level.key())), id: format!("print-{}", level.key()),
                            ul { class: "list",
                                for line in lines {
                                    li { key: "{line.name}", class: "food-row",
                                        div { class: "grow",
                                            p { class: "food-name", "{line.name}" }
                                            p { class: "small muted", "{line.portion}" }
                                            if !line.reasons.is_empty() { p { class: "small", {line.reasons.join(", ")} } }
                                        }
                                        Badge { level, small: true }
                                    }
                                }
                            }
                        }
                    }
                }
                if sources.is_empty() {
                    p { class: "notice", {tr("print-empty")} }
                }
                for (src, d) in sources {
                    DataStatusNotice { key: "{src}{d}", source: src.clone(), date: date::display(&d) }
                }
                button {
                    class: "btn btn-primary btn-large no-print",
                    onclick: move |_| { document::eval("window.print()"); },
                    Icon { name: IconName::Print }
                    {tr("print-action")}
                }
            } else {
                h1 { class: "title", {tr("print-create")} }
                p { class: "notice", {tr("card-no-profile")} }
                Link { class: "btn btn-primary btn-large", to: Route::ProfileEdit { id: "neu".into(), step: 1 }, {tr("action-create-profile")} }
            }
        }
    }
}
