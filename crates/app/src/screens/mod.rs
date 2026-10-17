//! One module per screen, named after the mockups in docs/mockups.

mod about;
mod card;
mod compare;
mod estimates;
mod home;
mod my_foods;
mod not_found;
mod nutrients;
mod plate;
mod portion;
mod print;
mod profile;
mod questions;
mod search;
mod settings;
mod term;
mod transfer;
mod welcome;

pub use about::About;
pub use card::FoodCard;
pub use compare::{Compare, CompareStart};
pub use estimates::Estimates;
pub use home::Home;
pub use my_foods::MyFoods;
pub use not_found::NotFound;
pub use nutrients::NutrientList;
pub use plate::Plate;
pub use portion::PortionScreen;
pub use print::PrintList;
pub use profile::{ProfileDraft, ProfileEdit, Profiles};
pub use questions::DoctorQuestions;
pub use search::Search;
pub use settings::{DataSources, DeviceTransfer, Settings, SettingsAdvanced};
#[cfg(test)]
pub use term::TERMS;
pub use term::Term;
pub use welcome::Welcome;
