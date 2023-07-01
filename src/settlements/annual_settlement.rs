use super::{Month, MonthSettlement};

use std::collections::BTreeMap;

use enum_iterator::all;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AnnualSettlement {
    billing_months: BTreeMap<Month, MonthSettlement>,
}

impl AnnualSettlement {
    pub fn new() -> AnnualSettlement {
        let mut billing_months = BTreeMap::new();

        for month in all::<Month>().collect::<Vec<_>>() {
            billing_months.insert(month, MonthSettlement::Unsettled);
        }

        AnnualSettlement { billing_months }
    }

    pub fn get_month_settlements(&self) -> &BTreeMap<Month, MonthSettlement> {
        &self.billing_months
    }

    pub fn update_month_settlement(&mut self, month: Month, billing_months: MonthSettlement) {
        self.billing_months.insert(month, billing_months);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_settlement_should_update_month_settlement() {
        let mut annual_settlement = AnnualSettlement::new();

        annual_settlement
            .update_month_settlement(Month::MAY, MonthSettlement::new_settled_month(10000.0));

        assert_eq!(
            annual_settlement
                .get_month_settlements()
                .get(&Month::MAY)
                .expect("Should contain MonthSettlement!"),
            &MonthSettlement::new_settled_month(10000.0)
        );
    }
}
