use chrono::{Datelike, Local};
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
use time::OffsetDateTime;

use crate::app::{AppState, CurrentDisplay};

use crate::frontend::CustomStyle;
use crate::frontend::Footer::render_footer;
/// Render a chart going upward.
pub fn render_delete_confirmation(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let popup_block = Block::bordered().title("Confirm Delete");
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    // clears out any background in the area before rendering the popup
    frame.render_widget(Clear, centered_area);
    if let Some(index) = app_state.table_state.selected() {
        let log = &app_state.get_data()[index];
        let paragraph = Paragraph::new(format!(
            "Date: {}\nWeight: {}",
            log.get_date(),
            log.get_weight()
        ))
        .block(popup_block);
        frame.render_widget(paragraph, centered_area);
    } else {
        frame.render_widget(
            Paragraph::new("Log was not selected").block(popup_block),
            centered_area,
        );
    }
}

pub fn render_delete_footer(frame: &mut Frame, area: Rect) {
    let hints: &[(&str, &str)] = { &[("y", "Confirm"), ("Any other Key", "Cancel")] };
    render_footer(frame, area, hints);
}

pub fn match_keys(keycode: KeyCode, app: &mut AppState) {
    match keycode {
        KeyCode::Char('y') | KeyCode::Char('Y') => delete_log(app),
        _ => {}
    };

    app.set_display(CurrentDisplay::Table);
}

fn delete_log(app: &mut AppState) {
    if let Some(index) = app.table_state.selected() {
        app.delete_log(index);
    }
}
