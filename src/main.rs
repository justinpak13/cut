#![warn(clippy::pedantic)]

mod app;
mod frontend;
mod weightlog;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};

use crate::app::{AddState, AppState, CurrentDisplay};
use frontend::display_app;

fn main() -> Result<()> {
    let mut app = AppState::init();

    app.table_state
        .scroll_down_by(u16::try_from(app.get_data().len())?);
    app.table_state.select_first_column();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| display_app(frame, &mut app))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match (key.code, app.get_display()) {
                    // always on keycodes
                    (KeyCode::Char('c'), _) => {
                        app.set_display(CurrentDisplay::Graph(app::GraphDisplay::Total));
                    }
                    (KeyCode::Char('t'), _) => app.set_display(CurrentDisplay::Table),
                    (KeyCode::Char('a'), _) => {
                        app.set_display(CurrentDisplay::Add(AddState::Calendar));
                    }

                    (keycode, CurrentDisplay::Add(_)) => {
                        frontend::add::match_keys(keycode, &mut app);
                    }
                    (KeyCode::Char('q') | KeyCode::Esc, _) => {
                        let _ = app.save();
                        return Ok(());
                    }

                    // based on display
                    (keycode, CurrentDisplay::Table) => {
                        frontend::weight_table::match_keys(keycode, &mut app);
                    }
                    (keycode, CurrentDisplay::Graph(_)) => {
                        frontend::weight_chart::match_keys(keycode, &mut app);
                    }

                    (keycode, CurrentDisplay::Delete) => {
                        frontend::delete::match_keys(keycode, &mut app);
                    }
                    _ => {}
                }
            }
        }
    })
}
