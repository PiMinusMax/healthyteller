//! Ranking and merging of name-search hits.
//!
//! Search services return many near-identical entries ("Apfel", "Äpfel", "Apfel")
//! and rank loose matches (category only) above clear ones. This module sorts hits
//! by how well the name matches the search terms and merges entries with the same
//! name and brand, keeping the one with the most information and the newest edit.

use healthyteller_core::Food;

/// A search hit with what is needed to rank it.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchHit {
    pub food: Food,
    /// Share of filled fields at the source, 0 to 1 and above.
    pub completeness: Option<f64>,
    /// Last edit at the source, Unix seconds.
    pub last_modified: Option<i64>,
    /// How many entries with the same name and brand were merged into this one.
    pub merged: usize,
}

/// How well a hit matches the search terms. Lower is better.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Match {
    /// The name is exactly the search terms.
    Exact,
    /// The name starts with the search terms.
    Prefix,
    /// Every search word is a word of the name.
    AllWordsInName,
    /// Every search word occurs in the name, possibly inside a longer word.
    AllInName,
    /// Every search word occurs in name or brand.
    AllInNameOrBrand,
    /// Some search words occur in name or brand.
    Some,
    /// The service matched on something else, for example the category.
    Other,
}

/// Lower case without diacritics, so "Äpfel" and "Apfel" compare equal.
pub fn fold(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'ä' | 'á' | 'à' | 'â' | 'å' => out.push('a'),
            'ö' | 'ó' | 'ò' | 'ô' => out.push('o'),
            'ü' | 'ú' | 'ù' | 'û' => out.push('u'),
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'í' | 'ì' | 'î' | 'ï' => out.push('i'),
            'ç' => out.push('c'),
            'ñ' => out.push('n'),
            'ß' => out.push_str("ss"),
            c => out.push(c),
        }
    }
    out
}

fn words(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect()
}

fn classify(terms: &str, food: &Food) -> Match {
    let name = fold(&food.name);
    let brand = food.brand.as_deref().map(fold).unwrap_or_default();
    let name_words = words(&name);
    let term_words = words(terms);
    if term_words.is_empty() {
        return Match::Other;
    }
    if name_words == term_words {
        return Match::Exact;
    }
    if name_words.starts_with(&term_words) {
        return Match::Prefix;
    }
    if term_words.iter().all(|t| name_words.contains(t)) {
        return Match::AllWordsInName;
    }
    if term_words.iter().all(|t| name.contains(t)) {
        return Match::AllInName;
    }
    if term_words
        .iter()
        .all(|t| name.contains(t) || brand.contains(t))
    {
        return Match::AllInNameOrBrand;
    }
    if term_words
        .iter()
        .any(|t| name.contains(t) || brand.contains(t))
    {
        return Match::Some;
    }
    Match::Other
}

/// Higher is better: more information first, then the newer edit.
fn quality(hit: &SearchHit) -> (i64, i64) {
    (
        (hit.completeness.unwrap_or(0.0) * 1000.0) as i64,
        hit.last_modified.unwrap_or(0),
    )
}

/// Merges hits with the same name and brand and sorts them by match, then quality.
/// The order of the service decides between otherwise equal hits.
pub fn rank_and_merge(terms: &str, hits: Vec<SearchHit>) -> Vec<SearchHit> {
    let terms = fold(terms);
    let mut merged: Vec<(Match, usize, SearchHit)> = Vec::new();
    for (position, hit) in hits.into_iter().enumerate() {
        let key = |f: &Food| {
            (
                words(&fold(&f.name)).join(" "),
                f.brand.as_deref().map(fold).unwrap_or_default(),
            )
        };
        let hit_key = key(&hit.food);
        if let Some((_, _, kept)) = merged.iter_mut().find(|(_, _, k)| key(&k.food) == hit_key) {
            let count = kept.merged + hit.merged;
            if quality(&hit) > quality(kept) {
                *kept = hit;
            }
            kept.merged = count;
        } else {
            merged.push((classify(&terms, &hit.food), position, hit));
        }
    }
    merged.sort_by(|(ma, pa, a), (mb, pb, b)| {
        ma.cmp(mb)
            .then_with(|| quality(b).cmp(&quality(a)))
            .then_with(|| pa.cmp(pb))
    });
    merged.into_iter().map(|(_, _, hit)| hit).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use healthyteller_core::{AllergenData, FoodId, FoodKind, SourceRef};

    fn hit(
        code: &str,
        name: &str,
        brand: Option<&str>,
        completeness: f64,
        modified: i64,
    ) -> SearchHit {
        SearchHit {
            food: Food {
                id: FoodId(format!("off:{code}")),
                name: name.into(),
                brand: brand.map(Into::into),
                kind: FoodKind::Brand,
                nutrients: Default::default(),
                allergens: AllergenData::default(),
                ingredients: None,
                nova_group: None,
                additives: Vec::new(),
                source: SourceRef {
                    id: "off".into(),
                    version: "test".into(),
                    retrieved_at: "2026-01-01".into(),
                },
            },
            completeness: Some(completeness),
            last_modified: Some(modified),
            merged: 1,
        }
    }

    fn names(hits: &[SearchHit]) -> Vec<String> {
        hits.iter()
            .map(|h| match &h.food.brand {
                Some(b) => format!("{} · {b}", h.food.name),
                None => h.food.name.clone(),
            })
            .collect()
    }

    #[test]
    fn folds_umlauts_and_case() {
        assert_eq!(fold("Äpfel Größe"), "apfel grosse");
    }

    #[test]
    fn clear_matches_come_before_loose_ones() {
        let ranked = rank_and_merge(
            "Apfel",
            vec![
                hit("1", "Bio-Äpfel Holsteiner Cox", Some("Testmarke"), 0.9, 10),
                hit("2", "Nicoter", None, 0.4, 10),
                hit("3", "Apfelmus", None, 0.9, 10),
                hit("4", "Apfel fruchtig", None, 0.5, 10),
                hit("5", "Apfel", None, 0.7, 10),
            ],
        );
        assert_eq!(
            names(&ranked),
            vec![
                "Apfel",
                "Apfel fruchtig",
                "Bio-Äpfel Holsteiner Cox · Testmarke",
                "Apfelmus",
                "Nicoter"
            ]
        );
    }

    #[test]
    fn same_name_and_brand_are_merged_into_the_best_entry() {
        let ranked = rank_and_merge(
            "Apfel",
            vec![
                hit("1", "Apfel", None, 0.4, 300),
                hit("2", "Äpfel", None, 0.8, 100),
                hit("3", "apfel", None, 0.8, 200),
                hit("4", "Apfel", Some("Testmarke"), 0.5, 10),
            ],
        );
        assert_eq!(ranked.len(), 2);
        // Most complete wins, the newer edit breaks the tie.
        assert_eq!(ranked[0].food.id, FoodId("off:3".into()));
        assert_eq!(ranked[0].merged, 3);
        assert_eq!(ranked[1].food.brand.as_deref(), Some("Testmarke"));
        assert_eq!(ranked[1].merged, 1);
    }

    #[test]
    fn several_words_may_match_name_and_brand() {
        let ranked = rank_and_merge(
            "testmarke joghurt",
            vec![
                hit("1", "Erdbeere", Some("Testmarke"), 0.9, 10),
                hit("2", "Joghurt mild", Some("Testmarke"), 0.5, 10),
            ],
        );
        assert_eq!(ranked[0].food.name, "Joghurt mild");
    }
}
