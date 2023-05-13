use std::ops::AddAssign;

const VAT_RATE: f64 = 0.23;
const INCOME_TAX_RATE: f64 = 0.12;

#[derive(Debug, PartialEq)]
pub struct Taxes {
    pub vat: f64,
    pub income_tax: f64,
    pub zus: f64,
}

impl Taxes {
    pub fn new(income: f64) -> Taxes {
        let vat = (income * VAT_RATE).round();
        let income_tax = (income * INCOME_TAX_RATE).round();
        let zus = 626.93;

        Taxes {
            vat,
            income_tax,
            zus,
        }
    }
}

impl AddAssign<&Taxes> for Taxes {
    fn add_assign(&mut self, other: &Taxes) {
        self.vat += other.vat;
        self.income_tax += other.income_tax;
        self.zus += other.zus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_calculate_correct_taxes() {
        let taxes = Taxes::new(1000.0);

        assert_eq!(taxes.vat, 230.0);
        assert_eq!(taxes.income_tax, 120.0);
        assert_eq!(taxes.zus, 626.93)
    }

    #[test]
    fn test_add_assig() {
        let mut taxes = Taxes::new(1000.0);
        let other_taxes = Taxes::new(2000.0);

        taxes += &other_taxes;

        assert_eq!(taxes.vat, 690.0);
        assert_eq!(taxes.income_tax, 360.0);
        assert_eq!(taxes.zus, 1253.86);
    }
}
