use chrono::{Datelike, Days, Local, NaiveDate};
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use crossterm::style;
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::{Marker, block};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{
    Axis, Block, Chart, Clear, Dataset, GraphType, Padding, Paragraph, Row, Table, TableState,
};
use ratatui::{Frame, layout};
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use time::OffsetDateTime;

use crate::app::{AddState, AppState, CurrentDisplay, GraphDisplay};

use crate::frontend::CustomStyle;
use crate::frontend::Footer::render_footer;
/// Render a chart going upward.
pub fn render_chart(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let (btree, title): (BTreeMap<NaiveDate, f32>, &str) = match &app_state.get_display() {
        CurrentDisplay::Graph(GraphDisplay::AverageWeekly) => (
            app_state
                .get_average_weekly_data()
                .iter()
                .map(|(date, weight)| (*date, *weight))
                .collect(),
            "Average Weekly Weight",
        ),
        CurrentDisplay::Graph(GraphDisplay::Week) => {
            let today = NaiveDate::from_yo_opt(
                app_state.current_date.year(),
                app_state.current_date.ordinal() as u32,
            )
            .expect("should not have issues creating date");
            let a_week_ago = today
                .checked_sub_days(Days::new(7))
                .expect("should not have issues with subtracting a week");
            (
                app_state
                    .get_average_daily_data()
                    .range((Included(&a_week_ago), Included(&today)))
                    .map(|(date, weight)| (*date, *weight))
                    .collect(),
                "Weight Data per Week",
            )
        }
        _ => (
            app_state
                .get_average_daily_data()
                .iter()
                .map(|(date, weight)| (*date, *weight))
                .collect(),
            "Average Daily Weight",
        ),
    };

    // if less than 1 date, do not render chart
    if btree.len() <= 1 {
        let text = Paragraph::new("Not enough data for a chart").block(CustomStyle::widget_block());
        frame.render_widget(text, area);
        return;
    }

    let min_date = &btree.keys().min().expect("should not have issue with date");
    let max_date = &btree.keys().max().expect("should not have issue with date");

    let min_weight = app_state.min_weight - 1.0;
    let max_weight = app_state.max_weight + 1.0;

    let data: Vec<(f64, f64)> = btree
        .iter()
        .map(|(k, v)| (k.to_epoch_days() as f64, *v as f64))
        .collect();

    let mid_label = btree
        .keys()
        .min_by_key(|d| {
            (d.to_epoch_days() as f64
                - ((max_date.to_epoch_days() as f64 + min_date.to_epoch_days() as f64) / 2.0))
                .abs() as i64
        })
        .map(|d| d.to_string())
        .unwrap_or_default();

    let x_axis = Axis::default()
        .bounds([
            min_date.to_epoch_days() as f64,
            max_date.to_epoch_days() as f64,
        ])
        .labels([min_date.to_string(), mid_label, max_date.to_string()]);

    let y_axis = Axis::default()
        .bounds([min_weight as f64, max_weight as f64])
        .labels([
            min_weight.to_string(),
            (min_weight + (max_weight - min_weight) / 2.0).to_string(),
            max_weight.to_string(),
        ]);

    let line_dataset = Dataset::default()
        .data(&data)
        .marker(Marker::Braille)
        .style(Color::Blue)
        .graph_type(GraphType::Line);

    let chart = Chart::new(vec![line_dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(title)
                .title_alignment(Alignment::Center)
                .title_style(Style::new().bold().italic().blue()),
        );
    frame.render_widget(chart, area);
}

pub fn match_keys(keycode: KeyCode, app: &mut AppState) {
    match keycode {
        KeyCode::Char('d') => app.set_display(CurrentDisplay::Graph(GraphDisplay::Total)),
        KeyCode::Char('w') => app.set_display(CurrentDisplay::Graph(GraphDisplay::AverageWeekly)),
        KeyCode::Char('s') => app.set_display(CurrentDisplay::Graph(GraphDisplay::Week)),
        KeyCode::Char('a') => app.set_display(CurrentDisplay::Add(AddState::Calendar)),
        KeyCode::Char('t') => app.set_display(CurrentDisplay::Table),
        KeyCode::Char('h') | KeyCode::Left => {
            if app.get_display() == &CurrentDisplay::Graph(GraphDisplay::Week) {
                app.current_date = app
                    .current_date
                    .previous_day()
                    .expect("should not have issues going back one day")
                    .max(app.min_date)
            }
        }

        KeyCode::Char('l') | KeyCode::Right => {
            if app.get_display() == &CurrentDisplay::Graph(GraphDisplay::Week) {
                app.current_date = app
                    .current_date
                    .next_day()
                    .expect("should not have issues going forward one day")
                    .min(app.max_date)
            }
        }
        _ => {}
    };
}

pub fn render_chart_footer(frame: &mut Frame, area: Rect) {
    let hints: &[(&str, &str)] = {
        &[
            ("d", "Average Daily View"),
            ("w", "Average Weekly View"),
            ("s", "Seven Day View"),
            ("a", "add entry"),
            ("t", "table view"),
            ("q", "quit"),
        ]
    };
    render_footer(frame, area, hints);
}
