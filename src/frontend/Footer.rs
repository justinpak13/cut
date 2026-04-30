use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Paragraph, Row, Table};

use crate::app::{AppState, CurrentDisplay};
use ratatui::layout::Layout;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use std::rc::Rc;

pub fn render_footer(frame: &mut Frame, area: Rect, hints: &[(&str, &str)]) {
    let dynamic_areas = get_layout_based_on_keybindings(hints.len(), area);

    create_dynamic_footer(frame, dynamic_areas, hints);
}

fn get_layout_based_on_keybindings(
    keybinding_count: usize,
    area: Rect,
) -> (Rc<[Rect]>, Rc<[Rect]>) {
    let [left_column, right_column] = area.layout(
        &Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]),
    );

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Length(1)].repeat(if keybinding_count.is_multiple_of(2) {
                keybinding_count / 2
            } else {
                (keybinding_count / 2) + 1
            }),
        )
        .split(left_column);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1)].repeat(keybinding_count / 2))
        .split(right_column);

    (left, right)
}

fn create_dynamic_footer(
    frame: &mut Frame,
    area: (Rc<[Rect]>, Rc<[Rect]>),
    hints: &[(&str, &str)],
) {
    let key_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let desc_style = Style::default().fg(Color::White);

    for (text_area, (key, desc)) in area.0.iter().chain(area.1.iter()).zip(hints.iter()) {
        let line = Line::from(vec![
            Span::styled(format!(" {key:<10}"), key_style), // left-pad key to align
            Span::styled(desc.to_string(), desc_style),
        ]);

        frame.render_widget(Paragraph::new(line), *text_area);
    }
}
