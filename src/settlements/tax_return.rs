use super::taxes::Taxes;

#[derive(Debug, PartialEq)]
pub struct TaxReturn {
    pub income: f64,
    pub taxes: Taxes,
}

impl TaxReturn {
    pub fn new(income: f64) -> TaxReturn {
        TaxReturn {
            income,
            taxes: Taxes::new(income),
        }
    }
}
