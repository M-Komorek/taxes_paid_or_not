use super::{annual_settlement::AnnualSettlement, Month, MonthSettlement};

use std::collections::HashMap;

pub struct SettlementHandler {
    year_settlements: HashMap<u32, AnnualSettlement>,
}

impl SettlementHandler {
    pub fn new(years: &Vec<String>) -> SettlementHandler {
        let mut year_settlements = HashMap::new();

        for year in years {
            let year = year.parse::<u32>().expect("It should be a valid year!");
            year_settlements.insert(year, AnnualSettlement::new());
        }

        SettlementHandler { year_settlements }
    }

    pub fn get_settlements(&self) -> &HashMap<u32, AnnualSettlement> {
        &self.year_settlements
    }

    pub fn get_year_settlements(&self, year: u32) -> &AnnualSettlement {
        &self.year_settlements.get(&year).unwrap()
    }

    pub fn update_settlements(&mut self, year_settlements: HashMap<u32, AnnualSettlement>) {
        if !year_settlements.is_empty() {
            self.year_settlements = year_settlements;
        }
    }

    pub fn update_settlement(
        &mut self,
        year: u32,
        month: Month,
        month_settlement: MonthSettlement,
    ) {
        self.year_settlements
            .get_mut(&year)
            .unwrap()
            .update_month_settlement(month, month_settlement);
    }
}
