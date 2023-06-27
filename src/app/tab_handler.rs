pub struct TabHandler {
    current_tab_index: usize,
    titles: Vec<String>,
}

impl TabHandler {
    pub fn new(titles: Vec<String>) -> TabHandler {
        TabHandler {
            current_tab_index: 0,
            titles,
        }
    }

    pub fn get_current_tab_index(&self) -> usize {
        self.current_tab_index
    }

    pub fn get_current_tab_title(&self) -> &str {
        &self.titles[self.current_tab_index]
    }

    pub fn get_tabs_titles(&self) -> &Vec<String> {
        &self.titles
    }

    pub fn next_tab(&mut self) {
        self.current_tab_index = (self.current_tab_index + 1) % self.titles.len();
    }

    pub fn previous_tab(&mut self) {
        if self.current_tab_index > 0 {
            self.current_tab_index -= 1;
        } else {
            self.current_tab_index = self.titles.len() - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_tab_should_go_to_next_tab() {
        let mut tab_handler = TabHandler::new(vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ]);

        assert_eq!(tab_handler.get_current_tab_title(), "first");

        tab_handler.next_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "second");

        tab_handler.next_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "third");

        tab_handler.next_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "first");
    }

    #[test]
    fn previous_tab_should_go_to_previous_tab() {
        let mut tab_handler = TabHandler::new(vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ]);

        assert_eq!(tab_handler.get_current_tab_title(), "first");

        tab_handler.previous_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "third");

        tab_handler.previous_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "second");

        tab_handler.previous_tab();
        assert_eq!(tab_handler.get_current_tab_title(), "first");
    }
}
