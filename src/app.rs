struct TabHandler<'a> {
    current_tab_index: usize,
    titles: Vec<&'a str>,
}

impl<'a> TabHandler<'a> {
    fn new(titles: Vec<&'a str>) -> TabHandler {
        TabHandler {
            current_tab_index: 0,
            titles,
        }
    }

    pub fn get_current_tab_index(&self) -> usize {
        self.current_tab_index
    }

    pub fn get_tabs_titles(&self) -> &Vec<&'a str> {
        &self.titles
    }

    fn next(&mut self) {
        self.current_tab_index = (self.current_tab_index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.current_tab_index > 0 {
            self.current_tab_index -= 1;
        } else {
            self.current_tab_index = self.titles.len() - 1;
        }
    }
}

pub struct App<'a> {
    title: &'a str,
    tab_handler: TabHandler<'a>,
    should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            tab_handler: TabHandler::new(vec!["2022", "2023"]),
            should_quit: false,
        }
    }

    pub fn get_title(&self) -> &'a str {
        &self.title
    }

    pub fn get_current_tab_index(&self) -> usize {
        self.tab_handler.get_current_tab_index()
    }

    pub fn get_tabs_titles(&self) -> &Vec<&'a str> {
        &self.tab_handler.get_tabs_titles()
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn on_left(&mut self) {
        self.tab_handler.previous();
    }

    pub fn on_right(&mut self) {
        self.tab_handler.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
