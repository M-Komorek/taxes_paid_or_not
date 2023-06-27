use crate::{
    app::{application::App, application_mode::ApplicationMode},
    settlements::month_settlement::MonthSettlement,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs},
    Frame,
};

pub fn draw<B>(frame: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    match app.get_application_mode() {
        ApplicationMode::Recovery => {
            let block = Paragraph::new(
                "Invalid format provided!\nExpected input format: JAN:10000.\nPress Enter to try again.",
            )
            .block(
                Block::default()
                    .title(" ---> Error <--- ")
                    .borders(Borders::ALL),
            );

            let popup_vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage((100 - 40) / 2),
                        Constraint::Percentage(40),
                        Constraint::Percentage((100 - 40) / 2),
                    ]
                    .as_ref(),
                )
                .margin(1)
                .split(frame.size());

            let area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage((100 - 60) / 2),
                        Constraint::Percentage(60),
                        Constraint::Percentage((100 - 60) / 2),
                    ]
                    .as_ref(),
                )
                .margin(1)
                .split(popup_vertical_layout[1])[1];

            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(block, area);
        }
        _ => {
            let chunks = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .margin(1)
                .split(frame.size());

            let titles = app
                .tab_handler()
                .get_tabs_titles()
                .iter()
                .map(|title| Spans::from(Span::styled(title, Style::default().fg(Color::Green))))
                .collect();

            let tabs = Tabs::new(titles)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(app.get_title()),
                )
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tab_handler().get_current_tab_index());

            frame.render_widget(tabs, chunks[0]);

            match app.tab_handler().get_current_tab_index() {
                0 => draw_tax_year_block(frame, app, chunks[1], 2022),
                1 => draw_tax_year_block(frame, app, chunks[1], 2023),
                _ => {}
            }
        }
    }
}

fn draw_tax_year_block<B>(frame: &mut Frame<B>, app: &mut App, area: Rect, year: u32)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .margin(1)
        .split(area);

    draw_income_input_block(frame, app, chunks[0]);
    draw_tax_table_block(frame, app, chunks[1], year);
}

fn draw_income_input_block<B>(frame: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Length(3)].as_ref())
        .split(area);

    let (message, style) = match app.get_application_mode() {
        ApplicationMode::Edit => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the data"),
            ],
            Style::default(),
        ),
        _ => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to append data."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
    };

    let mut text = Text::from(Spans::from(message));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    frame.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input_handler().get_input_value().as_ref())
        .style(match app.get_application_mode() {
            ApplicationMode::Edit => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" --> Input <-- "),
        );

    frame.render_widget(input, chunks[1]);

    match app.get_application_mode() {
        ApplicationMode::Edit => frame.set_cursor(
            chunks[1].x + app.input_handler().get_input_value().len() as u16 + 1,
            chunks[1].y + 1,
        ),
        _ => {}
    }
}

fn draw_tax_table_block<B>(frame: &mut Frame<B>, app: &mut App, area: Rect, year: u32)
where
    B: Backend,
{
    let header_row = Row::new(["Month", "Income", "VAT", "Income Tax"].iter().map(
        |header_cell_text| {
            Cell::from(*header_cell_text).style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
        },
    ))
    .bottom_margin(1);

    let rows = app
        .settlement_handler()
        .get_year_settlements(year)
        .get_month_settlements()
        .iter()
        .map(|(month, month_settlement)| match month_settlement {
            MonthSettlement::Unsettled => Row::new([
                Cell::from(month.to_string()),
                Cell::from("---"),
                Cell::from("---"),
                Cell::from("---"),
            ])
            .bottom_margin(1),
            MonthSettlement::Settled(tax_return) => Row::new([
                Cell::from(month.to_string()),
                Cell::from(tax_return.income.to_string()),
                Cell::from(tax_return.taxes.vat.to_string()),
                Cell::from(tax_return.taxes.income_tax.to_string()),
            ])
            .bottom_margin(1),
        });

    let table = Table::new(rows)
        .header(header_row)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" --> Taxes <-- "),
        )
        .widths(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        );

    frame.render_widget(table, area)
}
