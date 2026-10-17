//! What has been eaten during the day, summed per nutrient.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::food::{Food, FoodId, Portion};
use crate::nutrient::Nutrient;
use crate::quantity::Quantity;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlateItem {
    pub food_id: FoodId,
    pub portion: Portion,
}

/// Day total of one nutrient. Items without a value are counted, never treated as zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Total {
    pub amount: Option<Quantity>,
    pub items: usize,
    pub items_without_value: usize,
}

impl Total {
    pub fn is_complete(&self) -> bool {
        self.items_without_value == 0
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DayTotals {
    totals: BTreeMap<Nutrient, Total>,
}

impl DayTotals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, food: &Food, portion: &Portion) {
        for &nutrient in Nutrient::ALL {
            let total = self.totals.entry(nutrient).or_insert(Total {
                amount: None,
                items: 0,
                items_without_value: 0,
            });
            total.items += 1;
            match food.amount_in(nutrient, portion) {
                Some(q) => {
                    let q = q.to(nutrient.unit()).unwrap_or(q);
                    total.amount = match total.amount {
                        Some(sum) => sum.plus(q).or(Some(sum)),
                        None => Some(q),
                    };
                }
                None => total.items_without_value += 1,
            }
        }
    }

    pub fn get(&self, nutrient: Nutrient) -> Option<&Total> {
        self.totals.get(&nutrient)
    }

    pub fn item_count(&self) -> usize {
        self.totals.values().next().map_or(0, |t| t.items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::food::Preparation;
    use crate::food::tests::fixture_food;
    use crate::quantity::Unit;

    #[test]
    fn sums_known_values_and_counts_missing_ones() {
        let a = fixture_food(&[(Nutrient::Salt, 1.0, Unit::Gram)]);
        let b = fixture_food(&[]);
        let portion = Portion {
            grams: 200.0,
            preparation: Preparation::Raw,
        };
        let mut day = DayTotals::new();
        day.add(&a, &portion);
        day.add(&b, &portion);
        let salt = day.get(Nutrient::Salt).unwrap();
        assert_eq!(salt.amount, Some(Quantity::new(2.0, Unit::Gram)));
        assert_eq!(salt.items, 2);
        assert_eq!(salt.items_without_value, 1);
        assert!(!salt.is_complete());
        assert_eq!(day.item_count(), 2);
    }

    #[test]
    fn converts_to_display_unit() {
        let a = fixture_food(&[(Nutrient::Potassium, 0.3, Unit::Gram)]);
        let mut day = DayTotals::new();
        day.add(
            &a,
            &Portion {
                grams: 100.0,
                preparation: Preparation::Raw,
            },
        );
        let k = day.get(Nutrient::Potassium).unwrap().amount.unwrap();
        assert_eq!(k.unit, Unit::Milligram);
        assert!((k.amount - 300.0).abs() < 1e-9);
    }
}
