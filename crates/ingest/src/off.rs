//! Open Food Facts, API v2, one product per barcode, plus the name search.
//! <https://openfoodfacts.github.io/openfoodfacts-server/api/>

use std::collections::{BTreeMap, BTreeSet};

use healthyteller_core::{
    Allergen, AllergenData, Derivation, Food, FoodId, FoodKind, Nutrient, Quantity, SourceRef,
    SourcedValue, Unit, ValueBasis,
};
use serde_json::Value;

use crate::adapter::{FetchRequest, IngestError, RawArtifact, SourceAdapter, SourceMeta};
use crate::search::SearchHit;

pub const SOURCE_ID: &str = "off";

const FIELDS: &str = "code,product_name,product_name_de,generic_name_de,brands,nutriments,\
allergens_tags,traces_tags,ingredients_text,ingredients_text_de,nova_group,additives_tags,\
completeness,rev";

/// Open Food Facts asks every app to identify itself.
pub const USER_AGENT: &str = "HealthyTeller/0.1 (https://github.com/PiMinusMax/healthyteller)";

pub struct OpenFoodFacts;

/// `_100g` keys in Open Food Facts. All values except energy are stored in grams.
const NUTRIMENTS: &[(&str, Nutrient)] = &[
    ("proteins", Nutrient::Protein),
    ("fat", Nutrient::Fat),
    ("carbohydrates", Nutrient::Carbohydrate),
    ("fiber", Nutrient::Fibre),
    ("sugars", Nutrient::Sugars),
    ("starch", Nutrient::Starch),
    ("saturated-fat", Nutrient::SaturatedFat),
    ("monounsaturated-fat", Nutrient::MonounsaturatedFat),
    ("polyunsaturated-fat", Nutrient::PolyunsaturatedFat),
    ("omega-3-fat", Nutrient::Omega3),
    ("omega-6-fat", Nutrient::Omega6),
    ("cholesterol", Nutrient::Cholesterol),
    ("salt", Nutrient::Salt),
    ("sodium", Nutrient::Sodium),
    ("potassium", Nutrient::Potassium),
    ("calcium", Nutrient::Calcium),
    ("magnesium", Nutrient::Magnesium),
    ("phosphorus", Nutrient::Phosphorus),
    ("iron", Nutrient::Iron),
    ("zinc", Nutrient::Zinc),
    ("iodine", Nutrient::Iodine),
    ("selenium", Nutrient::Selenium),
    ("vitamin-a", Nutrient::VitaminA),
    ("vitamin-d", Nutrient::VitaminD),
    ("vitamin-e", Nutrient::VitaminE),
    ("vitamin-k", Nutrient::VitaminK),
    ("vitamin-c", Nutrient::VitaminC),
    ("vitamin-b1", Nutrient::VitaminB1),
    ("vitamin-b2", Nutrient::VitaminB2),
    ("vitamin-pp", Nutrient::Niacin),
    ("vitamin-b6", Nutrient::VitaminB6),
    ("vitamin-b9", Nutrient::Folate),
    ("vitamin-b12", Nutrient::VitaminB12),
];

const ALLERGEN_TAGS: &[(&str, Allergen)] = &[
    ("en:gluten", Allergen::Gluten),
    ("en:crustaceans", Allergen::Crustaceans),
    ("en:eggs", Allergen::Eggs),
    ("en:fish", Allergen::Fish),
    ("en:peanuts", Allergen::Peanuts),
    ("en:soybeans", Allergen::Soybeans),
    ("en:milk", Allergen::Milk),
    ("en:nuts", Allergen::Nuts),
    ("en:celery", Allergen::Celery),
    ("en:mustard", Allergen::Mustard),
    ("en:sesame-seeds", Allergen::Sesame),
    ("en:sulphur-dioxide-and-sulphites", Allergen::Sulphites),
    ("en:lupin", Allergen::Lupin),
    ("en:molluscs", Allergen::Molluscs),
];

/// Accepts EAN-8, UPC-A, EAN-13 and GTIN-14, digits only.
pub fn normalize_barcode(input: &str) -> Result<String, IngestError> {
    let digits: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let valid =
        matches!(digits.len(), 8 | 12 | 13 | 14) && digits.chars().all(|c| c.is_ascii_digit());
    if valid {
        Ok(digits)
    } else {
        Err(IngestError::InvalidQuery(input.to_owned()))
    }
}

