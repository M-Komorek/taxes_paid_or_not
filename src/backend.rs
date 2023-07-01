use crate::{
    app::{App, ApplicationMode},
    data_parser::{self, deserialize_from_json_file},
    ui,
};

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn create_and_run_app() -> Result<(), io::Error> {
    let mut terminal = setup_crossterm_terminal()?;

    let mut app = App::new(" --> Taxes Paid or Not <-- ".to_string());
    app.settlement_handler_mut()
        .update_settlements(deserialize_from_json_file()?);

    let app_result = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{:?}", err)
    }

    Ok(())
}

fn setup_crossterm_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn run_app<B>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>
where
    B: Backend,
{
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.get_application_mode() {
                ApplicationMode::Standard => match key.code {
                    KeyCode::Left => app.tab_handler_mut().previous_tab(),
                    KeyCode::Right => app.tab_handler_mut().next_tab(),
                    KeyCode::Char('a') => app.set_application_mode(ApplicationMode::Edit),
                    KeyCode::Char('q') => {
                        return data_parser::serialize_to_json_file(
                            app.settlement_handler().get_settlements(),
                        )
                    }
                    _ => {}
                },
                ApplicationMode::Edit => match key.code {
                    KeyCode::Char(c) => app.input_handler_mut().push_char_to_input_value(c),
                    KeyCode::Backspace => app.input_handler_mut().pop_char_from_input_value(),
                    KeyCode::Enter => app.add_settlement(),
                    KeyCode::Esc => app.set_application_mode(ApplicationMode::Standard),
                    _ => {}
                },
                ApplicationMode::Recovery => match key.code {
                    KeyCode::Enter => app.set_application_mode(ApplicationMode::Standard),
                    _ => {}
                },
            }
        }
    }
}
