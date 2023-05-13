use enum_iterator::all;
use std::collections::BTreeMap;

use super::{Month, MonthSettlement};

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
