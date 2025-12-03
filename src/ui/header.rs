use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

pub struct HeaderWidget;

impl Widget for &HeaderWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(vec!["tasq-rs".into()])
            .left_aligned()
            .render(area, buf);
    }
}
