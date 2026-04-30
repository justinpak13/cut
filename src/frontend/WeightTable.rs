use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Row, Table};


use crate::app::{AppState, CurrentDisplay};
use crate::frontend::Footer::render_footer;
use crate::frontend::CustomStyle;
/// Render a table with some rows and columns.
pub fn render_table(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let header = Row::new(["Date", "Weight"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .get_data()
        .iter()
        .map(|log| Row::new([log.get_date().to_string(), log.get_weight().to_string()]))
        .collect();

    let total = app.get_data().len();

    let footer = Row::new([format!(
        "{}/{}",
        (app.table_state
            .selected()
            .unwrap_or(0)
            .checked_add(1)
            .unwrap_or(total))
        .min(total),
        total
    )]);
    let widths = [Constraint::Percentage(30), Constraint::Percentage(50)];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("➤ ")
        .block(CustomStyle::widget_block());

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

pub fn match_keys(keycode: KeyCode, app: &mut AppState) {
    match keycode {
        KeyCode::Char('j') | KeyCode::Down => app.table_state.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.table_state.select_previous(),
        KeyCode::Char('l') | KeyCode::Right => app.table_state.select_next_column(),
        KeyCode::Char('h') | KeyCode::Left => app.table_state.select_previous_column(),
        KeyCode::Char('g') => app.table_state.select_first(),
        KeyCode::Char('G') => app.table_state.select_last(),
        KeyCode::Char('d') | KeyCode::Char('D') => app.set_display(CurrentDisplay::Delete),
        _ => {}
    };
}

pub fn render_table_footer(frame: &mut Frame, area: Rect) {
    let hints: &[(&str, &str)] = {
        &[
            ("↑↓←→/kjhl", "navigate"),
            ("a", "add entry"),
            ("e", "edit entry"),
            ("d", "delete entry"),
            ("c", "chart view"),
            ("s", "save"),
            ("q", "quit"),
        ]
    };
    render_footer(frame, area, hints);
}
