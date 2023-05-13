use super::tax_return::TaxReturn;

#[derive(Debug, PartialEq)]
pub enum MonthSettlement {
    Unsettled,
    Settled(TaxReturn),
}

impl MonthSettlement {
    pub fn new_settled_month(income: f64) -> MonthSettlement {
        MonthSettlement::Settled(TaxReturn::new(income))
    }
}
