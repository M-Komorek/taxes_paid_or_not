use super::{input_handler::InputHandler, tab_handler::TabHandler, ApplicationMode};
use crate::settlements::SettlementHandler;

pub struct App {
    title: String,
    application_mode: ApplicationMode,
    input_handler: InputHandler,
    tab_handler: TabHandler,
    settlements_handler: SettlementHandler,
}

impl App {
    pub fn new(title: String) -> App {
        let tab_handler = TabHandler::new(vec!["2022".to_string(), "2023".to_string()]);
        let settlements_handler = SettlementHandler::new(tab_handler.get_tabs_titles());

        App {
            title,
            application_mode: ApplicationMode::Standard,
            input_handler: InputHandler::default(),
            tab_handler,
            settlements_handler,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_application_mode(&self) -> &ApplicationMode {
        &self.application_mode
    }

    pub fn set_application_mode(&mut self, application_mode: ApplicationMode) {
        self.application_mode = application_mode;
    }

    pub fn input_handler(&self) -> &InputHandler {
        &self.input_handler
    }

    pub fn input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }

    pub fn tab_handler(&self) -> &TabHandler {
        &self.tab_handler
    }

    pub fn tab_handler_mut(&mut self) -> &mut TabHandler {
        &mut self.tab_handler
    }

    pub fn settlement_handler(&self) -> &SettlementHandler {
        &self.settlements_handler
    }

    pub fn add_settlement(&mut self) {
        match self.input_handler.parse_input_value() {
            Ok((month, month_settlement)) => {
                let year = self
                    .tab_handler
                    .get_current_tab_title()
                    .parse::<u32>()
                    .expect("The year from tab_handler should be converted to u32");
                self.settlements_handler
                    .update_settlement(year, month, month_settlement);

                self.input_handler.clear_input_value();
                self.application_mode = ApplicationMode::Standard;
            }
            Err(_) => {
                self.input_handler.clear_input_value();
                self.application_mode = ApplicationMode::Recovery;
            }
        };
    }
}
