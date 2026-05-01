use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};

use crate::app::{AddState, AppState, CurrentDisplay, Input};
use crate::frontend::custom_style;

fn calendar_rect(area: Rect) -> Rect {
    let cal_width = 24;
    let cal_height = 8;

    let x = area.x + (area.width.saturating_sub(cal_width)) / 2;
    let y = area.y + (area.height.saturating_sub(cal_height)) / 2;

    Rect::new(x, y, cal_width.min(area.width), cal_height.min(area.height))
}

pub fn render_current_month(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let calendar_area = calendar_rect(area);

    let mut event_store = CalendarEventStore::today(Style::default().red().bold());
    event_store.add(app.current_date, Style::default().blue().italic());

    let this_month = Monthly::new(app.current_date, event_store)
        .show_surrounding(Modifier::DIM)
        .show_month_header(Modifier::BOLD)
        .show_weekdays_header(Style::default().bold().green())
        .default_style(Style::default().bold().bg(Color::Rgb(50, 50, 50)));
    frame.render_widget(this_month, calendar_area);
}

pub fn render_weight_input(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]);
    let [help_area, input_area] = area.layout(&layout);
    let help_message = Paragraph::new(Text::from("Input Weight"));
    frame.render_widget(help_message, help_area);

    let input = match app.get_display() {
        CurrentDisplay::Add(AddState::WeightInput(Input::Invalid(error_string))) => {
            Paragraph::new(app.char_buf.as_str())
                .block(custom_style::input_block_invalid(error_string.as_str()))
        }
        _ => Paragraph::new(app.char_buf.as_str()).block(custom_style::input_block_valid()),
    };
    frame.render_widget(input, input_area);
    #[expect(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position can be controlled via the left and right arrow key
        input_area.x + app.character_index as u16 + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

pub fn render_note_input(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]);
    let [help_area, input_area] = area.layout(&layout);
    let help_message = Paragraph::new(Text::from("Input Note (Optional)"));
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.char_buf.as_str()).block(custom_style::input_block_valid());

    frame.render_widget(input, input_area);
    #[expect(clippy::cast_possible_truncation)]
    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position can be controlled via the left and right arrow key
        input_area.x + app.character_index as u16 + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

pub fn match_keys(keycode: KeyCode, app: &mut AppState) {
    match app.display {
        CurrentDisplay::Add(AddState::Calendar) => match keycode {
            KeyCode::Char('j') | KeyCode::Down => {
                app.current_date = app.current_date.next_occurrence(app.current_date.weekday())
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.current_date = app.current_date.prev_occurrence(app.current_date.weekday())
            }
            KeyCode::Char('l') | KeyCode::Right => {
                app.current_date = app
                    .current_date
                    .next_day()
                    .expect("should not have an issue with next day")
            }
            KeyCode::Char('h') | KeyCode::Left => {
                app.current_date = app
                    .current_date
                    .previous_day()
                    .expect("should not have an issue with next day")
            }
            KeyCode::Enter => {
                app.set_display(CurrentDisplay::Add(AddState::WeightInput(Input::Valid)));
            }
            KeyCode::Char('q') | KeyCode::Esc => app.set_display(CurrentDisplay::Table),
            _ => {}
        },
        CurrentDisplay::Add(AddState::WeightInput(_)) => match keycode {
            KeyCode::Char('0') => {
                if app.character_index > 0 {
                    app.enter_char('0');
                }
            }
            KeyCode::Char('1') => {
                app.enter_char('1');
            }
            KeyCode::Char('2') => {
                app.enter_char('2');
            }
            KeyCode::Char('3') => {
                app.enter_char('3');
            }
            KeyCode::Char('4') => {
                app.enter_char('4');
            }
            KeyCode::Char('5') => {
                app.enter_char('5');
            }
            KeyCode::Char('6') => {
                app.enter_char('6');
            }
            KeyCode::Char('7') => {
                app.enter_char('7');
            }
            KeyCode::Char('8') => {
                app.enter_char('8');
            }
            KeyCode::Char('9') => {
                app.enter_char('9');
            }
            KeyCode::Char('.') => {
                if !app.char_buf.contains(".") {
                    app.enter_char('.');
                }
            }
            KeyCode::Backspace => {
                app.delete_char();
            }
            KeyCode::Enter => {
                if let Ok(value) = app.char_buf.parse::<f32>() {
                    app.current_weight = value; 
                    app.char_buf.clear();
                    app.set_display(CurrentDisplay::Add(AddState::NoteInput));
                    app.reset_cursor();
                } else {
                    app.char_buf.clear();
                    app.reset_cursor();
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                app.set_display(CurrentDisplay::Add(AddState::Calendar));
            }
            _ => {}
        },

        CurrentDisplay::Add(AddState::NoteInput) => match keycode {
            KeyCode::Char(c) => {
                if c != ',' {
                    app.enter_char(c);
                }
            }
            KeyCode::Backspace => {
                app.delete_char();
            }

            KeyCode::Esc => {
                app.set_display(CurrentDisplay::Add(AddState::WeightInput(Input::Valid)));
            }
            KeyCode::Enter => {
                app.submit();
                app.reset_cursor();
                app.set_display(CurrentDisplay::Add(AddState::Calendar));
            }
            _ => {}
        },

        _ => unreachable!(),
    };
}
