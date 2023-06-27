use crate::settlements::{Month, MonthSettlement};
use std::str::FromStr;

#[derive(Default)]
pub struct InputHandler {
    input_value: String,
}

impl InputHandler {
    pub fn get_input_value(&self) -> &String {
        &self.input_value
    }

    pub fn push_char_to_input_value(&mut self, c: char) {
        self.input_value.push(c);
    }

    pub fn pop_char_from_input_value(&mut self) {
        self.input_value.pop();
    }

    pub fn clear_input_value(&mut self) {
        self.input_value.clear();
    }

    pub fn parse_input_value(&mut self) -> Result<(Month, MonthSettlement), String> {
        let parts: Vec<&str> = self.input_value.trim().split(':').collect();

        match parts.as_slice() {
            [month, income] => {
                let income = match income.trim().parse::<f64>() {
                    Ok(income) => income,
                    Err(_) => return Err("Invalid income value!".to_string()),
                };
                let month = match Month::from_str(month.trim()) {
                    Ok(month) => month,
                    Err(e) => return Err(e),
                };

                let month_settlement = MonthSettlement::new_settled_month(income);
                Ok((month, month_settlement))
            }
            _ => Err("Invalid input format!".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_value_should_fail_on_invalid_format() {
        let mut input_handler = InputHandler {
            input_value: "invalid_data_format".to_string(),
        };

        let result = input_handler.parse_input_value();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid input format!".to_string());
    }

    #[test]
    fn parse_input_value_should_fail_on_invalid_income() {
        let mut input_handler = InputHandler {
            input_value: "JAN:invalid".to_string(),
        };

        let result = input_handler.parse_input_value();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid income value!".to_string());
    }

    #[test]
    fn parse_input_value_should_fail_on_invalid_month() {
        let mut input_handler = InputHandler {
            input_value: "invalid:10000".to_string(),
        };

        let result = input_handler.parse_input_value();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "'invalid' is not a valid month!")
    }

    #[test]
    fn parse_input_value_should_succeed_on_valid_input() {
        let mut input_handler = InputHandler {
            input_value: "JAN:10000".to_string(),
        };

        let result = input_handler.parse_input_value();

        assert!(result.is_ok());

        let (month, month_settlement) = result.unwrap();
        assert_eq!(month, Month::JAN);
        assert_eq!(
            month_settlement,
            MonthSettlement::new_settled_month(10000.0)
        );
    }
}
