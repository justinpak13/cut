use ratatui::layout::HorizontalAlignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, TitlePosition};

const TITLE: &str = "CUT - Calorie Utilization Tracker";

// block styles
pub fn app_block() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_position(TitlePosition::Top)
        .title_alignment(HorizontalAlignment::Center)
        .title(TITLE)
}

pub fn widget_block() -> Block<'static> {
    Block::bordered().border_type(BorderType::Rounded)
}

pub fn input_block_valid() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Input:")
        .style(Style::default().fg(Color::Yellow))
}

pub fn input_block_invalid(error: &str) -> Block<'_> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Input:")
        .title_bottom(error)
        .style(Style::default().fg(Color::Red))
}
