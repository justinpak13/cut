use std::cmp::Ordering;

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

use crate::app::{AppState, CurrentDisplay, TableDisplay};
use crate::frontend::custom_style;
use crate::frontend::footer::render_footer;

const WEIGHT_LOSS_STYLE: Style = Style::new().light_green();
const WEIGHT_GAIN_STYLE: Style = Style::new().light_red();
const LOWEST_STYLE: Style = Style::new().blue().bold().slow_blink();

/// Render a table with some rows and columns.
pub fn render_total_table(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let header = Row::new(["Date", "Weight", "+/-", "Notes"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let data = app.get_data();
    let total = data.len();

    if total == 0 {
        frame.render_widget(
            Paragraph::new("There is no data. Please add a weight log")
                .block(custom_style::widget_block()),
            area,
        );
        return;
    }

    let mut rows: Vec<Row> = Vec::with_capacity(total);

    rows.push({
        let log = data
            .first()
            .expect("if there are not entries, should be caught above");

        let weight = log.get_weight();

        Row::new([
            Cell::new(log.get_date().to_string()),
            Cell::new(format!("{:.2}", weight)).style(if weight == app.min_weight {
                LOWEST_STYLE
            } else {
                Style::new()
            }),
            Cell::new(String::new()),
            Cell::new(log.get_note().unwrap_or_default()),
        ])
    });

    for window in data.windows(2) {
        let current_log = &window[1];
        let prev_log = &window[0];

        let difference = current_log.get_weight() - prev_log.get_weight();
        let difference_style = match difference.partial_cmp(&0.0) {
            Some(Ordering::Greater) => WEIGHT_GAIN_STYLE,
            Some(Ordering::Less) => WEIGHT_LOSS_STYLE,
            _ => Style::new(),
        };

        let weight = current_log.get_weight();

        let row = Row::new([
            Cell::new(current_log.get_date().to_string()),
            Cell::new(format!("{:.2}", weight)).style(if weight == app.min_weight {
                LOWEST_STYLE
            } else {
                Style::new()
            }),
            Cell::new(format!("{difference:+.2}")).style(difference_style),
            Cell::new(current_log.get_note().unwrap_or_else(String::new)),
        ]);
        rows.push(row);
    }

    let stats = format!(
        "Min: {:.2} | Max: {:.2} | Start: {} | End: {}",
        app.min_weight, app.max_weight, app.min_date, app.max_date
    );

    let position = Row::new([format!(
        "{}/{}",
        (app.table_state
            .selected()
            .unwrap_or(0)
            .checked_add(1)
            .unwrap_or(total))
        .min(total),
        total,
    )]);

    let widths = [
        Constraint::Length(11),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Fill(1),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(position.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("➤ ")
        .block(
            custom_style::widget_block()
                .title_bottom(stats)
                .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        );

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

pub fn render_weekly_table(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let header = Row::new(["Week of", "Average Weight", "+/-"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut data = app.get_average_weekly_data();
    let total = data.len();

    if total == 0 {
        frame.render_widget(
            Paragraph::new("There is no data. Please add a weight log")
                .block(custom_style::widget_block()),
            area,
        );
        return;
    }

    let min_average_weight: f32 = *data.values().min_by(|a, b| a.total_cmp(&b)).unwrap_or(&0.0);
    let max_average_weight: f32 = *data.values().max_by(|a, b| a.total_cmp(&b)).unwrap_or(&0.0);

    let mut rows: Vec<Row> = Vec::with_capacity(total);
    rows.push({
        let entry = data.first_entry().expect("should have a first value");

        Row::new([
            Cell::new(entry.key().to_string()),
            Cell::new(format!("{:.2}", entry.get())).style(if entry.get() == &app.min_weight {
                LOWEST_STYLE
            } else {
                Style::new()
            }),
            Cell::new(String::new()),
        ])
    });

    let mut data_iter = data.into_iter().peekable();

    while let (Some(prev_value), Some(current_value)) = (data_iter.next(), data_iter.peek()) {
        let difference = current_value.1 - prev_value.1;
        let difference_style = match difference.partial_cmp(&0.0) {
            Some(Ordering::Greater) => WEIGHT_GAIN_STYLE,
            Some(Ordering::Less) => WEIGHT_LOSS_STYLE,
            _ => Style::new(),
        };

        let weight = current_value.1;

        let row = Row::new([
            Cell::new(current_value.0.to_string()),
            Cell::new(format!("{:.2}", weight)).style(if weight == app.min_weight {
                LOWEST_STYLE
            } else {
                Style::new()
            }),
            Cell::new(format!("{difference:+.2}")).style(difference_style),
        ]);
        rows.push(row);
    }

    let stats = format!(
        "Min: {:.2} | Max: {:.2} | Start: {} | End: {}",
        min_average_weight, max_average_weight, app.min_date, app.max_date
    );

    let position = Row::new([format!(
        "{}/{}",
        (app.table_state
            .selected()
            .unwrap_or(0)
            .checked_add(1)
            .unwrap_or(total))
        .min(total),
        total,
    )]);

    let widths = [
        Constraint::Length(11),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(position.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("➤ ")
        .block(
            custom_style::widget_block()
                .title_bottom(stats)
                .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        );

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

pub fn match_keys(keycode: KeyCode, app: &mut AppState) -> bool {
    match keycode {
        KeyCode::Char('j') | KeyCode::Down => app.table_state.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.table_state.select_previous(),
        KeyCode::Char('l') | KeyCode::Right => app.table_state.select_next_column(),
        KeyCode::Char('h') | KeyCode::Left => app.table_state.select_previous_column(),
        KeyCode::Char('g') => app.table_state.select_first(),
        KeyCode::Char('G') => app.table_state.select_last(),
        KeyCode::Char('t') if app.get_display() == &CurrentDisplay::Table(TableDisplay::Week) => {
            app.set_display(CurrentDisplay::Table(TableDisplay::Total));
        }
        KeyCode::Char('s') if app.get_display() == &CurrentDisplay::Table(TableDisplay::Total) => {
            app.table_state.select_first();
            app.set_display(CurrentDisplay::Table(TableDisplay::Week));
        }
        KeyCode::Char('d' | 'D')
            if app.get_display() == &CurrentDisplay::Table(TableDisplay::Total) =>
        {
            app.set_display(CurrentDisplay::Delete);
        }
        _ => {
            return false;
        }
    }

    true
}

pub fn render_total_table_footer(frame: &mut Frame, area: Rect) {
    let hints: &[(&str, &str)] = {
        &[
            ("↑↓←→/kjhl", "navigate"),
            ("a", "add entry"),
            ("d", "delete entry"),
            ("s", "weekly view"),
            ("c", "chart view"),
            ("q", "quit"),
        ]
    };

    render_footer(frame, area, hints);
}

pub fn render_weekly_table_footer(frame: &mut Frame, area: Rect) {
    let hints: &[(&str, &str)] = {
        &[
            ("↑↓←→/kjhl", "navigate"),
            ("a", "add entry"),
            ("c", "chart view"),
            ("t", "total view"),
            ("q", "quit"),
        ]
    };
    render_footer(frame, area, hints);
}
