use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::app::{AppState, CurrentDisplay, TableDisplay};

use crate::frontend::footer::render_footer;
/// Render a chart going upward.
pub fn render_delete_confirmation(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let popup_block = Block::bordered().title("Confirm Delete");
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    // clears out any background in the area before rendering the popup
    frame.render_widget(Clear, centered_area);
    if let Some(index) = app_state.table_state.selected() {
        let log = &app_state.get_data()[index];
        let paragraph = Paragraph::new(format!(
            "Date: {}\nWeight: {}\nNote: {}",
            log.get_date(),
            log.get_weight(),
            log.get_note().unwrap_or_default()
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

    app.set_display(CurrentDisplay::Table(TableDisplay::Total));
}

fn delete_log(app: &mut AppState) {
    if let Some(index) = app.table_state.selected() {
        app.delete_log(index);
    }
}
