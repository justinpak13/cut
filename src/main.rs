#![warn(clippy::pedantic)]

mod app;
mod frontend;
mod weightlog;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::{io::Stdout, sync::mpsc::Receiver};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;

use crate::app::{AddState, AppState, CurrentDisplay};
use frontend::display_app;

fn main() -> Result<()> {
    let mut app = AppState::init();

    app.table_state
        .scroll_down_by(u16::try_from(app.get_data().len())?);
    app.table_state.select_first_column();

    let (tx, rx) = mpsc::channel::<Message>();
    let keystroke_tx = tx.clone();

    thread::spawn(move || listen_for_input(&keystroke_tx));

    ratatui::run(|terminal| {
        terminal.draw(|frame| display_app(frame, &mut app))?;

        main_thread(terminal, &mut app, &rx, &tx)?;
        Ok(())
    })
}

enum Message {
    Keystroke(KeyCode),
    Error(String),
    Save,
    Quit,
}

fn main_thread(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppState,
    rx: &Receiver<Message>,
    tx: &Sender<Message>,
) -> Result<()> {
    for message in rx {
        match message {
            Message::Keystroke(keycode) => {
                handle_input(keycode, app, tx);
                terminal.draw(|frame| display_app(frame, app))?;
            }
            Message::Error(error) => {
                eprintln!("{error}");
                break;
            }
            Message::Save => {
                let handle = thread::spawn(|| app.save());
            }
            Message::Quit => break,
        }
    }

    Ok(())
}

fn handle_input(input: KeyCode, app: &mut AppState, input_tx: &Sender<Message>) {
    match (input, app.get_display()) {
        // add could take any char as input so takes precedence
        (keycode, CurrentDisplay::Add(_)) => {
            frontend::add::match_keys(keycode, app);
        }
        // needs to be over the quit so q can go back to calendar instead of closing
        // entire app
        (keycode, CurrentDisplay::Delete) => {
            frontend::delete::match_keys(keycode, app);
        }

        // always on keycodes outside of add
        (KeyCode::Char('c'), _) => {
            app.set_display(CurrentDisplay::Graph(app::GraphDisplay::Total));
        }
        (KeyCode::Char('t'), _) => {
            app.set_display(CurrentDisplay::Table(app::TableDisplay::Total));
        }
        (KeyCode::Char('a'), _) => {
            app.set_display(CurrentDisplay::Add(AddState::Calendar));
        }
        (KeyCode::Char('q') | KeyCode::Esc, _) => {
            if app.edited {
                let _ = app.save();
            }
            input_tx
                .send(Message::Quit)
                .expect("channel should not be broken");
        }

        // based on display
        (keycode, CurrentDisplay::Table(_)) => {
            frontend::weight_table::match_keys(keycode, app);
        }
        (keycode, CurrentDisplay::Graph(_)) => {
            frontend::weight_chart::match_keys(keycode, app);
        }
    }
}

fn listen_for_input(tx: &Sender<Message>) -> Result<()> {
    loop {
        if let Some(keystroke) = event::read()?.as_key_press_event() {
            if tx.send(Message::Keystroke(keystroke.code)).is_err() {
                break;
            }
        }
    }

    Ok(())
}
