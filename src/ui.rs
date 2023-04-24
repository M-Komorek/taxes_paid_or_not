use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub fn draw<B>(frame: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let titles = app
        .get_tabs_titles()
        .iter()
        .map(|title| Spans::from(Span::styled(*title, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.get_title()),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.get_current_tab_index());

    frame.render_widget(tabs, chunks[0]);
}
