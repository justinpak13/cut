mod app;
mod frontend;
mod weightlog;

use chrono::Local;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use crossterm::style;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::{Marker, block};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{
    Axis, Block, Chart, Clear, Dataset, GraphType, Padding, Paragraph, Row, Table, TableState,
};
use ratatui::{Frame, layout};
use time::OffsetDateTime;

use crate::app::{AddState, AppState, CurrentDisplay};
use crate::weightlog::WeightLog;
use frontend::display_app;

fn main() -> Result<()> {
    let mut app = AppState::init();

    app.table_state.scroll_down_by(app.get_data().len() as u16);
    app.table_state.select_first_column();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| display_app(frame, &mut app))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match (key.code, app.get_display()) {
                    // always on keycodes
                    (KeyCode::Char('c'), _) => {
                        app.set_display(CurrentDisplay::Graph(app::GraphDisplay::Total))
                    }
                    (KeyCode::Char('t'), _) => app.set_display(CurrentDisplay::Table),
                    (KeyCode::Char('a'), _) => {
                        app.set_display(CurrentDisplay::Add(AddState::Calendar))
                    }

                    (keycode, CurrentDisplay::Add(add_state)) => {
                        frontend::Add::match_keys(keycode, &mut app)
                    }
                    (KeyCode::Char('q') | KeyCode::Esc, _) => {
                        let _ = app.save();
                        return Ok(());
                    }

                    // based on display
                    (keycode, CurrentDisplay::Table) => {
                        frontend::WeightTable::match_keys(keycode, &mut app)
                    }
                    (keycode, CurrentDisplay::Graph(graph_display)) => {
                        frontend::WeightChart::match_keys(keycode, &mut app)
                    }

                    (keycode, CurrentDisplay::Delete) => {
                        frontend::Delete::match_keys(keycode, &mut app)
                    }
                    _ => {}
                }
            }
        }
    })
}
