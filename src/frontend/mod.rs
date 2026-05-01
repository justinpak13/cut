pub mod add;
pub mod custom_style;
pub mod delete;
pub mod footer;
pub mod weight_chart;
pub mod weight_table;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear};
use ratatui::{Frame, layout};

use crate::app::{AddState, AppState, CurrentDisplay};

pub fn display_app(frame: &mut Frame, app: &mut AppState) {
    let layout = Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([Constraint::Fill(100)]);
    let [outer_boder] = frame.area().layout(&layout);
    let inner = outer_boder.centered(Constraint::Percentage(95), Constraint::Percentage(95));

    let [top, main, footer] = inner.layout(
        &Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(6),
        ])
        .spacing(1),
    );

    let keybindings = footer.centered(Constraint::Percentage(75), Constraint::Percentage(75));

    frame.render_widget(Clear, outer_boder);
    frame.render_widget(custom_style::app_block(), outer_boder);

    frame.render_widget(Clear, main);
    frame.render_widget(Clear, footer);
    frame.render_widget(Clear, top);

    render_top(frame, top);
    frame.render_widget(custom_style::widget_block(), footer);

    match app.get_display() {
        CurrentDisplay::Table => {
            weight_table::render_table(frame, main, app);
            weight_table::render_table_footer(frame, keybindings);
        }
        CurrentDisplay::Graph(_) => {
            weight_chart::render_chart(frame, main, app);
            weight_chart::render_chart_footer(frame, keybindings);
        }
        CurrentDisplay::Edit => {}
        CurrentDisplay::Add(add_state) => {
            let popup_block = Block::bordered().title("Select Date");
            let centered_area =
                main.centered(Constraint::Percentage(50), Constraint::Percentage(50));
            frame.render_widget(Clear, centered_area);
            frame.render_widget(popup_block, centered_area);
            match add_state {
                AddState::Calendar => {
                    add::render_current_month(frame, centered_area, app);
                }
                AddState::WeightInput(_) => {
                    add::render_weight_input(frame, centered_area, app);
                }
                AddState::NoteInput => {
                    add::render_note_input(frame, centered_area, app);
                }
            }
        }
        CurrentDisplay::Delete => {
            delete::render_delete_confirmation(frame, main, app);
            delete::render_delete_footer(frame, keybindings);
        }
    }
}

fn render_top(frame: &mut Frame, area: Rect) {
    let title = Line::from_iter([Span::from("Weight information").bold()]);
    frame.render_widget(title.centered(), area);
}
