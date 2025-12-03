use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Widget},
};

pub struct MainWidget;

impl Widget for &MainWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // พื้นที่ซ้าย: 40%
                Constraint::Percentage(70), // พื้นที่ขวา: 60%
            ])
            .split(area);

        Block::bordered().title("Tasks").render(chunks[0], buf);
        Block::bordered().title("Detail").render(chunks[1], buf);
    }
}
