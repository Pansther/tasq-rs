use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget},
};

pub struct FooterWidget;

impl Widget for &FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        Paragraph::new(instructions)
            .left_aligned()
            .render(area, buf);
    }
}
