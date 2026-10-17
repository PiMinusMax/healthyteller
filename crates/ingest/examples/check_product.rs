//! Checks a live answer of the Open Food Facts product API against the adapter.
//! Used by the weekly source check: `check_product <file.json>`.

use healthyteller_core::Nutrient;
use healthyteller_ingest::off::OpenFoodFacts;
use healthyteller_ingest::{RawArtifact, SourceAdapter};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: check_product <file.json>");
    let bytes = std::fs::read(&path).expect("read file");
    let raw = RawArtifact {
        bytes,
        retrieved_at: "live".into(),
    };
    let foods = match OpenFoodFacts.parse(&raw) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Open Food Facts answer could not be parsed: {e}");
            std::process::exit(1);
        }
    };
    let food = &foods[0];
    let mut problems = Vec::new();
    if food.name.trim().is_empty() || food.name == food.id.0 {
        problems.push("product name missing");
    }
    for n in [Nutrient::Energy, Nutrient::Sugars, Nutrient::Salt] {
        if !food.nutrients.contains_key(&n) {
            problems.push(n.key());
        }
    }
    if !food.allergens.known {
        problems.push("allergen information missing");
    }
    if problems.is_empty() {
        println!(
            "Open Food Facts product format unchanged: {} ({} values)",
            food.name,
            food.nutrients.len()
        );
    } else {
        eprintln!(
            "Open Food Facts product format changed, missing: {}",
            problems.join(", ")
        );
        std::process::exit(1);
    }
}
