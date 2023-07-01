use super::tax_return::TaxReturn;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MonthSettlement {
    Unsettled,
    Settled(TaxReturn),
}

impl MonthSettlement {
    pub fn new_settled_month(income: f64) -> MonthSettlement {
        MonthSettlement::Settled(TaxReturn::new(income))
    }
}
