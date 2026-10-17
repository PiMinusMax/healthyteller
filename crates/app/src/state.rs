//! App state, kept on the device and saved after every change.

use std::cell::RefCell;
use std::collections::BTreeMap;

use dioxus::prelude::*;
use healthyteller_core::{
    DayTotals, Food, FoodId, Level, Options, PlateItem, Portion, Preparation, Profile, Rule,
};
use healthyteller_storage::Store;
use serde::{Deserialize, Serialize};

use crate::date;

include!(concat!(env!("OUT_DIR"), "/rules.rs"));

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reference {
    #[default]
    DgeOege,
    Efsa,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The basic notice was read and confirmed on first start.
    pub accepted_notice: bool,
    pub reference: Reference,
    pub theme: Theme,
    /// Also show results of rules that are not reviewed yet, marked as drafts.
    pub include_draft_rules: bool,
    pub language: crate::i18n::Language,
}

/// A typical food with its usual portion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyFood {
    pub food_id: FoodId,
    pub portion: Portion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedPlate {
    pub name: String,
    pub items: Vec<PlateItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayEntry {
    pub item: PlateItem,
    /// Part of a plate saved earlier today; the current plate is not saved yet.
    pub saved: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DayLog {
    pub date: String,
    pub entries: Vec<DayEntry>,
    pub saved_plates: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedFood {
    pub food: Food,
    pub fetched_at: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppState {
    pub settings: Settings,
    pub profiles: Vec<Profile>,
    pub active_profile: Option<String>,
    pub my_foods: Vec<MyFood>,
    pub plates: Vec<SavedPlate>,
    pub day: DayLog,
    pub foods: BTreeMap<String, CachedFood>,
    pub rules: Vec<Rule>,
    pub rule_errors: Vec<String>,
}

/// Content of the encrypted export file for moving to another device.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportData {
    pub version: u32,
    pub settings: Settings,
    pub profiles: Vec<Profile>,
    pub active_profile: Option<String>,
    pub my_foods: Vec<MyFood>,
    pub plates: Vec<SavedPlate>,
    /// Source data of the foods above, so they work on the new device without a network.
    pub foods: BTreeMap<String, CachedFood>,
}

thread_local! {
    static STORE: RefCell<Option<Store>> = const { RefCell::new(None) };
}

const KEYS: [&str; 7] = [
    "settings",
    "profiles",
    "active-profile",
    "my-foods",
    "plates",
    "day",
    "foods",
];

fn app_dir() -> Option<std::path::PathBuf> {
    // Android has no XDG folders; the app's private folder follows from its id (Dioxus.toml).
    #[cfg(target_os = "android")]
    {
        Some(std::path::PathBuf::from(
            "/data/data/io.github.piminusmax.healthyteller/files",
        ))
    }
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        dirs::data_dir().map(|d| d.join("healthyteller"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

fn with_store<R>(f: impl FnOnce(&Store) -> R) -> R {
    STORE.with(|cell| {
        let mut slot = cell.borrow_mut();
        let store = slot.get_or_insert_with(|| Store::platform(app_dir()));
        f(store)
    })
}

fn load_rules() -> (Vec<Rule>, Vec<String>) {
    let mut rules = Vec::new();
    let mut errors = Vec::new();
    for text in RULE_FILES {
        match healthyteller_ingest::rules::parse_rule(text) {
            Ok(file) => {
                let problems = healthyteller_ingest::rules::validate(&file);
                if problems.is_empty() {
                    rules.push(file.rule);
                } else {
                    errors.extend(
                        problems
                            .into_iter()
                            .map(|p| format!("{}: {}", p.rule_id, p.message)),
                    );
                }
            }
            Err(e) => errors.push(e),
        }
    }
    (rules, errors)
}

impl AppState {
    pub fn load() -> Self {
        let (rules, rule_errors) = load_rules();
        let mut state = with_store(|s| AppState {
            settings: s.load("settings").unwrap_or_default(),
            profiles: s.load("profiles").unwrap_or_default(),
            active_profile: s.load("active-profile").unwrap_or_default(),
            my_foods: s.load("my-foods").unwrap_or_default(),
            plates: s.load("plates").unwrap_or_default(),
            day: s.load("day").unwrap_or_default(),
            foods: s.load("foods").unwrap_or_default(),
            rules,
            rule_errors,
        });
        state.roll_day();
        state
    }

    pub fn save(&self) {
        with_store(|s| {
            let results = [
                s.save(KEYS[0], &self.settings),
                s.save(KEYS[1], &self.profiles),
                s.save(KEYS[2], &self.active_profile),
                s.save(KEYS[3], &self.my_foods),
                s.save(KEYS[4], &self.plates),
                s.save(KEYS[5], &self.day),
                s.save(KEYS[6], &self.foods),
            ];
            for r in results {
                if let Err(e) = r {
                    tracing_log(&e.to_string());
                }
            }
        });
    }

    /// Deletes everything stored on this device.
    pub fn clear_all(&mut self) {
        with_store(|s| {
            for key in KEYS {
                let _ = s.remove(key);
            }
        });
        *self = AppState::load();
    }

    /// Starts a new day log when the date changed.
    pub fn roll_day(&mut self) {
        let today = date::today();
        if self.day.date != today {
            self.day = DayLog {
                date: today,
                entries: Vec::new(),
                saved_plates: 0,
            };
        }
    }

    pub fn active_profile(&self) -> Option<&Profile> {
        let id = self.active_profile.as_ref()?;
        self.profiles.iter().find(|p| &p.id == id)
    }

    pub fn food(&self, id: &str) -> Option<&Food> {
        self.foods.get(id).map(|c| &c.food)
    }

    pub fn cache_food(&mut self, food: Food) {
        self.foods.insert(
            food.id.0.clone(),
            CachedFood {
                food,
                fetched_at: date::today(),
            },
        );
    }

    pub fn my_food(&self, id: &str) -> Option<&MyFood> {
        self.my_foods.iter().find(|m| m.food_id.0 == id)
    }

    /// Day totals; `saved_only` leaves out the current plate.
    pub fn day_totals(&self, saved_only: bool) -> DayTotals {
        let mut totals = DayTotals::new();
        for entry in &self.day.entries {
            if saved_only && !entry.saved {
                continue;
            }
            if let Some(food) = self.food(&entry.item.food_id.0) {
                totals.add(food, &entry.item.portion);
            }
        }
        totals
    }

    pub fn add_to_plate(&mut self, food_id: FoodId, portion: Portion) {
        self.roll_day();
        self.day.entries.push(DayEntry {
            item: PlateItem { food_id, portion },
            saved: false,
        });
    }

    /// Results needing attention for a food in its usual portion, one per dimension, for lists:
    /// caution, avoid and values the source lacks. Never an overall score (AGENTS.md rule 7).
    pub fn attention(&self, food_id: &str) -> Vec<(String, Level)> {
        self.food(food_id)
            .map(|f| self.attention_for(f))
            .unwrap_or_default()
    }

    /// Like [`Self::attention`], for a food that is loaded but not stored, e.g. a search hit.
    pub fn attention_for(&self, food: &Food) -> Vec<(String, Level)> {
        let Some(profile) = self.active_profile() else {
            return Vec::new();
        };
        let portion = self
            .my_food(&food.id.0)
            .map_or_else(default_portion, |m| m.portion);
        let e = healthyteller_core::evaluate(
            food,
            &portion,
            profile,
            &self.rules,
            &self.day_totals(false),
            self.eval_options(),
        );
        e.results
            .iter()
            .filter(|r| {
                matches!(r.level, Level::Avoid | Level::Caution)
                    || r.reason == healthyteller_core::Reason::ValueUnknown
            })
            .map(|r| (crate::labels::result_title(r), r.level))
            .collect()
    }

    pub fn eval_options(&self) -> Options {
        Options {
            include_drafts: self.settings.include_draft_rules,
        }
    }

    pub fn export_data(&self) -> ExportData {
        let used: Vec<&str> = self
            .my_foods
            .iter()
            .map(|m| m.food_id.0.as_str())
            .chain(
                self.plates
                    .iter()
                    .flat_map(|p| p.items.iter().map(|i| i.food_id.0.as_str())),
            )
            .collect();
        ExportData {
            version: 1,
            settings: self.settings.clone(),
            profiles: self.profiles.clone(),
            active_profile: self.active_profile.clone(),
            my_foods: self.my_foods.clone(),
            plates: self.plates.clone(),
            foods: self
                .foods
                .iter()
                .filter(|(k, _)| used.contains(&k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }

    /// Replaces profiles, own foods, plates and settings with the imported ones.
    pub fn import_data(&mut self, data: ExportData) {
        self.settings = data.settings;
        self.profiles = data.profiles;
        self.active_profile = data.active_profile;
        self.my_foods = data.my_foods;
        self.plates = data.plates;
        self.foods.extend(data.foods);
    }

    pub fn new_profile_id(&self) -> String {
        let mut n = self.profiles.len() + 1;
        while self.profiles.iter().any(|p| p.id == format!("p{n}")) {
            n += 1;
        }
        format!("p{n}")
    }
}

fn tracing_log(message: &str) {
    dioxus::logger::tracing::warn!("{message}");
}

/// Global state handle.
pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}

/// Changes the state and saves it.
pub fn update(mut state: Signal<AppState>, f: impl FnOnce(&mut AppState)) {
    let mut s = state.write();
    f(&mut s);
    s.save();
}

pub fn default_portion() -> Portion {
    Portion {
        grams: 100.0,
        preparation: Preparation::Raw,
    }
}
