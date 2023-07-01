use super::taxes::Taxes;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaxReturn {
    pub net_income: f64,
    pub taxes: Taxes,
}

impl TaxReturn {
    pub fn new(income: f64) -> TaxReturn {
        TaxReturn {
            net_income: income,
            taxes: Taxes::new(income),
        }
    }
}
