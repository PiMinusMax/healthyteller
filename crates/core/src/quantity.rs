//! Amounts with units. Conversions only between units of the same kind.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    #[serde(alias = "g")]
    Gram,
    #[serde(alias = "mg")]
    Milligram,
    #[serde(alias = "ug", alias = "µg")]
    Microgram,
    #[serde(alias = "kcal")]
    Kilocalorie,
    #[serde(alias = "kj", alias = "kJ")]
    Kilojoule,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Mass,
    Energy,
}

impl Unit {
    pub fn symbol(self) -> &'static str {
        match self {
            Unit::Gram => "g",
            Unit::Milligram => "mg",
            Unit::Microgram => "µg",
            Unit::Kilocalorie => "kcal",
            Unit::Kilojoule => "kJ",
        }
    }

    fn kind(self) -> Kind {
        match self {
            Unit::Gram | Unit::Milligram | Unit::Microgram => Kind::Mass,
            Unit::Kilocalorie | Unit::Kilojoule => Kind::Energy,
        }
    }

    /// Factor to the base unit of the kind (gram or kilocalorie).
    fn to_base(self) -> f64 {
        match self {
            Unit::Gram | Unit::Kilocalorie => 1.0,
            Unit::Milligram => 1e-3,
            Unit::Microgram => 1e-6,
            // Thermochemical calorie: 1 kcal = 4.184 kJ by definition.
            Unit::Kilojoule => 1.0 / 4.184,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub amount: f64,
    pub unit: Unit,
}

impl Quantity {
    pub const fn new(amount: f64, unit: Unit) -> Self {
        Self { amount, unit }
    }

    /// Converts to another unit of the same kind; `None` for mass to energy and back.
    pub fn to(self, unit: Unit) -> Option<Quantity> {
        (self.unit.kind() == unit.kind())
            .then(|| Quantity::new(self.amount * self.unit.to_base() / unit.to_base(), unit))
    }

    pub fn scale(self, factor: f64) -> Quantity {
        Quantity::new(self.amount * factor, self.unit)
    }

    /// Adds another quantity, converted to this one's unit.
    pub fn plus(self, other: Quantity) -> Option<Quantity> {
        other
            .to(self.unit)
            .map(|o| Quantity::new(self.amount + o.amount, self.unit))
    }

    /// `self / other` as a plain ratio, if both have the same kind.
    pub fn ratio(self, other: Quantity) -> Option<f64> {
        let other = other.to(self.unit)?;
        (other.amount != 0.0).then(|| self.amount / other.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_within_mass() {
        let q = Quantity::new(1.5, Unit::Gram).to(Unit::Milligram).unwrap();
        assert!((q.amount - 1500.0).abs() < 1e-9);
        let q = Quantity::new(250.0, Unit::Microgram)
            .to(Unit::Milligram)
            .unwrap();
        assert!((q.amount - 0.25).abs() < 1e-12);
    }

    #[test]
    fn converts_energy() {
        let q = Quantity::new(4.184, Unit::Kilojoule)
            .to(Unit::Kilocalorie)
            .unwrap();
        assert!((q.amount - 1.0).abs() < 1e-12);
    }

    #[test]
    fn refuses_mass_to_energy() {
        assert!(
            Quantity::new(1.0, Unit::Gram)
                .to(Unit::Kilocalorie)
                .is_none()
        );
        assert!(
            Quantity::new(1.0, Unit::Gram)
                .plus(Quantity::new(1.0, Unit::Kilojoule))
                .is_none()
        );
    }

    #[test]
    fn ratio_uses_common_unit() {
        let r = Quantity::new(250.0, Unit::Milligram)
            .ratio(Quantity::new(1.0, Unit::Gram))
            .unwrap();
        assert!((r - 0.25).abs() < 1e-12);
        assert!(
            Quantity::new(1.0, Unit::Gram)
                .ratio(Quantity::new(0.0, Unit::Gram))
                .is_none()
        );
    }
}