/// Percent-encoding for a query parameter.
fn encode_query(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            out.push(b as char);
        } else if b == b' ' {
            out.push('+');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Fields the name search needs. The full product is loaded by barcode once picked.
const SEARCH_FIELDS: &str =
    "code,product_name,product_name_de,generic_name_de,brands,completeness,last_modified_t";

/// Search services of Open Food Facts, in the order the app tries them.
/// See sources/open-food-facts.md and docs/adr/0003-namenssuche.md.
const SEARCH_SERVICES: &[SearchService] = &[
    // Staging index of Open Food Facts. Used first while the production index
    // is stale (openfoodfacts/search-a-licious#469). Remove once that is fixed.
    SearchService::SearchALicious("https://search.openfoodfacts.net/search"),
    SearchService::SearchALicious("https://search.openfoodfacts.org/search"),
    SearchService::Legacy("https://world.openfoodfacts.org/cgi/search.pl"),
];

enum SearchService {
    /// Search-a-licious, the search Open Food Facts recommends.
    /// <https://search.openfoodfacts.org/docs>
    SearchALicious(&'static str),
    /// The older full-text search of the main site.
    Legacy(&'static str),
}

impl OpenFoodFacts {
    /// Full-text search by name or brand, one request per search service in the
    /// order to try them. Only the search terms leave the device.
    pub fn search_requests(&self, terms: &str) -> Result<Vec<FetchRequest>, IngestError> {
        let terms = terms.trim();
        if terms.chars().count() < 2 {
            return Err(IngestError::InvalidQuery(terms.to_owned()));
        }
        let q = encode_query(terms);
        Ok(SEARCH_SERVICES
            .iter()
            .map(|service| FetchRequest {
                url: match service {
                    SearchService::SearchALicious(base) => format!(
                        "{base}?q={q}&langs=de&page_size=50&fields={SEARCH_FIELDS}"
                    ),
                    SearchService::Legacy(base) => format!(
                        "{base}?search_terms={q}&search_simple=1&action=process&json=1&page_size=50&fields={SEARCH_FIELDS}"
                    ),
                },
                headers: vec![("User-Agent", USER_AGENT.to_owned())],
            })
            .collect())
    }

    /// Hits of a search answer, from either search, in the order of the service.
    /// Entries without a barcode are skipped. Rank them with
    /// [`crate::search::rank_and_merge`].
    pub fn parse_search(&self, raw: &RawArtifact) -> Result<Vec<SearchHit>, IngestError> {
        let json: Value =
            serde_json::from_slice(&raw.bytes).map_err(|e| IngestError::Format(e.to_string()))?;
        let products = json
            .get("hits")
            .or_else(|| json.get("products"))
            .and_then(Value::as_array)
            .ok_or_else(|| IngestError::Format("missing products".into()))?;
        Ok(products
            .iter()
            .filter_map(|p| {
                Some(SearchHit {
                    food: parse_product(p, &raw.retrieved_at).ok()?,
                    completeness: p.get("completeness").and_then(number),
                    last_modified: p.get("last_modified_t").and_then(Value::as_i64),
                    merged: 1,
                })
            })
            .collect())
    }
}

impl SourceAdapter for OpenFoodFacts {
    fn meta(&self) -> SourceMeta {
        SourceMeta {
            id: SOURCE_ID,
            name: "Open Food Facts",
            url: "https://world.openfoodfacts.org",
            license: "ODbL 1.0",
            citation: "Open Food Facts, https://world.openfoodfacts.org, ODbL",
        }
    }

    fn request(&self, query: &str) -> Result<FetchRequest, IngestError> {
        let code = normalize_barcode(query)?;
        Ok(FetchRequest {
            url: format!(
                "https://world.openfoodfacts.org/api/v2/product/{code}.json?fields={FIELDS}"
            ),
            headers: vec![("User-Agent", USER_AGENT.to_owned())],
        })
    }

    fn parse(&self, raw: &RawArtifact) -> Result<Vec<Food>, IngestError> {
        let json: Value =
            serde_json::from_slice(&raw.bytes).map_err(|e| IngestError::Format(e.to_string()))?;
        if json.get("status").and_then(Value::as_i64) == Some(0) {
            return Err(IngestError::NotFound);
        }
        let product = json
            .get("product")
            .ok_or_else(|| IngestError::Format("missing product".into()))?;
        Ok(vec![parse_product(product, &raw.retrieved_at)?])
    }
}

fn text(product: &Value, key: &str) -> Option<String> {
    product
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

fn number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().replace(',', ".").parse().ok(),
        _ => None,
    }
    .filter(|v: &f64| v.is_finite() && *v >= 0.0)
}

fn tags(product: &Value, key: &str) -> Vec<String> {
    product
        .get(key)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// First brand. API v2 sends a comma-separated string, Search-a-licious a list.
/// The staging index sends tag slugs with a language prefix (`xx:rewe-bio`);
/// those become readable words ("Rewe Bio").
fn brand(product: &Value) -> Option<String> {
    let first = match product.get("brands")? {
        Value::String(s) => s.split(',').next()?.trim().to_owned(),
        Value::Array(a) => {
            let first = a.first()?.as_str()?.trim();
            let first = match first.split_once(':') {
                Some((lang, rest))
                    if lang.len() == 2 && lang.chars().all(|c| c.is_ascii_lowercase()) =>
                {
                    rest.trim()
                }
                _ => first,
            };
            if is_slug(first) {
                unslug(first)
            } else {
                first.to_owned()
            }
        }
        _ => return None,
    };
    Some(first).filter(|b| !b.is_empty())
}

fn is_slug(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn unslug(slug: &str) -> String {
    slug.split('-')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().chain(c).collect::<String>())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn allergens(tags: &[String]) -> BTreeSet<Allergen> {
    tags.iter()
        .filter_map(|t| {
            ALLERGEN_TAGS
                .iter()
                .find(|(tag, _)| tag == t)
                .map(|&(_, a)| a)
        })
        .collect()
}

fn parse_product(product: &Value, retrieved_at: &str) -> Result<Food, IngestError> {
    let code = text(product, "code").ok_or_else(|| IngestError::Format("missing code".into()))?;
    let source = SourceRef {
        id: SOURCE_ID.to_owned(),
        version: product
            .get("rev")
            .and_then(Value::as_i64)
            .map_or_else(|| "api-v2".to_owned(), |rev| format!("api-v2, rev {rev}")),
        retrieved_at: retrieved_at.to_owned(),
    };
    let value = |quantity: Quantity| SourcedValue {
        quantity,
        basis: ValueBasis::Per100gAsSold,
        source: source.clone(),
        derivation: Derivation::Declared,
        samples: None,
        min: None,
        max: None,
    };

    let mut nutrients = BTreeMap::new();
    if let Some(n) = product.get("nutriments") {
        let per_100g = |key: &str| n.get(format!("{key}_100g")).and_then(number);
        if let Some(kcal) = per_100g("energy-kcal") {
            nutrients.insert(
                Nutrient::Energy,
                value(Quantity::new(kcal, Unit::Kilocalorie)),
            );
        } else if let Some(kj) = per_100g("energy") {
            let kcal = Quantity::new(kj, Unit::Kilojoule).to(Unit::Kilocalorie);
            if let Some(kcal) = kcal {
                nutrients.insert(Nutrient::Energy, value(kcal));
            }
        }
        for &(key, nutrient) in NUTRIMENTS {
            if let Some(grams) = per_100g(key) {
                let q = Quantity::new(grams, Unit::Gram);
                let q = q.to(nutrient.unit()).unwrap_or(q);
                nutrients.insert(nutrient, value(q));
            }
        }
    }

    let allergen_tags = tags(product, "allergens_tags");
    let trace_tags = tags(product, "traces_tags");
    let ingredients =
        text(product, "ingredients_text_de").or_else(|| text(product, "ingredients_text"));
    let known = ingredients.is_some() || !allergen_tags.is_empty() || !trace_tags.is_empty();

    let name = text(product, "product_name_de")
        .or_else(|| text(product, "product_name"))
        .or_else(|| text(product, "generic_name_de"))
        .unwrap_or_else(|| code.clone());

    Ok(Food {
        id: FoodId(format!("{SOURCE_ID}:{code}")),
        name,
        brand: brand(product),
        kind: FoodKind::Brand,
        nutrients,
        allergens: AllergenData {
            known,
            contains: allergens(&allergen_tags),
            traces: allergens(&trace_tags),
            completeness: product
                .get("completeness")
                .and_then(number)
                .map(|c| c.min(1.0)),
        },
        ingredients,
        nova_group: product
            .get("nova_group")
            .and_then(number)
            .map(|n| n as u8)
            .filter(|n| (1..=4).contains(n)),
        additives: tags(product, "additives_tags")
            .iter()
            .map(|t| t.trim_start_matches("en:").to_uppercase())
            .collect(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> RawArtifact {
        let path = format!(
            "{}/../../fixtures/open-food-facts/{name}",
            env!("CARGO_MANIFEST_DIR")
        );
        RawArtifact {
            bytes: std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}")),
            retrieved_at: "2026-01-01".into(),
        }
    }

    #[test]
    fn parses_product_fixture() {
        let foods = OpenFoodFacts.parse(&fixture("product-found.json")).unwrap();
        let food = &foods[0];
        assert_eq!(food.id, FoodId("off:4000000000017".into()));
        assert_eq!(food.name, "Testjoghurt natur");
        assert_eq!(food.brand.as_deref(), Some("Fixture Molkerei"));
        assert_eq!(food.nova_group, Some(1));
        assert_eq!(food.additives, vec!["E440".to_owned()]);

        let energy = &food.nutrients[&Nutrient::Energy];
        assert_eq!(energy.quantity, Quantity::new(60.0, Unit::Kilocalorie));
        assert_eq!(energy.derivation, Derivation::Declared);
        // Grams in the source, milligrams for display.
        let potassium = food.nutrients[&Nutrient::Potassium].quantity;
        assert_eq!(potassium.unit, Unit::Milligram);
        assert!((potassium.amount - 150.0).abs() < 1e-9);
        // String values with a decimal comma are accepted.
        assert!((food.nutrients[&Nutrient::Salt].quantity.amount - 0.12).abs() < 1e-12);
        // Missing values stay missing.
        assert!(!food.nutrients.contains_key(&Nutrient::Purine));
        assert!(!food.nutrients.contains_key(&Nutrient::VitaminD));

        assert!(food.allergens.known);
        assert!(food.allergens.contains.contains(&Allergen::Milk));
        assert!(food.allergens.traces.contains(&Allergen::Nuts));
        assert_eq!(food.allergens.completeness, Some(0.8));
    }

    #[test]
    fn energy_falls_back_to_kilojoule() {
        let foods = OpenFoodFacts
            .parse(&fixture("product-energy-kj.json"))
            .unwrap();
        let kcal = foods[0].nutrients[&Nutrient::Energy].quantity;
        assert!((kcal.amount - 100.0).abs() < 1e-9);
        assert!(!foods[0].allergens.known);
    }

    #[test]
    fn product_not_found() {
        assert_eq!(
            OpenFoodFacts.parse(&fixture("product-not-found.json")),
            Err(IngestError::NotFound)
        );
    }

    #[test]
    fn parses_search_results_and_skips_entries_without_code() {
        let foods = OpenFoodFacts
            .parse_search(&fixture("search-results.json"))
            .unwrap();
        let names: Vec<&str> = foods.iter().map(|h| h.food.name.as_str()).collect();
        assert_eq!(names, vec!["Testquark mager", "Testquark Vanille"]);
        assert!(foods[1].food.allergens.contains.contains(&Allergen::Milk));
        assert_eq!(foods[0].completeness, Some(0.7));
    }

    #[test]
    fn parses_search_a_licious_hits() {
        let foods = OpenFoodFacts
            .parse_search(&fixture("search-a-licious-results.json"))
            .unwrap();
        let names: Vec<&str> = foods.iter().map(|h| h.food.name.as_str()).collect();
        assert_eq!(names, vec!["Testquark mager", "Testquark Vanille"]);
        assert_eq!(foods[1].food.id, FoodId("off:4000000000055".into()));
        // Brands come as a list here.
        assert_eq!(foods[0].food.brand.as_deref(), Some("Fixture Molkerei"));
        assert_eq!(foods[1].last_modified, Some(1_780_000_000));
        // Hits carry no nutrients; the app loads the product by barcode.
        assert!(foods[0].food.nutrients.is_empty());
    }

    #[test]
    fn search_requests_in_order_with_encoded_terms() {
        let urls: Vec<String> = OpenFoodFacts
            .search_requests(" Quark & Müsli ")
            .unwrap()
            .into_iter()
            .map(|r| r.url)
            .collect();
        assert_eq!(urls.len(), 3);
        assert!(
            urls[0].starts_with("https://search.openfoodfacts.net/search?q=Quark+%26+M%C3%BCsli&"),
            "{}",
            urls[0]
        );
        assert!(
            urls[1].starts_with("https://search.openfoodfacts.org/search?q=Quark+%26+M%C3%BCsli&")
        );
        assert!(urls[2].starts_with("https://world.openfoodfacts.org/cgi/search.pl?"));
        assert!(urls[2].contains("search_terms=Quark+%26+M%C3%BCsli&"));
        assert!(OpenFoodFacts.search_requests("q").is_err());
    }

    #[test]
    fn brand_drops_language_prefix() {
        let product = serde_json::json!({ "brands": ["xx:Fixture Molkerei"] });
        assert_eq!(brand(&product).as_deref(), Some("Fixture Molkerei"));
        let product = serde_json::json!({ "brands": ["xx:fixture-molkerei-2"] });
        assert_eq!(brand(&product).as_deref(), Some("Fixture Molkerei 2"));
        let product = serde_json::json!({ "brands": "Marke: Spezial, Zweite" });
        assert_eq!(brand(&product).as_deref(), Some("Marke: Spezial"));
    }

    #[test]
    fn request_needs_a_valid_barcode() {
        let req = OpenFoodFacts.request(" 4000000000017 ").unwrap();
        assert!(
            req.url
                .starts_with("https://world.openfoodfacts.org/api/v2/product/4000000000017.json")
        );
        assert!(OpenFoodFacts.request("12ab").is_err());
        assert!(OpenFoodFacts.request("123").is_err());
    }
}
